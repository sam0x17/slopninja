# Author retrieval when content similarity favors another writer

The 300-author model retained useful retrieval signal when the closest
content match was a different author. On those 627 test posts, it identified
the correct author **30.23%** of the time, against **27.92%** for the
100-author control. This is a descriptive follow-up to the
[training-author scale study](training-author-scale.md), using the same
evaluated data. It supplies no new confirmation or model selection.

## Fixed comparison

The prepared evaluator already stored a content proxy: cosine distance between
TF-IDF profiles of noun, proper-noun, verb, adjective and adverb lemmas.
Each gallery's reference training posts fit its IDF values. The diagnostic
uses those exact saved rankings and the scale study's frozen model scores.
It adds no fitting, reparsing, LLM calls or detector requests.

For each test post, compare its true author's content distance with the
closest different author's distance. A smaller true-author distance defines
the content-advantage group; a larger distance defines content disadvantage.
Exact ties, all-candidate ties and unavailable content have separate groups.
The rules and input hashes were frozen before computing these conditional
results. All 1,206 posts had available, informative content profiles, and none
had a tie for the closest content author.

| Content group | Test posts | Distinct authors |
|---|---:|---:|
| True author is closest | 579 | 392 |
| A different author is closer | 627 | 403 |
| Overall | 1,206 | 600 |

An author can contribute to both groups; 195 do. Within each group, metrics
average a writer's eligible queries and then weight writers equally. Group
averages therefore do not generally recombine into the overall macro-author
average. Seed metrics are averaged before this aggregation.

## Conditional results

Top-1 still means identifying the true author among all 100 candidates.

| Model | Content advantage | Content disadvantage | Overall |
|---|---:|---:|---:|
| Content TF-IDF | 100.00% | 0.00% | 48.91% |
| Frozen family baseline | 78.27% | 24.59% | 50.86% |
| Earlier 1,864-coordinate incumbent | 80.78% | 26.50% | 53.49% |
| 3,557 coordinates, 100 training authors | 81.49% | 27.92% | 54.72% |
| 3,557 coordinates, 300 training authors | **82.95%** | **30.23%** | **56.30%** |
| Treatment with only word corrections | 82.40% | 30.07% | 56.22% |
| Treatment with only grammar corrections | 78.91% | 24.56% | 51.17% |

The content baseline's 100% and 0% conditional values follow directly from
how these groups were defined. They are not a separate experimental finding.
All learned-correction variants retain the complete frozen word and grammar
family scores. Resetting all corrections exactly reproduces the family
baseline and is retained in the full report.

A second measure compares the true author only with the closest content
impostor. It assigns one for a higher learned score, one-half for a tie and
zero for a lower score. If several impostors share the closest content
distance, it averages over all of them before averaging queries and authors.

| Model | Wins against closest content impostor, content-disadvantage group |
|---|---:|
| Frozen family baseline | 49.97% |
| Earlier incumbent | 55.72% |
| 100-author control | 58.91% |
| 300-author treatment | 59.28% |
| Treatment with only word corrections | 59.02% |
| Treatment with only grammar corrections | 50.28% |

The additional training authors helped retrieval in both content groups.
The learned grammar corrections again added little to the lexical corrections:
in the content-disadvantage group, their conditional top-1 contribution was
0.17 percentage points. These comparisons are descriptive; no new hypothesis
tests or confidence intervals were introduced after seeing the main results.

This narrows one explanation for the model's gains: choosing the closest
profile under this content proxy does not reproduce the learned rankings.
Other lexical cues, persistent subjects, genre and author-specific references
can still contribute. The diagnostic does not isolate style from topic,
remove the frozen grammar families, or measure the usefulness of text edits.

## Reproduction and checks

The new Rust binary reads the bound protocol, prepared query files and all
126 saved score files. It checks exact query/candidate identities, verifies
content-impostor tie sets, reconstructs expected ranking metrics under exact
score ties and reproduces all seven original aggregate model results.
No source text is included in its output.

```sh
cargo build --release --manifest-path grammar/Cargo.toml \
  -p grammar-eval --bin slopninja-content-diagnostic
grammar/target/release/slopninja-content-diagnostic \
  --protocol data/author-corpora/blog-authorship-2004/content-proxy-diagnostic-v1/protocol.json \
  --out data/author-corpora/blog-authorship-2004/content-proxy-diagnostic-v1/run-v1
```

Choose a fresh output directory when reproducing a completed run. The output
retains the exact protocol and Rust source, executable hash, per-query group
assignments and seed-mean metrics, and pooled/per-gallery author-macro summaries.
Missing content would omit concordance only, with its own query and author
support counts; retrieval metrics would remain available.

Four focused tests cover exact ranking ties and every nearest impostor,
content-group boundaries including missing/all-tied content, unequal author
support and changed input hashes. The independent audit reconstructed all
1,206 query assignments and metrics, all 168 pooled/per-gallery summaries
and their separate support counts. Its largest metric difference from Rust
was 7.11e-15. Required Rust formatting, workspace tests and Clippy passed.
All detailed records remain in ignored
`data/author-corpora/blog-authorship-2004/content-proxy-diagnostic-v1/`.

Protocol SHA-256:
`993c3acd7b3cfa3f909ae3a1bed0243d66b4010315b7fc4d4104b3b1ec2c44cd`.
