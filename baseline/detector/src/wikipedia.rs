//! Conservative historical Wikipedia text matching.
use regex::Regex;
use scraper::{ElementRef, Html, Node, Selector};

pub fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

// Deliberately partial. Unhandled markup creates a mismatch, never a guessed
// completion. No templates are expanded, so rendered-at-retrieval additions
// cannot by themselves establish historical wording.
pub fn literal_text(wikitext: &str) -> String {
    let mut text = wikitext.to_owned();
    for pattern in [
        r"(?s)<!--.*?-->",
        r"(?is)<ref\b[^>]*?/>|<ref\b[^>]*>.*?</ref\s*>",
    ] {
        text = Regex::new(pattern)
            .unwrap()
            .replace_all(&text, "")
            .into_owned();
    }
    text = Regex::new(r"\[\[(?:[^\[\]|]+\|)?([^\[\]]+)\]\]")
        .unwrap()
        .replace_all(&text, "$1")
        .into_owned();
    text = text.replace("'''", "").replace("''", "");
    // Decode character entities and omit remaining HTML tags without rendering
    // MediaWiki templates, parser functions or their parameter values.
    normalize(
        &Html::parse_fragment(&text)
            .root_element()
            .text()
            .collect::<String>(),
    )
}

pub fn lead_text(html: &str) -> String {
    let doc = Html::parse_fragment(html);
    let selector = Selector::parse(".mw-parser-output > p").unwrap();
    let paragraphs: Vec<_> = doc
        .select(&selector)
        .map(|p| {
            p.descendants()
                .filter(|node| {
                    !node.ancestors().filter_map(ElementRef::wrap).any(|a| {
                        ["sup", "style", "script", "table"].contains(&a.value().name())
                            || a.value()
                                .classes()
                                .any(|c| ["reference", "noprint", "mw-editsection"].contains(&c))
                    })
                })
                .filter_map(|n| {
                    if let Node::Text(t) = n.value() {
                        Some::<&str>(t)
                    } else {
                        None
                    }
                })
                .collect::<String>()
        })
        .collect();
    normalize(&paragraphs.join(" "))
}
