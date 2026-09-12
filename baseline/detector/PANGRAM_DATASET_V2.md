# Pangram dataset collection within $150

Declared 2026-09-12 before the first submission under the amended budget.
The user authorized at most $150 and requested varied AI percentages,
preferably spanning 60-100%. Apply that ceiling cumulatively, including the
two earlier jobs' $30 of reservations. Their estimated charge is $21.12;
retain reservations until billing evidence permits reconciliation. This leaves
$120 available for new reservations. Each new reservation must cover at least
the realtime-price estimate, above the discounted bulk estimate. The earlier
$292.04 proposal will not be submitted as a whole.

## Measure the requested range

Use Pangram 4's combined AI and AI-assisted content fraction for continuity
with the existing comparison. Also retain the separate AI, AI-assisted and
human fractions and every segment-level result. These fractions describe
detected content, not a calibrated probability of document authorship.

Report bins below 60%, 60-70%, 70-80%, 80-90%, 90% to below 100%, and 100%.
For binning only, treat combined sums within 0.00001 of one as 100%, matching
the collector's floating-point tolerance. Preserve the original numbers.
Aim for coverage across the interior bins,
without discarding observations outside the requested range. Report counts
separately for model-only writing, mixed workflows and human controls.

The cached 57-text Development comparison motivates a pilot: five of its
19 mixed-origin examples have combined fractions between 72.6% and 88.8%.
Two model-only drafts score 51.7% and 55.3%. Their independently recorded
origin labels remain unchanged. Most other nonhuman examples score 100%.
These are prior observations, not results from this new Train collection.

## Start with complete revision families

Use the closed Train revision export from v6: 454 complete families, each
containing a historical human proxy, an original model draft, a direct edit
of the human root, a Qwen Anti-AI revision of the draft and a subsequent
Mistral Fix Slop revision. All five records retain their exact text and
provenance. The input SHA256 is
`06b2731345296e2cd796fc5817227a086606027e766306481afd716a3c190283`.
No Development or Test record enters this collection.

For the pilot, select one whole family per human-source collection and
original-draft profile. Rank families by SHA256 of the JSON pair
`["slop-ninja-pangram-family-sampling-v1", source_group]`, then source group.
The source collections cover academic abstracts, news, encyclopedia prose
and narrative writing. Draft profiles include source-matched, plain,
informal, direct, Anti-AI and Fix Slop. Preserve every record from each
selected family. Report original composers and revision routes separately.
The selector and planner validate ancestry, Train assignments and external
evaluation rights before producing exact bulk requests.

## Spend incrementally on useful coverage

After collecting the pilot, inspect score distributions by origin, stage,
collection, draft profile and composer. Select additional unannotated
families from strata that supply the missing interior score ranges, while
retaining some coverage of other strata. Save the prior observations and
the next selection rationale before each submission. Exclude previously
annotated families from subsequent cohorts. Do not repeatedly buy the same
text or alter text merely to manufacture a desired score.

All submitted items, failures and out-of-range scores remain in the corpus.
Any bin-balanced training view must be a separate manifest with its selection
rule and counts. It cannot replace the complete collection. If the available
workflows do not populate a bin, report the gap and reconsider the next
batch before spending the remaining budget. No exact coverage guarantee is
possible before observing the detector.

This adaptive Train collection can identify difficult examples for a later
declared experiment. It is not an unbiased prevalence estimate or a new
benchmark. Keep the existing 57-text Development comparison separate, and
retain the frozen v6 fitting and Test protocol without Pangram supervision
or feedback. Commercial teacher-supervised fitting and redistribution of
raw provider outputs remain subject to the unresolved agreement questions
recorded in [the original collection plan](PANGRAM_CORPUS_EXPANSION.md).

Use the existing cumulative budget ledger, durable pre-submission
reservations and single-submission policy. Preserve exact inputs and all
returned versions, hashes and receipts; collect results before the provider's
48-hour retention deadline. Run sampling, builds and other intensive work on
the Studio. The account's actual remaining monthly credits are still unknown.

## Pilot results and second cohort

The first cohort returned 120 successful Pangram 4.0 observations from 24
families. All four source collections and six profiles were represented.
Original composers were OLMo (12 families), Qwen (9) and Mistral (3).
The charge estimate is $12.40, with a $15.50 reservation. Cumulative
reservations now total $45.50 under the $150 cap.

| Origin or stage | Below 60% | 60-70% | 70-80% | 80-90% | 90% to below 100% | 100% |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Human controls | 24 | 0 | 0 | 0 | 0 | 0 |
| Original model drafts | 0 | 1 | 2 | 0 | 0 | 21 |
| Human-text edits | 1 | 1 | 1 | 1 | 0 | 20 |
| First model revision | 0 | 0 | 0 | 0 | 0 | 24 |
| Second model revision | 0 | 0 | 0 | 0 | 0 | 24 |

All 24 human controls scored zero. The six nonhuman examples in the requested
interior range span 60.6-85.5%; a seventh scores 51.4%. All 48 successive
revisions score 100%. The observed variation comes from original drafts and
human-text edits. These small source/style groups do not establish a general
effect of a model or instruction.

Bindings: pilot input
`bee8e47a609aeaf9ef3ed4852a48385d6990dead960eb59c43b26ad8104c9fed`,
plan `0aee07ea557c4d904a734bace23b33a1ac9133188cb679e1f804194482ef24d8`,
annotations `c1f2a085eea1585ae7e87d5d0ff5aa260c2474f5e0c5926d48ebaf0f503c4a05`.
All 120 texts are new relative to the earlier 57-text Development collection.

Before selecting or submitting the second cohort, change its input to the
already frozen 471-family Train seed export, SHA256
`d0cfb18e850615def38c5854697a2e2088428fd4ef1a6dce92147f68cfd1a764`.
Each complete seed family has a human root, original draft and human-text edit.
This also permits the 17 original families whose later revision chain failed;
no source is reassigned and no text changes. Keep every seed record from each
selected family, exclude all pilot families, and use the same sampler/ranking.
This avoids spending on additional revision stages after their uniform pilot
scores, while preserving those pilot observations in the complete collection.

Select up to four new families from each of eight declared collection/profile
groups. Five supplied the pilot's lower scores: narrative/direct,
narrative/Fix Slop, PLOS/direct, PLOS/Fix Slop and PLOS/informal. Three provide
additional exploration: narrative/Anti-AI, encyclopedia/informal and news/plain.
This gives at most 32 new families and 96 texts. Record any exhausted groups,
actual composer balance, exact plan and reservation before submission. Review
the second cohort's distribution before choosing a third; the budget ceiling
does not require spending the entire allowance on uniformly high scores.

The second cohort returned all 96 results successfully, again at version 4.0.
It adds four examples between 70% and 90% and five below 60%. The combined
216-text collection contains 56 human controls and 160 nonhuman texts; the
nonhuman bin counts are 6 below 60%, 2 at 60-70%, 5 at 70-80%, 3 at 80-90%,
none at 90% to below 100%, and 144 at 100%. All human controls score zero.
The second charge estimate is $10.24 with a $12.80 reservation. Total
reservations are $58.30; estimated charges including the two earlier jobs
are $43.76. Actual billing remains unverified.

Before a third cohort, investigate longer existing texts. The 16 nonhuman
texts below 100% in the first two cohorts are 80-308 words long. Their
reported human fractions range from 12.8% to 67.2%; the corresponding
human-labeled windows contain 24-129 words. This suggests that passage
length and segmentation may limit coverage near 90-99%, but it does not
establish a minimum segment size or guarantee a result for longer writing.

The third cohort will contain every previously unannotated Train seed
family whose original draft has at least 400 words. There are 15 such
families before excluding earlier cohorts. Use word counts already emitted
by the Rust planner for the unchanged v5 Train corpus, whose input hash is
`ac0624daad4f1d0f132cc7af85b3e34eaf8c193a741734123ec056dc7e8084aa`.
Freeze the length-based membership and retain each selected seed family's
three original records byte for byte. Validate the resulting ancestry and
rights with the Rust tools before planning its requests. No concatenation,
padding, rewriting or substitution is part of this probe. Report the actual
count, cost and results before deciding on further spending.

The longer-text cohort returned all 42 observations from 14 new families.
Seven of its 28 nonhuman texts score between 66.7% and 100% exclusive,
including three in the previously empty 90-99% band. The third charge
estimate is $8.24 with a $10.30 reservation. This supports collecting more
longer passages, without establishing passage length as the cause.

The [first three cohorts' aggregate](results/pangram-dataset-v2-first-cohorts.json)
contains 258 unique texts from 70 Train families, with 70 human controls,
118 model-only texts and 70 mixed-origin texts. The 188 nonhuman results are:

| Detected content | Texts |
| --- | ---: |
| Below 60% | 6 |
| 60-70% | 3 |
| 70-80% | 6 |
| 80-90% | 5 |
| 90% to below 100% | 3 |
| 100% | 165 |

All human controls score zero. The collection has observations across the
requested range, but remains dominated by 100% results. Every result and
source-family relationship is retained. The three new cohorts cost an
estimated $30.88. Including the two earlier jobs, estimated project charges
total $52 and reservations total $68.60. These remain estimates and spending
reservations, not a statement of the account's billed balance.

Keep $30 of the remaining allowance available for a source-disjoint
comparison with the frozen v6 detector. This is an allocation within the
existing $150 ceiling, not a new paid reservation or additional authorization.
Further Train submissions must leave that headroom; their cumulative ledger
reservations, including prior jobs, may not exceed $120 until that comparison
is planned. Define benchmark membership independently of detector scores and
retain v6's fixed selection/Test procedure. The next Train length group to
consider is the 39 unannotated seed families with 300-399-word drafts, followed
by additional source/style coverage if the remaining budget supports it.

The fourth cohort submitted all 117 seed records from those 39 families. Its
estimate is $18.92, with $23.65 reserved. Cumulative estimates are $70.92 and
reservations $92.25. The provider returned all 117 results successfully, but the
strict collector stopped on one text echo: five source soft hyphens were removed.
Both result pages and the original request are archived. Before admitting this
cohort, validate all responses with a separately recorded, explicit option that
permits only removal of source U+00AD characters and the already allowed
whitespace normalization. Preserve both text hashes and mark such observations
as normalized rather than exact. Other textual changes must still fail.

The user then requested broader generator coverage and model-specific author
labels. [The attribution plan](GENERATOR_ATTRIBUTION.md) directs the remaining
Train allocation toward GPT and Claude as well as the existing local models.
The next 24-source set is frozen independently of its future detector results.
Do not spend that allocation on another expansion limited to the original three
composers. The $30 allocation for a fresh comparison remains available.

The fourth collection now passes the Rust collector with that explicit
normalization policy. It has 13 exact echoes, 103 whitespace-only echoes and
one echo with source soft hyphens removed. No additional request was submitted.
Its annotation SHA256 is
`f907378843135e871e9a596b8936f4f803957a9a1f6a5efb25ab4c4b51a62fcd`.
All 375 Train texts now have validated Pangram 4.0 observations, across 109
source families. The 109 historical human controls all score zero.

| Detected content | Model-only | Mixed | Combined nonhuman |
| --- | ---: | ---: | ---: |
| Below 60% | 2 | 8 | 10 |
| 60-70% | 2 | 2 | 4 |
| 70-80% | 4 | 5 | 9 |
| 80-90% | 5 | 6 | 11 |
| 90% to below 100% | 4 | 5 | 9 |
| 100% | 140 | 83 | 223 |

The [four-cohort report](results/pangram-dataset-v2-four-cohorts.json) retains
the counts, bindings and cost estimates. There are now 33 nonhuman texts in
the requested interior range. The original inputs and their provenance are
available in the [public snapshot](../../datasets/detector-origin-v1/README.md).
Individual provider reports and per-record weighting files remain local.

The prepared weighting view retains all 375 records and full generator ancestry.
It gives each origin equal total weight, then equalizes occupied score bins
within each origin. Its maximum weight is 10.4167 and its weight effective
sample size is 99.9650, computed as squared total weight divided by the sum
of squared weights. That number does not account for within-family correlation.
The sparse bins still need additional examples; weighting them more heavily
does not supply new evidence. No detector has been fitted on these weights,
and the frozen v6 experiment remains unchanged.
