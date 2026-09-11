# Narrative corpus generation

The v5 campaign is in progress. Qwen's cohort is complete; the other declared
providers remain pending. No v5 detector has been fitted or evaluated.
The [target amendment](ENCODER_NARRATIVE_V5_TARGET.md) prioritizes fully
model-written drafts under style steering. Copyedits remain auxiliary examples.

## Completed Qwen cohort

All 578 calls were attempted. There were 559 individually accepted outputs and
19 failures: 14 outputs below the minimum word count, four drafts with long
copied source spans, and one response that reached its token limit. No transport
failure occurred. Raw requests, responses and exclusions remain archived.

The declared complete-pair rule retains 273 of 289 source families, or 819 rows:
one human source, one model draft and one edit per family. It excludes 16 families,
including seven accepted drafts whose paired edits failed and six accepted edits
whose paired drafts failed. These successful siblings remain archived. No
detector scores affected admission, but the coupled rule can change the sample
of fully model-written text available for evaluation.

| Partition | Source families | Retained rows | Maximum tokens |
| --- | ---: | ---: | ---: |
| Train | 192 | 576 | 807 |
| Development | 26 | 78 | 595 |
| Calibration | 33 | 99 | 676 |
| Test, unopened | 22 | 66 | 717 |

Every row fits the 1,024-token contract, including special tokens. Nothing was
truncated or removed by the tokenizer audit. All six profiles remain represented
in each partition. The [cohort report](results/narrative-qwen-cohort-v1.json)
contains the planned and admitted profile counts and exact corpus bindings.
Qwen and Mistral's assignment-plan files are identical.

## Development diagnostics

As each training provider finishes, evaluate immutable v4 on that provider's full
Development cohort. This comparison is declared before the first such prediction.
Use its existing calibration and thresholds, preserve every score, and report
the human/model metric plus each writing profile. These are Development
diagnostics on the completed provider cohort, which may include variants not
selected for the final primary corpus. They do not replace the declared primary
comparison and cannot justify changing v5's frozen fit recipe or Test selection.

No new Pangram calls are required. Test inspection remains limited to corpus
counts, metadata and token lengths until final checkpoint selection.

## V4 on Qwen narrative Development

The [completed Development report](results/narrative-v4-qwen-development-v1.json)
evaluates v4's original weights, temperature and operating thresholds on all
26 admitted Qwen Development families. It includes 26 historical human sources
and 26 fully model-written drafts in the primary binary metric; the 26 edits
remain in auxiliary diagnostics. No v5 weights or Test records were evaluated.

V4 detected all 26 model drafts at both original operating thresholds, while
falsely flagging nine of the 26 human sources (34.6%). The observed human false
positive interval from source-family resampling is 15.4% to 53.8%. Observed false
positives in this cohort exceed both original Calibration targets of 1% and 5%.
Human/model binary log loss is 0.7090 and binary Brier score is 0.2194.

| Writing instruction | Model drafts detected | Human sources falsely flagged |
| --- | ---: | ---: |
| Anti-AI | 4/4 | 3/4 |
| Fix Slop | 4/4 | 0/4 |
| Direct | 5/5 | 2/5 |
| Informal | 5/5 | 2/5 |
| Plain | 4/4 | 2/4 |
| Source matched | 4/4 | 0/4 |

Both original thresholds give the same counts in this table. Different profiles
were assigned to different source families, so the human false-positive counts
do not show a causal effect of an instruction on human prose. Four or five model
drafts per profile also cannot establish reliable resistance to that writing
strategy. In this cohort, the observed failure is excessive human false positives;
the attempted steering did not evade either frozen operating threshold.

The result supplies a baseline for the planned narrative training. It changes
neither the fit recipe nor the unopened Test assignments. A revised detector
must improve human discrimination as well as recognize fully model-written prose.
