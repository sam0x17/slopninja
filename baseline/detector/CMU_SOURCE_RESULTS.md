# CMU narrative-source admission

Measured 2026-09-11 under the [fixed source-review policy](CMU_SOURCE_REVIEW.md).
Seventy-five of the first 100 sampled summaries passed admission. They add a
historical narrative register to the detector corpus; no new detector has been
trained or evaluated on them.

| Outcome | Summaries |
| --- | ---: |
| Admitted | 75 |
| No complete historical match | 16 |
| Long quotation requiring separate review | 7 |
| Additional source-notice terms requiring review | 2 |

All 84 complete historical matches came from the first fixed cutoff,
November 2, 2012. Two contained notice terms (`attribution` and `plagiar`), and
seven of the remaining summaries contained quoted passages of at least eight
whitespace-delimited words. These were excluded under the existing screen.
The notice rule can overexclude ordinary prose and cannot detect every rights
issue. Unresolved revisions and unsuccessful second-cutoff attempts remain in
the archived review; they are not replaced with current text.

The admitted corpus has 75 source families and zero merged duplicate groups:
52 Train, six Development, ten Calibration and seven sealed Test. Summary lengths
range from 84 to 496 lexical words. Each source retains its full TSV text bytes,
historical revision URL, contributor history, collection credit and extraction
notice. Original Wikipedia CC BY-SA 3.0 Unported terms remain explicit alongside
the CMU collection's CC BY-SA 3.0 United States grant. No book-author metadata
was used as summary-author evidence.

The corpus SHA256 is
`78b9798653ab65d697561f91796011e077fb29355b65bca4152df6926d4f08fb`.
The [admission summary](results/cmu-admitted-v1.json) and
[lineage summary](results/cmu-lineage-v1.json) bind the exact source file,
review plan, decisions and executed importers. Source text and captures remain
in ignored `data/baseline-detector/standard-corpora/`.

## Bounded expansion

Before any model generation or scoring, extend the same fixed ordered sample to
400 total source families by reviewing offsets 100, 200 and 300 in three sequential
100-row batches. Keep the same length range, exclusion corpus, cutoffs and
admission rules. Preserve the first 100 observations and all additional failures.
This is a data-yield decision based on source matching, with no detector scores.

Assemble the four reviewed batches once, using the existing deterministic split
seed. Verify that the initial 75 sources keep their partitions. Freeze the full
admitted corpus before assigning generator/profile pairs. An expected yield
near 300 families would keep narrative sources comparable in scale to the v4
training pool; that is a projection, not an observed count or an admission quota.

Recorded generations must request plot-summary prose rather than news reporting,
preserving character relationships, event order, qualifications and intended
tone. Instructions alone will not certify fidelity. A detector recipe and fresh
confirmation policy remain to be declared after corpus yield and token audits.
The opened v4 Test families cannot serve as fresh confirmation for that choice.
No additional Pangram spend is part of this source expansion.
