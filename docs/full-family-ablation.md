# Removing complete word or grammar contributions

The complete grammar component contributes **2.39 percentage points** of
macro-author top-1 accuracy in the frozen model: **56.30%** with words and
grammar, versus **53.91%** with word families alone. Grammar families alone
score **26.01%**. This descriptive result supports retaining grammar while
investigating a less sparse representation.

This diagnostic measures how the selected 300-author model uses its complete
word and grammar components. The earlier
[coordinate reset](training-author-scale.md) removed learned grammar
corrections while preserving every original grammar-family contribution.
That test could not establish whether the grammar component itself helps.

## Results

Each query ranks 100 candidate authors. All results average the three
inherited-seed metrics, with equal weight per author.

| Retained components | Top-1 | Top-5 | MRR |
|---|---:|---:|---:|
| Words and grammar | 56.30% | 74.57% | 0.6498 |
| Word families only | 53.91% | 71.99% | 0.6271 |
| Grammar families only | 26.01% | 47.60% | 0.3677 |

Adding grammar to the lexical score raises top-1 by 2.39 points and top-5 by
2.58 points. Author-level top-1 increases for 67 writers, decreases for 34 and
is unchanged for 499. These are paired differences within this fixed model,
without new confidence intervals or claims about a retrained lexical model.

The content groups retain their previous definitions and support counts.

| Retained components | True author closest by content, 579 posts / 392 authors | Different author closer by content, 627 posts / 403 authors |
|---|---:|---:|
| Words and grammar | 82.95% | 30.23% |
| Word families only | 79.59% | 26.80% |
| Grammar families only | 37.17% | 13.37% |
| Full minus lexical, percentage points | +3.36 | +3.43 |

Grammar helps top-1 in both content groups. The conditional gains do not
average back to the overall gain because each group weights its participating
authors equally; writers can contribute different proportions of their posts
to each group.

There is also a useful difference between global retrieval and the comparison
against one content-matched alternative. In the content-disadvantage group,
grammar alone beats the closest content impostor on **62.66%** of comparisons,
versus **59.28%** for the combined model and **54.60%** for lexical families
alone. Yet it identifies the true author among all 100 candidates only 13.37%
of the time. Optimizing global author retrieval and separating a writer from
the nearest content alternative are different objectives. Neither measures
semantic preservation or establishes independence from topic.

The [TRAIN representation audit](grammar-representation-audit.md) suggests a
specific next change: retain the current full clause frames and add counts
of their shared child relations. The original frame keys are sparse; the new
projection shares evidence across frames while discarding some detail within
its own family. A matched new calibration study must establish whether that
change improves retrieval.

## Fixed scoring

The diagnostic reuses all three selected treatment seeds and the six evaluated
test galleries: 600 authors, 1,206 posts and 100 candidate writers per query.
No parameter, checkpoint, vocabulary threshold or content-group boundary is
selected from these results.

For each query and candidate, the frozen model first adjusts the complete
fourteen family distances using its 3,557 learned coordinate multipliers.
Unselected and unseen coordinates keep multiplier one. The model then applies
its inherited monotone spline to each family distance, multiplies by the
original family weight, sums the retained terms and divides by the original
temperature with a negative sign.

| Variant | Terms retained in the final sum |
|---|---|
| Full | All fourteen families |
| Lexical families only | Word and word-bigram families |
| Grammar families only | The other twelve families |

Both ablations retain the original weights; there is no renormalization or
refitting. This measures each component's contribution inside the fixed model.
It does not measure the best achievable performance after training a model
specifically for one component.

The full score uses the original Rust scorer's summation order so that exact
rankings and ties can be checked against the saved Python predictions. The
sum of lexical and grammar scores must reproduce the full score within
floating-point summation tolerance. Per-family contributions remain available
for inspecting what was removed.

## Aggregation and content groups

Expected top-1, top-5 and reciprocal-rank metrics average over exact score ties.
The nearest-content-impostor measure averages a win, half-credit tie or loss
against every different author tied for closest under the existing content
proxy. The report gives each measure separately.

The [content diagnostic](content-proxy-diagnostic.md) supplies the exact saved
query assignments and tied impostor IDs. The groups distinguish content
advantage, disadvantage, equality, all-candidate ties and unavailable content.
The ablation does not redefine those groups. Missing content would omit the
impostor comparison while retaining retrieval metrics, with separate support
counts for each measure.

Seed metrics are averaged per query. Query means are then computed within
each author and group, followed by an equally weighted mean over authors.
The report retains pooled and per-gallery summaries, individual author means
and paired full-minus-lexical/full-minus-grammar differences. It also counts
authors with positive, exactly zero and negative differences per metric.
Authors can appear in multiple content groups, so the conditional macro-author
means do not generally recombine into the overall value.

These are descriptive analyses after fitting and evaluation. No new confidence
intervals, hypothesis tests or model-selection decisions are introduced.
The six galleries are development history for future representation work.
Conditional performance under one content proxy does not establish topic
independence, and retrieval does not certify useful text edits.

## Reproduction and verification

```sh
cargo build --release --manifest-path grammar/Cargo.toml \
  -p grammar-eval --bin slop_ninja-ablate-families
grammar/target/release/slop_ninja-ablate-families \
  --protocol data/author-corpora/blog-authorship-2004/full-family-ablation-v1/protocol.json \
  --out data/author-corpora/blog-authorship-2004/full-family-ablation-v1/run-v2
```

Choose a fresh output directory when reproducing a completed run. The report
retains all three variants' logits, signed per-family contributions, per-query
metrics, author means, paired differences, and exact artifact/source hashes.

The independent audit reconstructed **1,085,400 candidate scores** and all
**10,854 rankings and tie sets** across three variants, three seeds and six
galleries. It verified all 126 model summaries, 84 paired summaries, 8,370
author rows and 5,580 paired-author rows. The largest independent score
difference was 4.27e-14. Full scores reproduced all prior Python rankings;
their largest numerical difference was 5.69e-14. The final Rust formatting,
workspace tests and Clippy checks passed, including eleven focused ablation
tests and the new clause-child extractor.

The authoritative output is `run-v2`. The first run serialized array filenames
in an unsuitable object form. A string-path regression test and corrected
export fixed that metadata error. Every numerical array and query/author row
is byte-identical between the preserved first run and the corrected run.
The separate `figures-v2/` directory contains standalone PNG, SVG and PDF
figures, aggregate CSV data and provenance bound to the corrected report.

Protocol SHA-256:
`ce665d5fe3db67afc9c6576e6a25faccc6c1a874bb31b7d1bba2b4197c1784e2`.
Final report SHA-256:
`78515bb8d5f74200981b2bf078cc7a9484605b2ebb3dc9c71291444ca93afad7`.

Exact inputs, predictions, source snapshots and detailed results stay in
ignored `data/author-corpora/blog-authorship-2004/full-family-ablation-v1/`.
