# Do author scores prefer the writer's grammatical choices?

The existing neural author score and the conditional contraction model usually
preferred different forms in this fixed synthetic comparison. They agreed in
19 of 90 comparisons on isolated sentences and 3 of 90 after appending the same
unrelated sentence. The neural model preferred the expanded form in all 90
appended comparisons. These results limit its use as an edit objective.

The [later-post evaluation](preference-chronology.md) supplies separate evidence
that author contraction histories help predict subsequent choices. Neither
study establishes the best phrasing for a particular sentence or audience.

## Fixed comparison

Reuse the six directional fixtures from the
[conditional preference study](conditional-edit-preferences.md). They contain
three pairs, each presented in both directions:

| Pair | Expanded example | Contracted example |
|---|---|---|
| does | The service does not stop. | The service doesn't stop. |
| could | The client could not retry. | The client couldn't retry. |
| can | The client cannot retry. | The client can't retry. |

Use the same ten target authors selected by lexical ID order in the earlier
[edit utility study](edit-utility-matrix.md), their same 151 TRAIN posts, and
the three fixed neural seeds (0, 17, 29). Verify that the target posts match
the conditional model's TRAIN history by post ID, author, date and panel.
The conditional population background remains the original 300 TRAIN authors.
There is no new author allocation, model fit, parameter update or selection.

For each author and seed, compare the two individual neural logits. Do not
average across authors or seeds to choose an edit. A higher candidate logit
means that the author model prefers that candidate. The conditional model
prefers contraction when its probability is greater than 0.5 and expansion
when it is less than 0.5. Preserve exact ties and unavailable scores separately.

Evaluate each pair as raw text and with this exact suffix on both forms:

> After the reviewer of the draft had checked the notes, the editor put the folder on the desk beside the lamp.

The suffix follows two newline characters. The conditional predictor receives
the raw auxiliary/context key in both conditions, so its context invariance is
imposed by the interface. This experiment measures the neural score's response
to the added context; it does not show that the conditional model learned to
ignore irrelevant material.

Retain all 360 directional rows. Use only the expanded-source direction for
the primary 180 rows: three pairs, ten authors, three seeds and two contexts.
Check that reversing each pair preserves both models' canonical preferred
form. The two directions are the same comparison, not independent evidence.

Run the unchanged claim guard on the raw pair. Keep its eligibility decision
separate from each model's preferred form. Retaining a source because the
guard rejects its alternative does not count as agreement between models.
The appended context is not claim-guarded.

## Results

All 12 unique texts parsed successfully. Both neural scores were available in
every comparison, with no exact ties. All 180 inverse checks passed.

| Context and personal support | Comparisons | Same preferred form | Opposite preferred form |
|---|---:|---:|---:|
| Raw, all | 90 | 19 (21.11%) | 71 |
| Raw, personal context history | 57 | 16 (28.07%) | 41 |
| Raw, population fallback | 33 | 3 (9.09%) | 30 |
| Appended, all | 90 | 3 (3.33%) | 87 |
| Appended, personal context history | 57 | 3 (5.26%) | 54 |
| Appended, population fallback | 33 | 0 | 33 |

The conditional model preferred contraction in 87 of 90 comparisons: all
`could` and `can` cases and 27 of 30 `does` cases. Its three expansion choices
are one target author's preference repeated across the three neural seeds.
The neural model preferred expansion for all `does` and `can` cases in both
contexts. For `could`, it preferred contraction in 16 of 30 raw comparisons
and none of the appended comparisons. Those 16 context-dependent reversals
account for the entire change in agreement.

The raw claim guard accepted both directions of `does` and `could`. It rejected
both directions of `can`, retaining the source for each model regardless of
the model's preference. This repeats the prior parser limitation: spaCy supplies
different finite-verb morphology for `cannot` and `can't`. The guard's `changed`
label here records those parser observations; it does not establish changed
meaning. All expected outcomes, including these two mismatches, remain saved.

This is a small engineering diagnostic with synthetic text, reused target
authors and correlated seeds. Agreement is not accuracy. A population fallback
is not evidence of a particular author's observed preference. Natural-language
context, emphasis and register can justify an expanded form even when an author
usually contracts it. No confidence interval, human preference judgment,
readability assessment or detector measurement is attached to this comparison.

## Implication for the representation

Author recognition and edit selection need separate evaluation. A model can
recognize a writer while assigning a higher score to a local choice that the
writer seldom makes. Continue using the existing neural model for the retrieval
task on which it was evaluated. For supported grammar alternatives, the
conditional profiles provide directly testable probabilities and explicit
history counts. They remain a candidate component of an editing process.

Further grammar dimensions need the same checks: define the possible forms,
establish when both forms are available, measure author preferences from earlier
writing, and test predictions on later writing. Claim, tone and readability
checks remain necessary before applying those preferences to a rewrite.

## Reproduction and local artifacts

The Rust entry point is `slopninja-preference-score-comparison`; orchestration
lives in `grammar/crates/grammar-eval/src/preference_score_comparison.rs`.
It uses the existing Rust scorer and preference estimator. Python supplies only
the spaCy parses. No LLM or detector calls occurred.

The protocol, command, logs, annotations, target audit, full directional rows,
canonical rows, support counts, zero-count categories, scores and exact executed
sources are retained in ignored
`data/author-corpora/blog-authorship-2004/preference-score-comparison-v1/`.
Private corpus data and author identifiers remain there.

The reviewed protocol SHA256 is
`889965c3e483abbe8ff0ed77e3f936d6a6d149216dc284f8659f5fc23832e2b6`.
The report SHA256 is
`3906da761471db3922ff0fc49eaa17d11eff3e0cfadf1f92aa9843113b455745`.
The execution receipt SHA256 is
`f68b874a8de167e4c035536e4ea0936d7098f7b0a66e8f9ec0e6af75f8ded407`.

Copy the command from the retained `run-command.sh`, change `--out` to a fresh
directory and execute that copy to reproduce the comparison. Keep the original
command, run and hashes intact.

The independent consistency audit reproduced all directional/canonical
preferences, guard decisions, summary counts and unavailable-family projections
from the saved data. Every full conditional prediction matched the previous
profiles, every raw guard matched its prior result, and all 151 target TRAIN
post bindings matched. This audit did not recompute neural logits or rerun
spaCy. Its receipt SHA256 is
`33577f7e0b19251458877a7c17c437a4ab402f3f0b4057778f58f43573f8d7f4`.
The combined workspace checks passed formatting, 408 test invocations and
Clippy with warnings denied, including 16 installed-parser test invocations.
Shared module tests count once per executable target. The check receipt SHA256 is
`d6e9eb269ced5673d6e191f9d0fe72cda624c2f001a5b4184be5e298035196bd`.
