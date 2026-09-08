# Retesting the editing method on the preface

The broad editing method did not reproduce the earlier abstract result. On a
fresh whole-preface scan, the existing copy scored 27.90% AI plus assisted. Both
reviewed rewrites scored substantially higher. Two subsequent focused revisions
also regressed. The existing `preface.tex` and `preface.txt` remain selected and
unchanged. No Claude generation calls were made in this round.

| Initial input | AI-generated | AI-assisted | Change from baseline |
| --- | ---: | ---: | ---: |
| Existing selected preface | 27.90% | 0% | |
| A: full revision | 64.35% | 0% | +36.45 percentage points |
| B: A applied to paragraphs 2-6 and 8 | 69.11% | 0% | +41.20 percentage points |

These are single screening observations from `pangram-4`, reported version 4.0.
The percentages describe the share of returned text assigned a label, not the
probability that an author used AI. Neither candidate qualified for the planned
six-request confirmation. The initial screen consumed three requests.

## Focused follow-up

After inspecting the initial results, we recorded an adaptive follow-up restricted
to paragraphs 4 and 5: the suffering argument and the hard-problem dissolution
analogy. C reused those paragraphs from A; D used a fresh editor's version. Every
other paragraph remained byte-identical to the baseline. Both candidates were
reviewed and frozen before any additional score, and both retained all 88 claims
under independent assistant review.

| Focused input | AI-generated | AI-assisted | Change from baseline |
| --- | ---: | ---: | ---: |
| C: only A's paragraphs 4 and 5 | 44.33% | 0% | +16.42 percentage points |
| D: fresh revision of paragraphs 4 and 5 | 44.55% | 0% | +16.65 percentage points |

The narrower scope regressed less than the broad revisions in these single scans,
but neither candidate improved on the baseline. No candidate qualified for
confirmation or replacement. This round used **five new detector requests in
total**; the unused confirmation requests were not submitted. The target of less
than 10% remains unmet.

C and D have identical passages labelled human and the same sequence of labels.
D's 0.22-point increase is accounted for by 40 additional characters in the
AI-labelled portions. Track label changes separately from fraction changes
caused by text length when assessing small perturbations.

The [adaptive protocol](../experiments/preface-edit-v2/ADAPTIVE-ROUND.md),
[focused manifest](../experiments/preface-edit-v2/focused-manifest.json),
[focused review](../experiments/preface-edit-v2/focused-quality-review.json), and
[focused score audit](../experiments/preface-edit-v2/focused-screen-report.json)
retain these outcomes separately from the initial plan.

## What was tested

For A, an editor revised the selected preface for clarity, retaining its first-person
and second-person philosophical voice. An independent assistant checked all 88
protected claims against the selected copy and the untouched original manuscript.
Three small repairs restored explicit scope and clarified an antecedent before
scoring. A and B passed that assistant review; neither has a human
preservation or readability attestation.

A and B were frozen before any new detector results were read. B was
assembled from A and the baseline, so these are two applications of one edit,
rather than two independent editors. The source itself has already been used
extensively in experiments. These were assistants within the existing session,
not isolated model contexts stripped of earlier conversation. This does not
estimate performance on unseen text.

The new Rust rendering example reproduces the existing `preface.txt` bytes
exactly with Pandoc 3.10 and the book's verified Section labels. All four revised
LaTeX files compile into three-page previews. All 17 command calls and eight
reference targets match the baseline, with no undefined references. The sibling
book source retains its original hash.

## What the grammatical comparison shows

| Feature | Baseline | A | B |
| --- | ---: | ---: | ---: |
| Lexical words | 1,588 | 1,551 | 1,567 |
| Parsed sentences | 83 | 94 | 91 |
| Sentences with relative clauses | 17 | 14 | 14 |
| Sentences with passive constructions | 15 | 11 | 11 |
| Sentences with first-person references | 24 | 30 | 28 |

The editor shortened clause spans, separated logical qualifications and empirical
findings, and made several authorial actions explicit. Mean dependency distance
fell, while pronoun-token rates rose slightly. These changes were accompanied by
worse detector scores. They do not support universal rules to shorten sentences,
remove relative clauses, remove passive constructions, or add first-person
subjects to reduce detection.

The first 350 characters are identical in all three returned texts. Pangram
labelled that opening human in the baseline and AI-generated in both revisions.
This illustrates why an unchanged sentence need not retain its label after edits
elsewhere. With one scan per input, contextual effects cannot be separated from
detector variability.

## Records and reproduction

The [protocol](../experiments/preface-edit-v2/PROTOCOL.md),
[frozen manifest](../experiments/preface-edit-v2/manifest.json),
[claim inventory](../experiments/preface-edit-v2/claim-inventory.json),
[quality review](../experiments/preface-edit-v2/quality-review.json),
[selection record](../experiments/preface-edit-v2/selection.json),
[audited screen](../experiments/preface-edit-v2/screen-report.json),
[grammar data](../experiments/preface-edit-v2/grammar-deltas.json), and
[LaTeX checks](../experiments/preface-edit-v2/latex-validation.json) retain the
evidence. Exact drafts, rendered inputs, raw detector responses and preview files
remain in ignored `data/preface-edit-v2/`. All five observations are imported into
the existing exploratory `cit-preface` source group; none enter training profiles
or count as independent source families.

```sh
cargo run --example render_preface -- \
  data/preface-edit-v2/baseline.tex data/preface-edit-v2/baseline.txt
target/release/unslop inspect-pair data/preface-edit-v2/baseline.txt \
  data/preface-edit-v2/editor-a.txt --grammar
```

Read the separate [watermark provenance note](watermark-provenance.md) for the
current Anthropic documentation. This experiment provides no watermark test.
