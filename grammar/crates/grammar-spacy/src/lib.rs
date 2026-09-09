//! Optional spaCy adapter for the parser-independent grammar core.
//!
//! Python emits annotations of the exact source, with Unicode character offsets.
//! This adapter verifies source identity, converts offsets to UTF-8 bytes, and
//! validates all spans and dependency trees before returning the core IR.

use anyhow::{Context, Result, ensure};
use grammar_core::syntax::{Document, Sentence, Token, validate};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::io::Write;
use std::process::{Command, Stdio};

pub const ANNOTATION_VERSION: &str = "grammar-spacy-annotation-v1";

#[derive(Deserialize)]
struct WireDocument {
    text: String,
    parser_identity: String,
    tokens: Vec<WireToken>,
    sentences: Vec<WireSentence>,
}

#[derive(Deserialize)]
struct WireToken {
    i: usize,
    start_char: usize,
    end_char: usize,
    text: String,
    lemma: String,
    pos: String,
    tag: String,
    dep: String,
    head: usize,
    sentence: usize,
    morph: BTreeMap<String, Vec<String>>,
    is_punct: bool,
    is_space: bool,
}

#[derive(Deserialize)]
struct WireSentence {
    start_char: usize,
    end_char: usize,
    root: usize,
    token_start: usize,
    token_end: usize,
}

#[derive(Deserialize)]
struct WireBatchRecord {
    index: usize,
    #[serde(flatten)]
    result: WireBatchResult,
}

#[derive(Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum WireBatchResult {
    Ok { annotation: serde_json::Value },
    Error { text: String, error: String },
}

/// Parse the original UTF-8 text without normalization or quote substitutions.
/// The default model is the installed `en_core_web_sm` English dependency parser.
pub fn parse(text: &str, python: &str) -> Result<Document> {
    let bridge = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("python/syntax_bridge.py");
    let mut child = Command::new(python)
        .arg(bridge)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Cannot start syntax interpreter {python:?}"))?;
    let input_result = child
        .stdin
        .take()
        .context("Syntax bridge stdin unavailable")?
        .write_all(text.as_bytes());
    let output = child
        .wait_with_output()
        .context("Cannot wait for syntax bridge")?;
    ensure!(
        output.status.success(),
        "Syntax annotation failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    );
    input_result.context("Cannot write text to syntax bridge")?;
    let wire = serde_json::from_slice(&output.stdout)
        .context("Syntax bridge returned invalid annotation JSON")?;
    from_wire(text, wire)
}

/// Parse a caller-bounded batch, loading the installed model once.
///
/// The returned records preserve input order and use exactly the same annotation
/// semantics and identity as [`parse`]. Each document is independently validated;
/// a failed annotation or validation is retained as an error at its input index.
/// Interpreter, transport, response count and ordering failures reject the batch.
/// Empty batches return immediately without starting an interpreter.
pub fn parse_batch(
    texts: &[&str],
    python: &str,
) -> Result<Vec<std::result::Result<Document, String>>> {
    if texts.is_empty() {
        return Ok(Vec::new());
    }
    let input = serde_json::to_vec(texts).context("Cannot encode syntax batch input")?;
    let bridge = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("python/syntax_bridge.py");
    let mut child = Command::new(python)
        .arg(bridge)
        .arg("--batch")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Cannot start syntax interpreter {python:?}"))?;
    // The bridge consumes the complete JSON input before loading the model or
    // writing records, so this write cannot block on undrained annotation output.
    let input_result = child
        .stdin
        .take()
        .context("Syntax bridge stdin unavailable")?
        .write_all(&input);
    let output = child
        .wait_with_output()
        .context("Cannot wait for syntax batch bridge")?;
    ensure!(
        output.status.success(),
        "Syntax batch annotation failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    );
    input_result.context("Cannot write texts to syntax batch bridge")?;
    let records = serde_json::from_slice(&output.stdout)
        .context("Syntax bridge returned invalid batch JSON")?;
    from_batch_wire(texts, records)
}

fn from_batch_wire(
    texts: &[&str],
    records: Vec<WireBatchRecord>,
) -> Result<Vec<std::result::Result<Document, String>>> {
    ensure!(
        records.len() == texts.len(),
        "Syntax batch returned {} records for {} inputs",
        records.len(),
        texts.len()
    );
    ensure!(
        records
            .iter()
            .enumerate()
            .all(|(index, record)| record.index == index),
        "Syntax batch returned records out of input order"
    );
    Ok(texts
        .iter()
        .zip(records)
        .map(|(text, record)| {
            let result: Result<Document> = match record.result {
                WireBatchResult::Ok { annotation } => serde_json::from_value(annotation)
                    .context("Syntax batch returned invalid document JSON")
                    .and_then(|wire| from_wire(text, wire)),
                WireBatchResult::Error {
                    text: returned_text,
                    error,
                } => {
                    if returned_text != *text {
                        Err(anyhow::anyhow!(
                            "Syntax bridge changed the exact source text"
                        ))
                    } else {
                        Err(anyhow::anyhow!("Syntax annotation failed: {error}"))
                    }
                }
            };
            result.map_err(|error| format!("{error:#}"))
        })
        .collect())
}

fn from_wire(text: &str, wire: WireDocument) -> Result<Document> {
    ensure!(
        wire.text == text,
        "Syntax bridge changed the exact source text"
    );
    ensure!(
        wire.parser_identity
            .starts_with(&format!("{ANNOTATION_VERSION};"))
            && wire.parser_identity.ends_with(";raw-text-v1"),
        "Syntax bridge returned an unsupported annotation identity"
    );
    // Python string offsets count Unicode scalars, while Rust edits use bytes.
    // The final boundary permits exclusive spans ending at text.len().
    let boundaries: Vec<usize> = text
        .char_indices()
        .map(|(i, _)| i)
        .chain([text.len()])
        .collect();
    let byte = |character: usize| -> Result<usize> {
        boundaries.get(character).copied().with_context(|| {
            format!("Syntax character offset {character} exceeds the source length")
        })
    };
    let tokens = wire
        .tokens
        .into_iter()
        .map(|token| {
            Ok(Token {
                i: token.i,
                start_byte: byte(token.start_char)?,
                end_byte: byte(token.end_char)?,
                text: token.text,
                lemma: token.lemma,
                pos: token.pos,
                tag: token.tag,
                dep: token.dep,
                head: token.head,
                sentence: token.sentence,
                morph: token.morph,
                is_punct: token.is_punct,
                is_space: token.is_space,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let sentences = wire
        .sentences
        .into_iter()
        .map(|sentence| {
            Ok(Sentence {
                start_byte: byte(sentence.start_char)?,
                end_byte: byte(sentence.end_char)?,
                root: sentence.root,
                token_start: sentence.token_start,
                token_end: sentence.token_end,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let doc = Document {
        text: wire.text,
        parser_identity: wire.parser_identity,
        tokens,
        sentences,
    };
    validate(&doc)?;
    Ok(doc)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn installed_python() -> Option<std::path::PathBuf> {
        let crate_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        std::env::var_os("GRAMMAR_SPACY_PYTHON")
            .map(std::path::PathBuf::from)
            .or_else(|| {
                ["../../.venv/bin/python", "../../../.venv/bin/python"]
                    .iter()
                    .map(|path| crate_dir.join(path))
                    .find(|path| path.exists())
            })
    }

    fn empty_batch_record(index: usize) -> WireBatchRecord {
        WireBatchRecord {
            index,
            result: WireBatchResult::Ok {
                annotation: serde_json::json!({
                    "text": "",
                    "parser_identity": format!("{ANNOTATION_VERSION};synthetic-fixture;raw-text-v1"),
                    "tokens": [],
                    "sentences": [],
                }),
            },
        }
    }

    fn wire(text: &str, items: &[(&str, &str, &str, usize)]) -> WireDocument {
        let mut cursor = 0;
        let mut tokens = Vec::new();
        for (i, &(word, pos, dep, head)) in items.iter().enumerate() {
            let start_byte = text[cursor..].find(word).unwrap() + cursor;
            let end_byte = start_byte + word.len();
            tokens.push(WireToken {
                i,
                start_char: text[..start_byte].chars().count(),
                end_char: text[..end_byte].chars().count(),
                text: word.into(),
                lemma: word.to_lowercase(),
                pos: pos.into(),
                tag: pos.into(),
                dep: dep.into(),
                head,
                sentence: 0,
                morph: BTreeMap::new(),
                is_punct: pos == "PUNCT",
                is_space: pos == "SPACE",
            });
            cursor = end_byte;
        }
        let sentences = if tokens.is_empty() {
            Vec::new()
        } else {
            vec![WireSentence {
                start_char: tokens[0].start_char,
                end_char: tokens.last().unwrap().end_char,
                root: items
                    .iter()
                    .enumerate()
                    .find(|(i, item)| *i == item.3)
                    .unwrap()
                    .0,
                token_start: 0,
                token_end: tokens.len(),
            }]
        };
        WireDocument {
            text: text.into(),
            parser_identity: format!("{ANNOTATION_VERSION};synthetic-fixture;raw-text-v1"),
            tokens,
            sentences,
        }
    }

    #[test]
    fn character_offsets_convert_to_exact_utf8_spans_without_normalization() {
        let text = "cafe\u{301} isn’t 🦊 ordinary.";
        let items = [
            ("cafe\u{301}", "NOUN", "nsubj", 1),
            ("is", "AUX", "ROOT", 1),
            ("n’t", "PART", "neg", 1),
            ("🦊", "PUNCT", "punct", 1),
            ("ordinary", "ADJ", "acomp", 1),
            (".", "PUNCT", "punct", 1),
        ];
        let doc = from_wire(text, wire(text, &items)).unwrap();
        assert_eq!(doc.text.as_bytes(), text.as_bytes());
        assert_eq!(doc.tokens[0].end_byte, 6);
        assert_eq!(doc.tokens[2].text, "n’t");
        assert_eq!(doc.tokens[3].end_byte - doc.tokens[3].start_byte, 4);
        for token in &doc.tokens {
            assert_eq!(&text[token.start_byte..token.end_byte], token.text);
        }
        let mut corrupt = wire(text, &items);
        corrupt.tokens[2].end_char = text.chars().count() + 1;
        assert!(from_wire(text, corrupt).is_err());
        let mut normalized = wire(text, &items);
        normalized.text = "café isn’t 🦊 ordinary.".into();
        assert!(
            from_wire(text, normalized)
                .unwrap_err()
                .to_string()
                .contains("exact source")
        );
    }

    #[test]
    fn incompatible_annotation_identity_and_broken_trees_are_rejected() {
        let text = "Birds fly.";
        let items = [
            ("Birds", "NOUN", "nsubj", 1),
            ("fly", "VERB", "ROOT", 1),
            (".", "PUNCT", "punct", 1),
        ];
        let mut wrong_identity = wire(text, &items);
        wrong_identity.parser_identity = "normalized-parser-v1".into();
        assert!(
            from_wire(text, wrong_identity)
                .unwrap_err()
                .to_string()
                .contains("unsupported annotation identity")
        );
        let mut wrong_head = wire(text, &items);
        wrong_head.tokens[0].head = 3;
        assert!(
            from_wire(text, wrong_head)
                .unwrap_err()
                .to_string()
                .contains("out of bounds")
        );
        let mut wrong_root = wire(text, &items);
        wrong_root.sentences[0].root = 0;
        assert!(from_wire(text, wrong_root).is_err());
    }

    #[test]
    fn missing_interpreter_is_an_explicit_error() {
        assert!(
            parse("Some text.", "/grammar-spacy-nonexistent-python")
                .unwrap_err()
                .to_string()
                .contains("Cannot start syntax interpreter")
        );
        assert!(
            parse_batch(&["Some text."], "/grammar-spacy-nonexistent-python")
                .unwrap_err()
                .to_string()
                .contains("Cannot start syntax interpreter")
        );
        assert!(
            parse_batch(&[], "/grammar-spacy-nonexistent-python")
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn batch_requires_one_record_per_input_in_exact_order() {
        assert!(
            from_batch_wire(&["", ""], vec![empty_batch_record(0)])
                .unwrap_err()
                .to_string()
                .contains("1 records for 2 inputs")
        );
        assert!(
            from_batch_wire(
                &["", ""],
                vec![empty_batch_record(1), empty_batch_record(0)]
            )
            .unwrap_err()
            .to_string()
            .contains("out of input order")
        );
        assert!(
            from_batch_wire(
                &["", ""],
                vec![empty_batch_record(0), empty_batch_record(0)]
            )
            .is_err()
        );
    }

    #[test]
    fn batch_retains_each_annotation_and_validation_failure() {
        let mut wrong_identity = empty_batch_record(1);
        let WireBatchResult::Ok { annotation } = &mut wrong_identity.result else {
            panic!()
        };
        annotation["parser_identity"] = serde_json::json!("incompatible");
        let records = vec![
            empty_batch_record(0),
            wrong_identity,
            WireBatchRecord {
                index: 2,
                result: WireBatchResult::Ok {
                    annotation: serde_json::Value::Null,
                },
            },
            WireBatchRecord {
                index: 3,
                result: WireBatchResult::Error {
                    text: "".into(),
                    error: "synthetic parser failure".into(),
                },
            },
            WireBatchRecord {
                index: 4,
                result: WireBatchResult::Error {
                    text: "changed source".into(),
                    error: "synthetic parser failure".into(),
                },
            },
            empty_batch_record(5),
        ];
        let actual = from_batch_wire(&["different source", "", "", "", "", ""], records).unwrap();
        assert!(
            actual[0]
                .as_ref()
                .unwrap_err()
                .contains("exact source text")
        );
        assert!(
            actual[1]
                .as_ref()
                .unwrap_err()
                .contains("unsupported annotation identity")
        );
        assert!(
            actual[2]
                .as_ref()
                .unwrap_err()
                .contains("invalid document JSON")
        );
        assert!(
            actual[3]
                .as_ref()
                .unwrap_err()
                .contains("synthetic parser failure")
        );
        assert!(
            actual[4]
                .as_ref()
                .unwrap_err()
                .contains("exact source text")
        );
        assert_eq!(actual[5].as_ref().unwrap().text, "");
    }

    #[test]
    fn installed_batch_matches_individual_parses_and_continues_after_parser_error() {
        let Some(python) = installed_python() else {
            eprintln!(
                "spaCy integration skipped: set GRAMMAR_SPACY_PYTHON or install a local .venv"
            );
            return;
        };
        let python = python.to_str().unwrap();
        let texts = [
            "\n\ncafe\u{301} isn’t empty. 🦊 We saw birds that flew home.\n\n",
            "We can't stay; they don't know.\tCan you help?",
            "",
            "   ",
        ];
        let actual = parse_batch(&texts, python).unwrap();
        for (text, actual) in texts.iter().zip(actual) {
            assert_eq!(actual.unwrap(), parse(text, python).unwrap());
        }
        // spaCy rejects inputs above its configured maximum before tokenization.
        // An error in one input must not discard the valid document after it.
        let too_long = "x".repeat(1_000_001);
        let actual = parse_batch(&[&too_long, texts[1]], python).unwrap();
        assert!(actual[0].as_ref().unwrap_err().contains("E088"));
        assert_eq!(actual[1].as_ref().unwrap().text, texts[1]);
    }

    #[test]
    fn installed_bridge_rejects_malformed_batch_input() {
        let Some(python) = installed_python() else {
            eprintln!(
                "spaCy integration skipped: set GRAMMAR_SPACY_PYTHON or install a local .venv"
            );
            return;
        };
        let bridge =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("python/syntax_bridge.py");
        for input in ["{}", "[\"text\", 42]"] {
            let mut child = Command::new(&python)
                .arg(&bridge)
                .arg("--batch")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            child
                .stdin
                .take()
                .unwrap()
                .write_all(input.as_bytes())
                .unwrap();
            let output = child.wait_with_output().unwrap();
            assert!(!output.status.success());
            assert!(output.stdout.is_empty());
            assert!(
                String::from_utf8_lossy(&output.stderr)
                    .contains("Batch input must be a JSON array of strings")
            );
        }
    }

    #[test]
    fn installed_parser_preserves_raw_text_and_produces_valid_typed_features() {
        let Some(python) = installed_python() else {
            eprintln!(
                "spaCy integration skipped: set GRAMMAR_SPACY_PYTHON or install a local .venv"
            );
            return;
        };
        let text = "\n\ncafe\u{301} isn’t empty. 🦊 We saw birds that flew home.\n\n";
        let doc = parse(text, python.to_str().unwrap()).unwrap();
        assert_eq!(doc.text, text);
        assert!(doc.parser_identity.starts_with(ANNOTATION_VERSION));
        assert!(doc.parser_identity.contains(";spacy="));
        assert!(!doc.parser_identity.contains("grammar-rules-v1"));
        assert!(doc.tokens.iter().any(|token| token.text == "cafe\u{301}"));
        assert!(doc.tokens.iter().any(|token| token.text == "n’t"));
        assert!(
            doc.tokens
                .iter()
                .any(|token| token.text == "🦊" && token.end_byte - token.start_byte == 4)
        );
        let result = grammar_core::features::extract(&doc).unwrap();
        let grammar_core::features::Family::Distribution { opportunities, .. } =
            &result.families["clause_head_frame"]
        else {
            panic!()
        };
        assert!(*opportunities >= 3);
        let grammar_core::features::Family::Metrics { opportunities, .. } =
            &result.families["syntax_sentence_load"]
        else {
            panic!()
        };
        assert_eq!(*opportunities, 2);
        for text in ["", "   "] {
            let doc = parse(text, python.to_str().unwrap()).unwrap();
            assert_eq!(doc.text, text);
            let features = grammar_core::features::extract(&doc).unwrap();
            let grammar_core::features::Family::Distribution { opportunities, .. } =
                &features.families["word"]
            else {
                panic!()
            };
            assert_eq!(*opportunities, 0);
        }
    }
}
