# Identical-text parser controls

This study checks whether repeated annotation of identical full texts changes
the evidence used by Slop Ninja's dative comparisons. The preceding
[argument-policy study](dative-arguments.md) compared source posts with edited
candidates. Those inputs differ in their bytes and linguistic context. The
present control holds each text fixed while varying repetition, interpreter
restart and processing order. All 1,638 controlled repeat comparisons were
identical. Every available historical annotation also matched.

The input set contains every source and candidate associated with the primary
transformer's 60 proposals from that completed study: 57 source posts and
60 candidate texts, all 117 unique. Selection includes proposals regardless
of their comparison outcome. Both `incumbent_sm` and `isolated_trf` receive
this same set, including texts for which the small parser has no saved
candidate annotation. The selection is conditional on the preceding primary
proposals; it does not represent a random sample of corpus writing.

Each parser runs in three separate interpreters. Within each process, every
text appears twice consecutively. Texts A, B and C below stand for increasing
SHA256 order, with the full 117-text sequence retained.

| Process | Request order | Requests |
| --- | --- | ---: |
| `canonical_a` | A, A, B, B, C, C, ... | 234 |
| `canonical_b` | The exact same sequence in a fresh interpreter | 234 |
| `reversed_c` | Descending SHA256 order, still two adjacent copies per text | 234 |

Each process is one call to the unchanged `grammar_spacy::parse_batch`.
The calls run sequentially. Repeated occurrences retain their process,
request index and repeat position; they are neither deduplicated nor served
from an annotation cache. There are 702 annotation requests per parser and
1,404 across both parsers. A process loads its model once, then the bridge
calls `nlp(text)` separately for each array entry. It does not concatenate
documents or use `nlp.pipe`. The bridge retains records until the batch ends.

The requested comparisons are fixed before annotation:

| Comparison | Per parser | Both parsers |
| --- | ---: | ---: |
| Adjacent repeat 0 versus repeat 1, in all three processes | 351 | 702 |
| `canonical_a` versus `canonical_b`, at both repeat positions | 234 | 468 |
| `canonical_a` versus `reversed_c`, at both repeat positions | 234 | 468 |
| Saved historical reference versus `canonical_a` repeat 0 | 117 | 234 |
| Total requested comparisons | 936 | 1,872 |

Historical comparisons remain separate from the controlled repeats. Missing
historical references still receive requested rows, with unavailable status.
Counts reuse the same texts and annotations across contrasts; the 1,872 rows
are not independent observations.

The primary measure is exact equality of validated, typed
`grammar_core::syntax::Document` values, serialized in a fixed field order.
Whitespace or key ordering in an outer saved JSON wrapper does not affect
this measure. Each comparison requires identical text bytes and parser
identity. Token occurrences align by exact UTF-8 byte span and text, including
repeated words. Changed tokenizations remain unmatched. The diagnostic retains
POS, tag, raw and normalized lemma, morphology, dependency and sentence
differences. Dependency heads are compared by their anchored occurrences;
numeric token/head index changes are also reported separately. Sentence index
shifts are distinguished from changed sentence spans and roots.

When Documents differ, the unchanged original 14 feature families are
extracted and compared. Exact Document equality implies equal deterministic
features and is labeled as an inference, without another extraction. Token,
field, attachment, sentence and feature counts describe overlapping effects;
they must not be added as independent failures. These feature comparisons
involve no author scoring, fitting or parameter selection.

The frozen parser configurations use spaCy 3.8.16 with English model 3.8.0.
Their bound wrappers set OMP, MKL and OpenBLAS thread environment variables to
8, `PYTHONHASHSEED=0`, tokenizer parallelism off and CUDA visibility empty.
The existing CPU configuration is retained. The bridge loads each model with
NER excluded and adds no random-seed or deterministic-kernel override.

The transformer's pinned configuration uses a curated RoBERTa model with
ByteBPE encoding: windows of 144 encoded pieces, stride 104, and at most
384 spans per transformer sub-batch. Overlapping representations are averaged.
These windows belong to individual documents. An edit can change the encoded
sequence and its window context even if identical-text repetitions agree.
The local inference path calls the PyTorch model in evaluation mode under
`no_grad`; configured training dropout does not establish inference-time
randomness. The configuration's training seed is distinct from an explicit
seed-setting call in the annotation bridge.

Every per-document error is retained at its scheduled occurrence. An
interpreter or transport failure makes all 234 requests in that process
unavailable, with the process error retained. There are no replacement parses.
Comparisons report requested, compared, identical, different and unavailable
counts separately. A failed or missing side never counts as an observed
annotation difference or equality. Source, parser or artifact binding failures
stop the run and leave a failure record.

The protocol binds the predecessor reports, selection records, exact text
digests, historical annotation references, parser assets, source files and
executable. The new
[`slopninja-parser-repeat`](../grammar/crates/grammar-eval/src/bin/slopninja-parser-repeat.rs)
CLI provides `freeze` and `run` commands; `run` requires the expected protocol
SHA256. Protocols, corpus-derived text and annotations remain under ignored
`data/author-corpora/blog-authorship-2004/parser-repeat-v1/`. The
[runner](../grammar/crates/grammar-eval/src/parser_repeat_study.rs) and
[comparison module](../grammar/crates/grammar-eval/src/parser_repeat_observation.rs)
retain the complete request and comparison records.

Adjacent pairs measure observed within-process repeat variation. The two
canonical processes measure restart-associated variation under the same
schedule. The reversed process provides an order-associated contrast across
fresh interpreters; it cannot isolate order causation from a restart effect.
Agreement throughout would establish observed stability on these selected
texts and settings, without establishing general parser determinism.
Historical differences also have an earlier execution context. None of these
comparisons establishes that an edit caused a parser change, certifies meaning
or readability, or grants an automatic edit license.

## Completed results

Both parsers completed all 702 scheduled annotations, with no document or
process errors. The same 117 texts were used for both parsers.

| Comparison | Small: identical / compared | Transformer: identical / compared |
| --- | ---: | ---: |
| Adjacent repeats, all three processes | 351/351 | 351/351 |
| Same schedule in a fresh interpreter | 234/234 | 234/234 |
| Reversed schedule in a fresh interpreter | 234/234 | 234/234 |
| Historical references | 104/104 | 117/117 |

Thirteen small-parser historical candidate references were unavailable. All
57 source references and 47 candidate references were available for that
parser; all 117 were available for the transformer. These 13 missing
comparisons explain the difference between 1,872 requested and 1,859 completed
comparisons. There were zero observed annotation differences. Exact Document
equality implies equal deterministic features, so no repeated feature
extraction was necessary.

The results provide no evidence that identical-text variability, interpreter
restart or the tested order change explains the preceding source/candidate
disagreements. They establish repeatability for these selected texts and
settings. They do not identify the mechanism behind differences after an edit.
In particular, the window configuration alone cannot attribute a changed
dependency to a changed transformer window.

The next diagnostic should retain all 60 edits and compare their whole-post
analyses with analyses of the exact edited sentence and its mapped counterpart
in isolation. Missing or changed sentence correspondence must remain in the
denominator. Isolation changes linguistic context and encoded positions
together; local agreement would not isolate window causation or authorize
overriding a whole-post rejection. Existing punctuation and missing-case
limitations also remain unresolved.

The required workspace checks passed: 869 tests in 44 suites, installed spaCy
integration, formatting, Clippy and the release build. All 41 bound source
files remained unchanged. No author model was fitted, and no LLM or detector
was called.
