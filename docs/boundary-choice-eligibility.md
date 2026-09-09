# Period and semicolon choice eligibility

This study checks whether the existing rules, parser and claim guard can
recognize both forms of a period/semicolon boundary in the same source text.
It does not estimate how an author prefers to write. Before fitting such a
preference, we need to know which observed boundaries admit an exact alternative
and which disappear because of rule, parsing or guard restrictions.

The completed screen found no paired eligible edits. Across all TRAIN posts,
the rules proposed 103 period-to-semicolon joins and no semicolon-to-period
splits. Every join lacked an inverse-rule proposal and received a changed
forward guard result. Structural preference fitting cannot proceed from this
coverage; the saved evidence now supports a parser compatibility comparison.

Use the unchanged 3,889 TRAIN posts from the same 300 authors: 1,224 original,
1,363 replication and 1,302 confirmation posts. The hash-bound
`preference-chronology-v1/metadata-v1/training-posts.json` identifies the source
Documents. DEV, TEST and new authors are outside this stage. All original posts
and authors remain in coverage, including those with no boundary sites.

## Source sites and proposals

A semicolon site is any source parser token whose exact text is `;`.
A period site is an adjacent pair of source parser sentences whose left
sentence ends in a literal `.` token. Do not prefilter sentence pairs by their
whitespace or paragraph gap. These definitions describe observed punctuation
and parser segmentation, not every place where a writer could make a
semantically equivalent choice.

Call `split_independent_semicolon` on semicolon sources and
`join_independent_sentences` on period sources. Use their unchanged descriptor
methods directly, without the search pipeline's candidate limit. Link every
proposal to its raw source site by the punctuation patch's starting byte.
A raw site without a proposal remains in the denominator. Its recorded reason
is that the frozen rule produced no proposal, not an invented explanation of
why the text is ungrammatical or unsuitable.

Each proposal starts from the full original document. Apply its exact expected
UTF-8 bytes independently; do not chain changes at different sites. Multiple
proposals at one site, if present, remain separate opportunities. A source site
can contribute at most one eligible-site count.

## Parse, restore and check both directions

Parse each unique exact full-document counterpart once, in fixed batches of
64. Sort the combined corpus-counterpart and synthetic-form requests by their
lexical SHA256 values. Reusing a hash requires exact text-byte equality.
Keep every opportunity's link to that parse record. Reusing a parse avoids
duplicate work; it does not turn correlated proposals into independent samples.
Do not parse an isolated sentence in place of the full counterpart.

On a successfully parsed counterpart, enumerate its inverse rule without a
candidate cap. A split uses the join rule as its inverse; a join uses the split
rule. Materialize every inverse proposal and retain every one that restores
the complete original source byte for byte. Capitalization and whitespace
must match. Merely constructing the mechanical inverse of a patch does not
show that the inverse rule recognizes the alternate form.

Run the unchanged claim guard forward with the actual forward edits. For each
exact restoring inverse, run it backward with that inverse's actual edits and
the original source Document. A forward opportunity is eligible only if:

1. At least one inverse-rule candidate exactly restores the source.
2. The forward guard returns `observed_preserved`.
3. At least one exact restoring inverse also returns `observed_preserved` in
   the reverse guard.

Retain every matching inverse and every guard result, including unsuccessful
ones. A site is eligible if any linked forward opportunity meets all three
conditions. This is an observed engineering check. The guard still makes no
semantic equivalence, readability, emphasis or tonal-intent certification.

The observer's rule enumeration is uncapped, but the unchanged guard has an
internal limit of 256 candidates when it recognizes licensed boundary edits
across its three boundary rules. Consequently, an enumerated proposal may lack
the guard's boundary exception. Retain `licensed_boundary_rule` presence in
both directions. Its absence alone does not identify the cap as the cause.
The study measures the current complete pipeline, including this limitation.

## Missing data and denominators

A returned per-text error remains attached to every affected opportunity as
`parse_error`. This includes per-document validation errors returned by the
unchanged parser adapter. Do not drop the source post or retry selectively
until it passes. Outer transport, batch-count or ordering failures stop the
attempt, as do corrupt bound source caches or text/parser identity mismatches
in a returned successful Document. Retain failure artifacts. A missing guard
result is untested, not a successful guard or a guard that returned `unresolved`.

Report distinct units separately:

| Unit | What it counts |
|---|---|
| Source catalog | All fixed authors, dates and posts, including zero-site rows |
| Raw site | Each declared observed semicolon token or period sentence pair |
| Forward opportunity | Each original-rule proposal linked to a raw site |
| Parse request | Each unique full-document counterpart |
| Inverse candidate | Each proposal from the declared inverse rule |
| Exact restoration | Each inverse candidate that reproduces the whole source |
| Eligible opportunity | A forward proposal meeting the paired checks |
| Eligible site | A raw source site with at least one eligible opportunity |

For each source form, report proposal-site coverage and eligible sites over
all raw sites. Attach explicit denominators to rates conditional on receiving
a proposal or a successful parse. Never present a rate among accepted proposals
as coverage of all source boundaries. With no raw sites, the corresponding
rate is null and its count is zero.

Keep no-proposal sites, per-text parse errors, missing exact inverses and both
guard outcome distributions. Some findings overlap: a candidate can have a
forward guard failure as well as no restoring inverse. Report these facts
separately, with the number actually tested. Also distinguish multiple inverse
matches from multiple forward proposals and shared parse requests.

Summaries separate source form and corpus panel, with pooled counts alongside
them. Retain every original post, author/date group and author, with both
source forms and explicit zero counters. Report how many authors and dates
contain sites, proposals or eligible choices. Rates attach their raw or
conditional denominators and are null when those denominators are zero.
This screen does not compute an author-balanced rate or confidence interval,
fit a preference model, or select a threshold or adoption decision.

## Asymmetric availability and interpretation

Observed semicolons and observed period sentence pairs are different source
populations. Their capitalization, syntax, document context and paragraph
structure need not match. The parser, forward rule, exact inverse requirement
and guard can reject the two forms at different rates. Requiring both
directions narrows the catalog to choices recognized by this pipeline; it does
not show that missing choices are random or that the remaining forms provide
an unbiased measure of author preference.

Eligible counts therefore describe this restricted catalog. They are not
evidence that a writer chose a semicolon despite having an equally suitable
period alternative, or an estimate of how often the writer should prefer one.
Source form, rejection and support remain available for any later version
that studies preferences with a separately frozen evaluation design.

The synthetic panel contains 12 pairs and 24 directed cases. Bind both forms,
inverse case identities and exact UTF-8 patches before corpus outcomes.
Known rule expectations are boolean; new parse-dependent probes have null
expectations. Guard results and paired eligibility are observations without
preasserted success labels. Keep mismatches and unknown expectations distinct,
and report the panel separately from TRAIN counts. Do not alter the existing
rules, parser or guard to make an expectation pass. Source texts, derived counterparts, annotations,
identifiers and per-author records stay in ignored
`data/author-corpora/blog-authorship-2004/boundary-choice-eligibility-v1/`.
The public report uses aggregate counts and synthetic examples only.

## Results

The frozen pipeline found **no paired eligible choices**. There is no support
for fitting period/semicolon author preferences from this catalog. Parser and
rule compatibility need a separate versioned investigation first.

All 3,889 posts and 300 authors remained in coverage. The parser processed 127
unique texts, comprising 103 corpus counterparts and the 24 synthetic forms,
in two batches with no per-text errors.

| TRAIN panel | Raw period sites | Period sites with proposals | Raw semicolon sites | Semicolon sites with proposals | Eligible sites, either form |
|---|---:|---:|---:|---:|---:|
| Original | 10,291 | 33 | 1,855 | 0 | 0 |
| Replication | 13,843 | 37 | 1,333 | 0 | 0 |
| Confirmation | 13,089 | 33 | 1,471 | 0 | 0 |
| Pooled | 37,223 | 103 | 4,659 | 0 | 0 |

Period proposals covered 103 / 37,223 raw sites (0.277%), in 80 posts and 80
author/date groups from 59 authors. Period sites occurred in 2,924 posts,
2,567 author/date groups and 288 authors. Semicolon sites occurred in 637
posts, 572 author/date groups and 198 authors; none produced a split proposal.
The coverage files retain all 3,375 author/date groups, including zero-site
groups.

Each of the 103 corpus proposals parsed successfully. Every forward guard
returned `changed`, and every counterpart produced zero inverse-rule
proposals. These are overlapping failures on the same 103 opportunities.
There were no exact restoring inverses, so no reverse guards were tested.
Eligibility among period proposals was 0 / 103; the corresponding conditional
rate for semicolon proposals is null because there were no proposals. Eligible
raw-site coverage was zero for both forms. All 103 forward guards recognized
the licensed boundary rule, so missing recognition under the guard's candidate
limit does not explain these failures.

### Synthetic directions

The table covers all 12 pairs, with each direction tested once. A `Yes` below
means the exact declared candidate was emitted by the forward rule. Neither
direction of any pair was eligible.

| Pair | Period to semicolon emitted | Semicolon to period emitted |
|---|---|---|
| Transitive clauses, second subject `I` | Yes | No |
| Ordinary intransitive clauses | Yes | No |
| Copular `be` clauses | Yes | No |
| Proper subjects, including `The Hague` | Yes | No |
| Repeated verb with distinct subjects and objects | Yes | No |
| Reused `reads records` / `Users check results` probe | No | No |
| Negation exclusion | No | No |
| Modal exclusion | No | No |
| Shared-time exclusion | No | No |
| Prepositional exclusion | No | No |
| Quotation exclusion | No | No |
| Paragraph-gap exclusion | No | No |

All 13 definite no-proposal expectations held: both directions of the six
exclusion pairs, plus the previously observed semicolon failure in the reused
probe. The other 11 expectations were deliberately unspecified before parsing;
they are observations, not passes or failures against a success target. The
five emitted period candidates all had `changed` forward guards and zero
inverse proposals. The harness also evaluated the declared edits for all 24
synthetic directions, including edits the rules did not emit: all 24 guards
returned `changed` for content/scope while preserving their observed local
operators. Those diagnostic comparisons do not supply missing rule proposals
or reverse eligibility checks.

### What failed

For the synthetic pair `The server stopped. The worker waited.` and
`The server stopped; the worker waited.`, the saved period parse makes both
verbs roots of separate sentences. In the semicolon parse, `stopped` becomes
a `ccomp` dependent of `waited`, which remains the root. The subjects remain
attached to their respective verbs. All five emitted synthetic joins show
this first-predicate `ROOT` to `ccomp` change. The copular example also reaches
this failure: its period form was accepted, so a missing finite predicate was
not the obstacle in that case.

The unchanged [split rule](../grammar/crates/grammar-core/src/rules.rs)
rejects `ccomp` in `plain_scope` and expects a following independent predicate
attached through `parataxis` or `conj`. The
[claim guard](../grammar/crates/grammar-eval/src/claim_guard.rs) recognizes
restricted independent-clause boundary changes involving those relations;
it does not treat `ccomp` as interchangeable with a sentence root. This
accounts for the observed synthetic asymmetry without establishing whether
the declared edit preserves discourse meaning or emphasis.

A post-hoc inspection of saved corpus guard records found a matched-predicate
`ROOT` to `ccomp` transition in 100 / 103 proposals. One of those also had a
`ROOT` to `conj` transition. Three proposals had no such matched-predicate
dependency change, so this is a common failure pattern rather than a complete
explanation of every rejection. The inspection added no parsing and changed
no eligibility result. It does not explain why every raw semicolon site lacked
a proposal; the original rules expose no per-site rejection reason.

First compare pinned [spaCy English pipelines](https://spacy.io/models/en)
on the same examples, keeping the intended exclusions fixed. Any proposed
parser or boundary-analysis change then needs a separate set of independent
clauses and genuine complement, scope and quotation counterexamples.
Simply accepting every `ccomp` change would discard a check
that also protects real grammatical distinctions. Preserve this zero-support
result as the baseline, and require a new paired-eligibility audit before
estimating author preferences. No preference, readability, tonal-intent or
detector conclusion follows from the current failures.

## Reproduction

The ignored study directory is
`data/author-corpora/blog-authorship-2004/boundary-choice-eligibility-v1/`.
Its `run-command.sh` records the exact invocation. From the repository root,
the equivalent command below requires a fresh output directory:

```sh
grammar/target/release/slopninja-boundary-eligibility \
  --protocol data/author-corpora/blog-authorship-2004/boundary-choice-eligibility-v1/protocol.json \
  --expected-protocol-sha256 a2e4575bf565ec3f5a0b6a4aceb4f0382bca5ee84f2e783bbf34429fa9577b0a \
  --repo "$PWD" \
  --out data/author-corpora/blog-authorship-2004/boundary-choice-eligibility-v1/run-reproduction
```

The saved report SHA256 is
`37c43284da313fffa56f747d0e7d7b572428b202ded74b3fa059b2273b6f1d75`;
`synthetic.json` is
`bc4245f720199fefeed248ac9bfd60eb44c51c65dc66a6f7abdc85fe29722654`.
The receipt binds source versions, the executable and the report; the report
binds the annotations. The run took 10.11 seconds
and made no LLM or detector calls, fit no neural model and estimated no author
preferences. `posthoc-parser-diagnostic.json` records the later inspection of
saved predicate transitions separately from the frozen report.

The independent consistency audit checked all 3,889 source bindings, 41,882 raw
sites, 103 opportunities, 24 synthetic rows, 3,997 retained artifact hashes and
127 parse artifacts. Every post/date/author/panel/pooled counter and rate matched,
including null denominators. This was a saved-result consistency check, with
no parser, rule or guard replay. Its source and binary were frozen before
boundary outcomes were read. The audit receipt SHA256 is
`389d69a85b8a03326120fb32dadf8d45cb6aaebfff8a86b7e7610316a1a28be1`.

The Rust workspace passed formatting, 437 test invocations (including 17
installed-parser invocations) and Clippy with warnings denied. Shared module
tests count once per executable target. These checks verify the implementation
and its declared failures; they do not turn zero eligibility into a usable
structural editing model.
