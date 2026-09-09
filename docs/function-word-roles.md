# Function words in syntactic roles

`function_word_role_v1` counts parser lemmas together with their local syntactic
roles. It connects word choice to grammatical use: the original word and word
bigram families count surface forms, while the original dependency, POS and
frame families omit lexical identity. This new family therefore combines
lexical and grammatical information.

The implementation is
[`function_word_roles.rs`](../grammar/crates/grammar-eval/src/function_word_roles.rs).
It reads a validated syntax `Document`. Extraction requires no fitting or parser
invocation, and adds no family to the default extractor automatically. A caller
that combines families must declare that composition under a new feature schema
and retain the source and parser identities.

A token emits exactly one event when both conditions hold:

- The original lexical predicate accepts it: neither `is_space` nor `is_punct`
  is set, and its surface text contains at least one Unicode letter.
- Its supplied UPOS is `ADP`, `AUX`, `CCONJ`, `DET`, `PART`, `PRON` or `SCONJ`,
  or its supplied dependency relation is exactly `neg`.

Matching both the POS and negation conditions still produces one event. There
is no word list, inferred negation label or head-eligibility filter. The
governing head comes from the validated dependency annotation.

Each key is a serialized JSON tuple in this exact order:

```text
[normalized_lemma_or_null, own_POS, incoming_relation, head_POS, side]
```

`side` is `L` or `R` according to the token's index relative to its head. A
self-headed token uses `SELF` and its own POS as `head_POS`. Incoming relation
labels remain unchanged, including the parser's root label. For example,
synthetic annotations for *We see them* can produce:

```json
["we", "PRON", "nsubj", "VERB", "L"]
["they", "PRON", "dobj", "VERB", "R"]
```

The second example assumes the parser supplies `they` as the lemma of *them*.
The key has no additional fields for the head's lemma, surface spelling,
morphology or absolute positions. JSON encoding preserves field boundaries even
when a lemma or parser label contains quotes, separators or control characters.

The shared
[`lexical_context.rs`](../grammar/crates/grammar-eval/src/lexical_context.rs)
helper applies NFC, Unicode lowercase, then NFC to the supplied lemma. It
preserves punctuation, apostrophes, spaces and literal parser placeholders.
An empty or whitespace-only lemma becomes JSON `null`. It never substitutes
the token's surface text or splits a lemma into words. A missing lemma and the
literal string `"null"` have distinct keys.

The denominator is the number of eligible token events, including events with
missing lemmas. Repeated keys retain their full integer count. Counts sum to
the denominator; no vocabulary pruning or smoothing occurs. With no eligible
tokens, extraction returns an empty count map and zero opportunities, meaning
the family is unavailable for that document.

`document_family` returns the distribution. `extract` additionally returns
`missing_lemma_events` and individual events. These audit records include the
token and head indices and byte spans into the unchanged source. Those fields
stay outside the feature key, so they can support later edit alignment without
becoming author-style coordinates.

The synthetic tests cover eligibility, negation without double counting,
repetition, missing lemmas, Unicode normalization, key collisions, head roles,
positions, malformed syntax and zero opportunities. The installed spaCy test
checks function-word and negation events on synthetic sentences.

This representation describes usage, not semantic equivalence. A contraction
may retain the same role events when the parser supplies the same lemmas;
existing surface-word features can still record its spelling. A sentence split
may change root or attachment events without changing the claim. The
[edit-utility notes](edit-utility-matrix.md) describe the separate checks needed
when evaluating edits, alongside the [predicate-operator family](predicate-operators.md).
