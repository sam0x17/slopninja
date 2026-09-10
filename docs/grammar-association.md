# Grammar association diagnostic

This diagnostic asks whether the two new joint feature distributions improve
author retrieval beyond their simpler marginals or inventories. It reuses the
same 3,889 TRAIN posts from 300 previously examined authors in three fixed
100-author galleries. These authors have already informed feature development.
The result cannot serve as fresh author confirmation or establish edit fidelity.

The Rust implementation derives categorical views from the hash-bound
`grammar-choices-v1` event records. It reads no new corpus text or annotations,
fits no parameters, and selects no model or seed. All views retain their full
event vocabulary, including events absent from a particular target. Distances
use squared Hellinger distance between raw probability profiles:
`max(0, 1 - sum(sqrt(query_probability) * sqrt(target_probability)))`.

For function words, the baseline is the equal-weight mean of two distances:
the `[lemma-or-null, own-POS]` marginal and the complete nonlemma role tuple
`[own-POS, dependency, head-POS, side]`. The joint mixture gives half its weight
to that baseline and half to the exact `function_word_role_v1` distribution.

Five controls replace the original joint distribution with a shuffled version.
For each fixed seed, lemmas are permuted within each post's own-POS strata.
All other event fields remain fixed. This preserves both integer marginals,
including null lemmas and repeated occurrences. Every post has one deterministic
version per seed, reused as both query and reference. The procedure does not
apply a common permutation across an author's posts.

The fixed seeds are `0`, `17`, `29`, `101`, and `1009`. A compact JSON tuple
containing the versioned permutation domain, seed, and post ID initializes a
per-post xorshift64 stream through SHA256. POS strata use lexical order and
events use source-token order. Reverse Fisher-Yates draws use rejection sampling
over the generator's nonzero state range. The local protocol specifies the byte
order, fallback state, and bounded-integer calculation exactly.

For operators, the baseline is a typed per-head inventory. It retains incoming
dependency, head POS, finite-carrier cardinality, and the operator multiset.
Finite evidence is first attached to its operator using the original ordinal;
the ordinal is then removed. HEAD evidence remains separate, including for
`HEAD_AUX`. Operator items retain role, lemma or null, and their finite evidence.
The inventory discards source order, side, attachment paths, and exact parent
identities. Repeated identical items retain explicit multiplicity. Its denominator
remains one event per original eligible head.

The operator joint mixture gives half its weight to inventory distance and half
to exact `predicate_operator_sequence_v1` distance. It tests sequence and
attachment organization conditional on the per-head inventory. It does not
isolate word order alone or recover semantic negation scope.

Each target averages post probabilities equally within dates, then averages
dates equally. A query's own-author target excludes its entire date; other
authors retain all their TRAIN dates. No view is truncated or renormalized to
shared support. A missing required denominator or an author with fewer than two
dates stops the whole diagnostic, with no replacement or selective exclusion.

All nine variants score the same queries and candidates. Exact distance ties
receive expected random-order top1, top5, and reciprocal-rank credit. Results
average posts within dates, dates within authors, and authors within each panel
or across the pooled 300 authors.

The paired comparisons include joint versus marginals, every shuffled mixture
versus marginals, joint versus the mean of five shuffled per-query metrics, and
operator joint versus inventory. The shuffled mean averages retrieval metrics,
not scores. These controls are descriptive perturbations, not five independent
replications or a method of isolating topic.

Paired intervals resample authors within each fixed panel for 2,000 replicates,
retaining their date/post averages. They use a separate fixed xorshift seed and
the historical modulo draw convention. Sorted replicate indices 49 and 1949
are zero-based. No significance threshold, model adoption, or seed selection
depends on these intervals.

The new code is split across
[`association_projection.rs`](../grammar/crates/grammar-eval/src/association_projection.rs),
[`association_retrieval.rs`](../grammar/crates/grammar-eval/src/association_retrieval.rs),
and [`grammar_association.rs`](../grammar/crates/grammar-eval/src/grammar_association.rs).
The standalone command requires a frozen protocol hash and a new output path:

```sh
cargo run --release --manifest-path grammar/Cargo.toml -p grammar-eval \
  --bin slop_ninja-grammar-association -- \
  --protocol PROTOCOL.json --expected-protocol-sha256 PROTOCOL_SHA256 \
  --repo REPOSITORY_ROOT --out NEW_RUN_DIRECTORY
```

The run retains projection hashes and opportunity counts for every post, full
query/candidate distances for all ten component views, nine sets of exact ranks,
per-author results, paired intervals, executed source copies, and hash receipts.
Failed attempts remain separate from any later corrected run.

The completed run retained every post and author. All 19,445 post/seed controls
preserved both function-word marginals exactly. The original joint counts matched
the prior event audit for both families on all 3,889 posts. The projections used
689,053 function-word events and 248,567 clause heads. No denominator was missing.

The table gives author/date-balanced top1 accuracy in percent. Each panel has
100 candidates; their post counts are 1,224, 1,363, and 1,302 respectively. The
shuffled mean is a summary of five per-query metrics, not another ranked model.

| Variant | Original | Replication | Confirmation | Pooled |
|---|---:|---:|---:|---:|
| Function marginals | 31.313 | 30.477 | 29.804 | 30.531 |
| Function joint mixture | 29.192 | 30.129 | 30.216 | 29.846 |
| Shuffled joint mixture, seed 0 | 26.303 | 24.484 | 24.408 | 25.065 |
| Shuffled joint mixture, seed 17 | 25.895 | 25.596 | 24.426 | 25.305 |
| Shuffled joint mixture, seed 29 | 26.000 | 23.306 | 25.116 | 24.808 |
| Shuffled joint mixture, seed 101 | 25.270 | 24.560 | 24.557 | 24.796 |
| Shuffled joint mixture, seed 1009 | 24.992 | 25.933 | 25.123 | 25.349 |
| Mean of five shuffled metrics | 25.692 | 24.776 | 24.726 | 25.065 |
| Operator inventory | 15.251 | 16.661 | 13.803 | 15.238 |
| Operator joint mixture | 15.251 | 16.744 | 13.803 | 15.266 |

The paired differences below are percentage points. Intervals follow the frozen
descriptive bootstrap; they do not account for earlier feature-development choices
on these authors or multiple comparisons.

| Comparison | Pooled top1 difference | Descriptive 95% interval |
|---|---:|---:|
| Function joint minus marginals | -0.685 | [-1.552, +0.200] |
| Shuffle 0 minus marginals | -5.466 | [-6.559, -4.402] |
| Shuffle 17 minus marginals | -5.226 | [-6.379, -4.015] |
| Shuffle 29 minus marginals | -5.724 | [-6.814, -4.576] |
| Shuffle 101 minus marginals | -5.736 | [-6.909, -4.591] |
| Shuffle 1009 minus marginals | -5.182 | [-6.309, -4.103] |
| Function joint minus shuffled metric mean | +4.781 | [+3.825, +5.752] |
| Operator joint minus inventory | +0.028 | [-0.083, +0.167] |

Natural function-word associations outperform the deliberately disrupted
associations in all three panels. That comparison establishes a useful diagnostic
distinction, but adding the natural joint distribution at the fixed half weight
does not improve on the simpler marginals here. Its pooled top5 accuracy also
falls from 53.174% to 51.920%, and MRR falls from 0.418620 to 0.408990. The result
does not establish that the joint information is useless under another estimator
or weighting; this run does not test those alternatives.

Operator sequence and attachment organization produce almost the same retrieval
result as the inventory: pooled MRR is 0.253280 versus 0.253210. This diagnostic
provides little reason to spend a fresh confirmation cohort on this particular
equal-weight operator mixture. The operator records remain useful for inspecting
local grammatical choices and for the separate synthetic edit checks. Retrieval
scores do not establish meaning preservation.

The run completed once in 5.19 seconds after the release build, with no fitting,
model calls, or detector calls. The local record is
`data/author-corpora/blog-authorship-2004/grammar-association-v1/run-v1/`.
Protocol SHA256 is
`d626ec212ebda062044e38bc6d27bbffff7e4b94462e381d1afea557935ded28`;
report SHA256 is
`600523ed72e31cae82dea6624f970e19fcd73be71e7d6f49056d31e5166f67db`.
The candidate protocol bytes were preserved at approval; a separate `freeze.json`
records the approval and frozen status before retrieval. Corpus events, author
profiles, and per-author results remain in ignored storage.

A separate Rust audit reproduced all 38,890 projection hashes, 19,445 marginal
checks, 35,001 query rankings and their exact tie ranges, balanced summaries,
and eight paired bootstrap comparisons. It also reconstructed a fixed sample of
18,000 full-support distances from 18 queries across all candidates and views;
the largest difference was 4.44e-16. This geometry sample was bounded; the ranking
replay covered every saved query. All 299 workspace tests, formatting, and Clippy
checks passed. The local independent audit and workspace receipts are
`independent-rust-audit-v1/receipt.json` and `final-rust-checks-v1/receipt.json`.
