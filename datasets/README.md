# Published detector datasets

| Snapshot | Texts | JSONL size | Contents |
| --- | ---: | ---: | --- |
| [Revision detector v6](detector-v6/README.md) | 5,463 | 52.34 MB | Frozen primary corpus, including repeated model revisions and complete ancestry |
| [Reference detector v5](detector-v5/README.md) | 3,384 | 32.88 MB | Frozen Train, Development, Calibration and opened Test corpus |
| [Pangram inputs, snapshot 1](detector-origin-v1/README.md) | 432 | 4.05 MB | Inputs to the initial annotation campaigns, including later model revisions |
| [GPT/Claude captures, snapshot 1](detector-hosted-v1/README.md) | 96 generated + 24 supporting roots | 1.99 MB including audit | Four hosted generators, complete prompts, provenance and source attribution |
| [Revision inputs, snapshot 1](detector-revisions-v1/README.md) | 110 | 0.94 MB | Fifth annotation cohort, including complete revision chains |

These snapshots include exact text, generator provenance, source attribution,
licenses and checksums. They overlap; their row counts are not additive.
Ordinary Git stores these files. Individual Pangram reports and working
invocation logs remain in ignored storage. Hosted captures are published for
research; they have not been admitted for commercial model training.

As of 2026-09-13, the local Pangram collection covers 581 distinct Train texts
from 155 source families, 57 Development texts from 19 families, and 162 fresh
Test texts from 18 families. That is **800 distinct annotated texts from 192
source families**. The fresh comparison retains its disjointness checks against
the earlier collection. Repeated scans do not increase these counts.

All 162 fresh Pangram requests succeeded. The [collection report](../baseline/detector/results/pangram-revision-v6-collection.json)
records the score histograms and budget. Scores remain concentrated
at 100%: 106 of 108 model-only texts fall in that bin, and only eight nonhuman
texts fall between 60% and 100%. The [completed v5/v6 comparison](../baseline/detector/releases/v0.6.0/RESULTS.md)
includes the fresh Pangram check and both models' frozen operating points.
These Test observations remain outside fitting; individual annotations stay
in ignored storage.

The published v6 primary corpus contains 5,463 records from 797 families:
3,833 Train, 530 Development, 540 Calibration and 560 Test. Its four JSONL files
total 52,335,071 bytes (52.34 MB), with another 7,988,644 bytes of rights metadata.
V6 training, calibration and the declared final evaluation are complete.
Publishing these exact copies did not change the selected model, thresholds
or evaluation. The v6 corpus overlaps v5; it is not an additional disjoint
collection. Published Test inputs must be retired from future hidden benchmarks.
