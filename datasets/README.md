# Published detector datasets

| Snapshot | Texts | JSONL size | Contents |
| --- | ---: | ---: | --- |
| [Reference detector v5](detector-v5/README.md) | 3,384 | 32.88 MB | Frozen Train, Development, Calibration and opened Test corpus |
| [Pangram inputs, snapshot 1](detector-origin-v1/README.md) | 432 | 4.05 MB | Inputs to the initial annotation campaigns, including later model revisions |
| [GPT/Claude captures, snapshot 1](detector-hosted-v1/README.md) | 96 generated + 24 supporting roots | 1.99 MB including audit | Four hosted generators, complete prompts, provenance and source attribution |

These snapshots include exact text, generator provenance, source attribution,
licenses and checksums. They overlap; their row counts are not additive.
Ordinary Git stores these files. Individual Pangram reports and working
invocation logs remain in ignored storage. Hosted captures are published for
research; they have not been admitted for commercial model training.

As of 2026-09-12, the local Pangram collection covers 471 distinct Train texts
from 133 source families, plus 57 Development texts. The hosted snapshot's
manifest records the disjointness check against the earlier Train collection.
Repeated scans do not increase those text counts.

The frozen v6 primary corpus currently contains 5,463 records from 797 families:
3,833 Train, 530 Development, 540 Calibration and 560 Test. Its four JSONL files
total 52,335,071 bytes (52.34 MB), with another 7,988,644 bytes of rights metadata.
V6 remains in ignored storage while fitting and its declared final evaluation
are pending. It overlaps v5 and is not an additional disjoint collection.
