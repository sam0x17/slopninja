# Controlled edit results

One complete scientific abstract revision scored **0% AI plus assisted text on
three fresh confirmation scans**, while its unchanged model baseline scored
100% on all three. A minimal relative-clause edit in another abstract repeatedly
reduced the combined score from 72.31% to 36.61%. Neither result establishes a
reliable transformation across new documents.

The [frozen protocol](../experiments/edit-screen-v1/PROTOCOL.md) selected three
previously flagged training sources. The screen contained 15 candidates and six
fresh baselines, followed by 12 confirmation requests for two selected candidates.
Every request used `pangram-4` and returned version `4.0`. Candidate text was fixed
before its first scan; confirmation used new task IDs and alternating arms.

## Word and sentence screens

All edits below were applied independently to the same complete Codex abstract.
Each percentage is one full-context observation, including AI-assisted text.

| Source | Edit | Baseline | Candidate |
| --- | --- | ---: | ---: |
| Farmers/BVD | principal to main | 56.48% | 56.66% |
| Farmers/BVD | Nevertheless to Still | 56.48% | 56.27% |
| Farmers/BVD | Promote examined to the main verb | 56.48% | 56.06% |
| Farmers/BVD | Move the concession to the end | 56.48% | 56.71% |
| Ecotourism | sufficient to enough | 72.31% | 72.25% |
| Ecotourism | utility to usefulness | 72.31% | 83.70% |
| Ecotourism | Projects that adopt to Projects adopting | 72.31% | 36.61% |
| Ecotourism | conducted monitoring to monitored; front the method | 72.31% | 36.92% |
| Sierra Leone | persist to remain | 47.55% | 69.84% |
| Sierra Leone | adequate to sufficient | 47.55% | 47.49% |
| Sierra Leone | Put the study location first | 47.55% | 47.52% |
| Sierra Leone | Simplify the final social-context relative clause | 47.55% | 24.85% |

The reduced-relative comparison was selected for confirmation because it changed
little text while producing a large score difference. Three new baseline scans
were all 72.3111%, and all three candidate scans were 36.6096%. That supports a
local effect of this exact edit in this document. It does not establish that
reducing relative clauses will consistently lower a detector score.

The [observation audit](../experiments/edit-screen-v1/observation-audit.json)
checked the returned labels as well as the percentages. Four word swaps changed
only character weighting, with no decoded label change; the other two worsened
labels. None of the six word swaps reduced the amount of text assigned an
assisted label. For the reduced-relative edit, the edited sentence stayed
AI-assisted. Three later, unchanged sentences containing 646 characters switched
from assisted to human, and all confirmation scans reproduced that change.
This demonstrates an effect beyond the edited sentence, without identifying its
cause inside the detector.

## Complete revisions

A separate editor revised three Claude Opus abstracts before seeing their
human originals or detector scores. Independent assistant review then compared
the drafts with both the model inputs and the human originals. Corrections were
made before scoring, so this arm includes source-aware preservation repairs.

| Source | Baseline | Complete revision | Fresh confirmation |
| --- | ---: | ---: | --- |
| Ecotourism | 100% | 18.80% | Not selected |
| Farmers/BVD | 89.83% | 78.90% | Not selected |
| Sierra Leone | 100% | 0% | Baseline 100%, candidate 0%, in all three pairs |

The Sierra Leone revision retained the seven focus groups, 22 interviews, two
villages, sampling population, analytical method, access/use distinction, every
listed barrier, interactions among barriers, and the recommendation's structural
and equity conditions. The review restored the claim about access to a skilled
birth attendant, preserved the reference to the women's own communities, and kept
the general scope of the findings. The [review record](../experiments/edit-screen-v1/editor-b-review.json)
distinguishes new editing losses from inherited model distortions.

The ecotourism model's claim that the strategy is widely promoted remains in
its revision but is not supported by the human abstract. Preserving the immediate
model input does not guarantee fidelity to an earlier source or factual truth.

These reviews were performed by assistants. `reviewed_by_human` remains false;
the combined quality-and-detector gate has not passed. No source manuscript was
changed. Exact tested texts and raw responses remain in local `data/edit-screen-v1/`.

## What the features do and do not explain

All three complete revisions reduced relative-clause and pronoun-token rates.
They also reduced lexical word counts by roughly 11-16%. These concurrent changes
prevent attributing the whole-document results to one grammatical coordinate.
The BVD revision had no parsed relative-clause sentences and still scored 78.90%.
Removing a construction is not sufficient to produce a low score.

The single-word results do not support a rule of replacing formal words with
more common ones. Our FineWeb pilot contains more occurrences of remain than
persist, yet that substitution worsened the Sierra Leone result. It also contains
no usefulness occurrences, although that word might sound simpler than utility.
Frequency profiles can suggest experiments; they cannot certify a useful edit.

The [subsequent development evaluation](fixed-process-results.md) froze a complete
editing instruction without supplying human originals or detector feedback.
It produced no new detector passes: two already-zero inputs stayed at zero and
two flagged inputs worsened. This separates the successful exploratory edits
from the performance of that repeatable process on different source families.

## Reproduction

```sh
cargo run --example edit_screen -- prepare \
  experiments/edit-screen-v1/manifest.json data/edit-screen-v1/prepared
target/release/unslop score-corpus data/edit-screen-v1/prepared/prepared.jsonl \
  --out data/edit-screen-v1/scores --max-requests 21
cargo run --example edit_screen -- report \
  data/edit-screen-v1/prepared/prepared.jsonl data/edit-screen-v1/scores \
  experiments/edit-screen-v1/screen-report.json
```

Preparation and reporting are offline Rust operations. The scoring command uses
Pangram credits; completed local results are reused. Existing collection files are
required to reproduce the exact generated baselines. The prepared JSONL is an
experimental scoring input, not a corpus to ingest into training word profiles.

Tracked evidence includes the [complete screen report](../experiments/edit-screen-v1/screen-report.json),
[Sierra Leone confirmation](../experiments/edit-screen-v1/confirmation-sierra.json),
[ecotourism confirmation](../experiments/edit-screen-v1/confirmation-ecotourism.json),
and [grammar measurements](../experiments/edit-screen-v1/grammar-deltas.json).
