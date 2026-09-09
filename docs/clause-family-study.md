# Testing compositional clause features

Adding `clause_child_backoff_v1` did not establish improved author retrieval.
On 397 new authors and 799 posts, accuracy changed from **50.0812% to
50.0882%**. The paired gain was **+0.007 percentage points**, with a 95%
interval of **[-0.161, +0.154]**. The predefined superiority criterion failed.

| Measure | Original 14 families | Extended 15 families |
|---|---:|---:|
| Author-macro top-1 | 50.0812% | 50.0882% |
| Author-macro top-5 | 69.3143% | 69.3773% |
| Author-macro reciprocal rank | 0.591260 | 0.591018 |

The [representation audit](grammar-representation-audit.md) found that these
clause-child events recur across authors more often than complete clause
frames. This matched experiment shows why that support audit needed a
retrieval test: the better-supported counts added almost no discrimination
under this fitting procedure. The extension remains available for research;
this result does not justify adopting it as an accuracy improvement.

The experiment used new feature exports and freshly fitted models under a
protocol frozen before annotation, fitting or inspection of held-out outcomes.
All six fits completed. The original models selected epochs 110, 135 and 134;
the extended models selected 108, 133 and 131, in seed order 0, 17 and 29.
All six selected models had validation top-1 of 51.4580%. Fitting took about
23.3 seconds in total on the recorded CPU environment, excluding data
preparation, input validation and final evaluation.

## Matched models

| Property | Original representation | Extended representation |
|---|---:|---:|
| Feature families | 14 | 15 |
| Trainable parameters | 45 | 46 |
| Fitting authors | 300 | 300 |
| Fitting posts | 3,889 | 3,889 |
| Candidates per query | 100 | 100 |
| Fitting updates per seed | 200 | 200 |

Both models retain word counts, word bigrams and the twelve original grammar
families. The extended model adds one family of clause-child events. Both
learn positive family coefficients, a shared monotone spline and a
temperature. These fits use raw family distances. The previous model with
3,557 learned coordinate corrections remains a separate practical reference.

Rust preserves the original fourteen distance arrays and computes the new
categorical family's full squared Hellinger distance. Reference profiles
average event probabilities within each date, then across dates. For a
training query, its own author's reference excludes the entire query date.
Previously unseen events retain their distance contribution.

The two fits share thirty spline knots derived from the original fourteen
families' training distances. They also share the initial coefficients for
those families after temperature scaling. The new family receives a positive
initial coefficient. This prevents a softmax normalization change from
silently reducing all original coefficients at initialization.

Three fixed seeds, 0, 17 and 29, each receive 200 Adam updates at learning
rate 0.03. Within each training panel, the loss gives equal weight to authors,
then dates within authors, then posts within dates. The three panels receive
equal weight. All six fits retain every checkpoint, including epoch zero.

## Held-out authors and decision rule

Two new galleries supply 200 requested validation authors. Four more supply
400 requested test authors. Allocation excludes all 2,300 authors in the
twenty-three earlier full cohorts, including authors later excluded during
parsing. The raw metadata audit found no repeated authors, source posts,
exact or normalized texts, or date groups across the twenty-nine cohorts.
The unchanged eligibility rule excluded one validation author and three test
authors who lacked two usable training dates. The final samples contained
199 validation authors with 337 queries, and 397 test authors with 799 queries.
No missing new-family coverage occurred among the eligible inputs.

Validation uses chronological development posts to select a checkpoint for
each arm and seed. Selection maximizes author-macro top-1, then reciprocal
rank, then prefers the earliest epoch. The test authors' training posts build
their reference profiles; their development outcomes remain unused. All six
model choices froze before test scoring. The sample size remained fixed.

The primary comparison is extended minus original author-macro top-1.
Query metrics are averaged across seeds before author means are calculated.
A paired bootstrap resamples authors within their fixed galleries. The
protocol requires the lower bound of its 95% interval to exceed zero for a
superiority conclusion. The interval describes uncertainty over authors
within these galleries, conditional on their candidate sets. Top-5 and
reciprocal rank are descriptive secondary measures.

Existing parser exclusions apply uniformly to both arms, without repair or
replacement. An unavailable new-family query or required reference fails the
study. Private text, profiles, arrays and fitted models remain in ignored
local data. The preparation program also writes old baseline outcomes as a
side effect; those files are not inspected for this experiment.

The new family's selected softmax coefficient was 0.32% to 0.44% across
seeds. This describes a fitted parameter; its actual score contribution also
depends on distances and the learned spline. The earlier complete-family
ablation still supports retaining grammar in the working model. This result
concerns the additional clause-child family.

## Verification and continuation

The independent audit rebuilt all nine named tensors from bound feature
caches, including date-balanced and held-date profiles. Every original14
distance byte was preserved; reconstructed new-family distances agreed
within 8.7e-16. The reviewer reproduced the thirty shared knots from 5,444,600
training distance values, checked all 1,206 checkpoint hashes and all six
checkpoint choices, and verified their selected training losses.

Final verification reproduced 475,818 scores, all 4,794 complete rankings and
tie groups, author aggregates and the paired bootstrap. The maximum score
discrepancy from an independent spline calculation was 1.54e-12. Eight
focused Python tests and required Rust formatting, tests and Clippy passed.
The figure in local `clause-family-study-v1/figures-v1/` was visually checked;
PNG, SVG, PDF, aggregate CSV and source/report provenance are retained.

An initial export guard incorrectly equated a prepared space's expanded
categorical vocabulary with the original numerical space. The correction
validates each space against its own bound identity and verifies identical
numerical axes separately. The first valid training tensor, failed-attempt
logs and both executed source versions remain available. This correction
changed input validation and provenance, with no distance or profile changes.

The generic `slopninja-score-families` Rust command loads the selected portable
model, maps its named families to a validated tensor and applies the shared
spline and family coefficients. It supports both representations. All 24
model/gallery replays passed: 475,818 candidate scores and 4,794 rankings,
with no ordering or tie discrepancies and maximum numerical error 5.69e-14.
The final workspace checks passed formatting, 184 tests and Clippy.

From `grammar/`, build and inspect the command with:

```sh
cargo build --release -p grammar-eval --bin slopninja-score-families
target/release/slopninja-score-families --help
```

Scoring requires the portable model, named-family export, their expected
SHA-256 hashes and the original reference space. It writes logits and a
bound manifest to a new directory. The local `rust-replay-plan-v1.json`
records the 24 verified combinations. Python is needed for fitting and
spaCy annotation; fitted family scoring runs in Rust.

The next representation experiments should test choices missing from the
current counts, such as function words conditioned on syntactic role and
local auxiliary/negation sequences. These remain hypotheses. The present
result does not establish style/topic separation, useful edits, preservation
of meaning, readability or detector performance. No LLM or detector calls
were made. The existing working model remains unchanged.

Local protocol: `data/author-corpora/blog-authorship-2004/clause-family-study-v1/protocol.json`.
SHA-256: `6ab92408ebd52d46d2981189741ef937863a503ff769d1ad5adf5f310b77dcf5`.
Result SHA-256: `8edaa4d6a00e5e0e3eb919317a60d8166229b83cf69d8840d3d1b95da1975add`.
