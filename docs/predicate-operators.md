# Predicate-local operator sequences

`predicate_operator_sequence_v1` records auxiliary, copular, and negation
annotations together at each eligible clause head. The Rust implementation is
[`predicate_operators.rs`](../grammar/crates/grammar-eval/src/predicate_operators.rs).
It reads a validated `grammar_core::syntax::Document` and preserves the parser
identity. Extraction requires no language model or corpus fitting.

The eligible head set is exactly the original `clause_head_frame` set: lexical
roots, `csubj`, `csubjpass`, `csubj:pass`, `ccomp`, `xcomp`, `advcl`, `acl`,
`relcl`, `acl:relcl`, and `VERB`/`AUX` conjuncts. A lexical token must contain a
Unicode letter and have neither its punctuation nor space flag set. This set
includes fragments and nonfinite clauses.

Each eligible head emits one distribution event. Its context contains the
incoming dependency, with self-headed roots represented as `ROOT`, and the
head's POS. The main predicate lemma is omitted. A head tagged `AUX` contributes
an explicit `HEAD_AUX` operator at `SELF`; its parser lemma is retained.

Starting at that head, extraction follows lexical `aux`, `auxpass`, `aux:pass`,
and `cop` children. It follows the same edges through auxiliary/copular children
without a fixed depth limit. It also includes lexical `neg` children of the
head or any included auxiliary/copular node, then stops at those negation nodes.
It does not follow `xcomp`, `ccomp`, `conj`, or arbitrary descendants during
this traversal. A separately eligible embedded head receives its own event.
An `advmod` such as *never* is not converted into `neg` using its word or lemma.

The two passive auxiliary labels map to `PASSIVE_AUX`; the remaining roles are
`AUX`, `COP`, and `NEG`. Operators remain in token order, including repetitions.
Each stores its side relative to the head, its canonical dependency-role path,
and the identity of its parent as `HEAD` or an operator ordinal in that ordered
sequence. Parent identity distinguishes two same-role auxiliary siblings even
when their role paths match. Absolute token indices are excluded from the
distribution key.

The shared [`lexical_context.rs`](../grammar/crates/grammar-eval/src/lexical_context.rs)
helper normalizes each parser lemma with NFC, Unicode lowercase, then NFC.
It preserves punctuation, internal or surrounding spaces, and literal parser
placeholders. An empty or whitespace-only lemma is JSON `null`. Extraction
never substitutes the surface word or splits a lemma into words. Thus missing
lemmas remain observed missing values, and do not remove an opportunity.

The finite-carrier candidates are the head and included auxiliary/copular
nodes with POS `VERB` or `AUX`. A candidate qualifies when its morphology
contains `VerbForm=Fin` or its tag is `VBD`, `VBP`, `VBZ`, or `MD`. The signature
contains `NONE`, `ONE`, or `MULTIPLE` and, for each qualifying carrier:

- Its position as `HEAD` or an operator ordinal, POS, and supplied tag.
- Sorted supplied values for `VerbForm`, `Tense`, `Mood`, `Person`, and `Number`,
  with absent fields represented explicitly as JSON `null`.
- Separate flags for the finite-tag and `VerbForm=Fin` evidence, plus any other
  supplied `VerbForm` values.

A finite tag paired with `VerbForm=Inf`, for example, remains visible as
conflicting evidence. An AUX head appears once among the finite candidates,
as `HEAD`. Unknown morphology does not imply a tense or a finite carrier.
`NONE` means that no eligible candidate had the specified parser evidence.

Keys are serialized JSON structures with tagged variants and explicit arrays.
Lemmas or parser labels containing delimiters cannot merge fields or change
list boundaries. A head with no included operator emits an explicit `NONE`
operator sequence, alongside its finite signature. With no eligible heads,
the family has zero opportunities and an empty count map. The denominator is
the number of eligible heads; counts sum exactly to that integer.

The standalone `PredicateOperatorExtractor` wraps the unchanged default
extractor, preserving its original 14 families and adding this family under a
new feature-schema identity. It does not silently add the separate clause-child
family. A combined extractor must declare its own composition and schema.

The API also exposes `PredicateObservation { head_index, event }` records for
source alignment. These records let an edit evaluator compare the same
predicate before and after a change; aggregating events loses that alignment.
For example, replacing *leave* with *stay* can preserve this delexicalized family
while changing the claim. Swapping modals between otherwise similar clauses can
also preserve a document's event counts. This representation alone cannot
certify meaning preservation. Its paths describe parser attachment and do not
establish semantic negation scope.

If the parser gives *do not* and *don't* the same operator lemmas and compatible
annotations, they produce the same operator event. Contraction spelling remains
available in the existing lexical features. A parser disagreement is reported
as an annotation difference; this module does not interpret it as semantic loss.

The focused tests use synthetic annotations for nested operators, repeated
auxiliaries, sibling-parent distinctions, passive aliases, copulas, missing
lemmas, finite-evidence conflicts, malformed syntax, Unicode eligibility, and
zero opportunities. The installed spaCy integration parses five synthetic
sentences and checks the legacy family bytes and clause-head denominators.

To extract from an existing, hash-bound annotation without loading a parser:

```sh
cargo run --manifest-path grammar/Cargo.toml -p grammar-eval \
  --bin slop_ninja-predicate-operators -- \
  --input annotation.json --expected-sha256 ANNOTATION_SHA256 --out result.json
```

The command requires a new output path. Its JSON contains the versioned
features and separate source-aligned observations.
