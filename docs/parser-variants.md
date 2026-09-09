# Parser variants for boundary choices

This study compares the existing small spaCy parser with isolated small,
medium and transformer installations on frozen synthetic boundary pairs and
attribution counterexamples. The preceding [eligibility study](boundary-choice-eligibility.md)
found no usable paired choices under the current parser, rules and claim
guard. This comparison checks where the same unchanged rules and guard behave
differently when supplied with other parser annotations.

The planned variants are `incumbent_sm`, `isolated_sm`, `isolated_md` and
`isolated_trf`, using spaCy 3.8.16 and the corresponding English model 3.8.0
on CPU. Bind the actual Python, installed package versions, model metadata,
parser bridge and executable before execution. An unavailable installation
remains unavailable; do not silently substitute another model or version.
There is no corpus evaluation, fitting, threshold selection or rule change.

## Fixed inputs and the environment control

Reuse all 12 legacy pairs and their 24 directed cases without changing their
texts, UTF-8 patches or expectations. The separately frozen supplementary
panel contains 12 further pairs and 24 directions: six independent-clause
probes and six complement, scope or quotation exclusions. Two pairs insert
or remove a semicolon within a reporting or belief complement. Their
`complement` source label identifies a challenge case; it defines no new
rewrite rule. Keep both panels distinct in results. These examples are
constructed engineering tests; they do not carry human ratings of meaning,
tone or readability.

Each parser receives the same exact complete strings. Deduplicate identical
strings within each variant and retain every fixture-to-parse link. Freeze
the request order and batching before execution. There are no isolated-clause
substitutions or repeated attempts chosen because their results look better.

The isolated small model is an environment control. Compare its canonical
Document fields with the incumbent small model for every shared input,
including token boundaries, sentence segmentation, dependencies, lemmas and
morphology. Formatting differences between saved JSON files do not constitute
annotation differences. Report all differing texts and fields, plus the
resulting differences in rule or guard behavior. If this control differs,
describe the other contrasts as differences between model/environment
configurations. The control does not isolate the cause of such a difference.

## Decisions and denominators

Use the existing uncapped boundary descriptors and unchanged claim guard.
For each directed fixture, distinguish emission of the exact declared target
from any other proposal. On that exact target, retain the actual forward
patches, every exact restoring inverse-rule candidate and both guard reports.
Eligibility requires an exact inverse and `observed_preserved` in the forward
guard and at least one corresponding reverse guard. Preserve intermediate
failures even when another condition already prevents eligibility.

Also evaluate the declared fixture patches directly with the guard whenever
both texts parse, including cases for which the rules emit no exact target.
This separate diagnostic can reveal an unwanted approval that a conservative
proposer currently prevents. Each supplementary exclusion has
`guard_must_reject: true`: a tested `changed` or `unresolved` result meets that
engineering expectation, `observed_preserved` is a mismatch, and a missing
guard remains unavailable. A false flag imposes no guard-pass expectation.
These direct checks cannot supply a missing rule proposal or inverse.

Report both directed cases and their shared undirected pair, including the
complement challenge pairs within their separate category. A pair has two,
one or zero eligible directions, with unavailable directions counted
separately. State whether both directions were evaluated before describing a
pair as fully rejected. The two directions and the four configurations reuse
the same examples and do not constitute independent samples.

Keep each configuration's coverage by panel and frozen example category:
requested and parsed texts, directed cases, exact-target emissions, inverse
restorations, guard outcomes and eligible directions. Report undirected pair
counts alongside them. Every rate names its denominator, and a zero
denominator produces a null rate. Keep expected exclusions, expected
emissions and unspecified expectations separate. An unspecified expectation
cannot become a successful prediction after observing the output.

## Failures and interpretation

Retain each per-text parser error and mark all dependent directions
unavailable. A missing inverse is a tested rule result; a parse failure is
untested downstream behavior. Bound-file corruption, request/order mismatch,
invalid returned identity or transport failure stops the affected attempt
and leaves a failure record. Report incomplete variants without dropping
their requested fixtures. Do not replace missing results with successful
eligibility or with a guard's `unresolved` classification.

Compare excluded cases as well as intended independent-clause examples. More
emissions can expose unwanted edits even when the guard later blocks them.
More final eligible pairs provide evidence about this fixed synthetic
pipeline; they do not establish semantic or discourse equivalence. Accepting
genuine complement or scope counterexamples would undermine a proposed
compatibility improvement. Keep failures visible instead of ranking parsers
solely by acceptance counts.

No parser is adopted from this screen alone. Any later change needs a stated
handling of observed exclusions and environment differences, followed by a
separately frozen eligibility study on the intended data. This stage makes
no author-preference, readability, tonal-intent or detector-performance claim.

## Independent review

Before outcome access, review the source and frozen protocol against these
decisions. A bounded saved-result audit will verify source and fixture hashes,
parse identities and sharing, exact edit restoration, expectation labels,
eligibility conditions and all grouped counts. It will compare the two small
model annotation records without rerunning NLP. Such a consistency audit
does not independently prove that a dependency parse or guard judgment is
correct.

## Results

None of the four configurations supplied a paired eligible boundary choice.
The larger models produced different annotations, but those differences did
not resolve the rule and guard restrictions in this fixed synthetic set. No
parser was adopted and no author-preference model was fitted.

All 48 unique texts parsed successfully under each configuration: 192 saved
annotations and 192 directed case evaluations across the four configurations.
Each configuration evaluated the same 24 undirected pairs, comprising 12
legacy pairs and 12 supplementary pairs. Both directions were available for
every pair; every pair had zero eligible directions.

| Configuration | Exact legacy candidates, 24 directions | Exact supplementary candidates, 24 directions | Restoring inverse candidates | Eligible directions, 48 total | Supplementary rejection violations, 12 assertions |
|---|---:|---:|---:|---:|---:|
| Incumbent small | 5 | 5 | 0 | 0 | 0 |
| Isolated small | 5 | 5 | 0 | 0 | 0 |
| Isolated medium | 5 | 5 | 0 | 0 | 0 |
| Isolated transformer | 5 | 5 | 0 | 0 | 0 |

The same ten period-to-semicolon edits were emitted in every configuration.
Each matched both the declared target text and the exact declared patches.
All ten forward guards returned `changed`, and none of their parsed
counterparts produced any inverse-rule proposal. No reverse guard was tested.
All ten forward guards recognized the licensed join rule, so missing boundary
recognition under the guard's internal candidate limit did not explain these
failures. No semicolon-to-period candidate or complement challenge candidate
was emitted.

The 13 specified legacy no-emission expectations and the 12 specified
supplementary no-emission expectations all held for each configuration. The
other 23 directional expectations were unspecified before parsing. The six
supplementary exclusion pairs also required direct guard rejection in both
directions: all 12 assertions held for every configuration, with no unavailable
checks. These outcomes retain the intended exclusions on this matrix; they do
not establish general guard accuracy.

Direct guard outcomes provide one difference that eligibility counts conceal.
For `The editor said the clerk filed the form.` and
`The editor said; the clerk filed the form.`, both small configurations and the
medium model returned `unresolved` in each direction. The transformer returned
`changed` in each direction. Both outcomes meet the frozen rejection
expectation. Across all 48 directions, the small and medium configurations
therefore returned 46 `changed` and 2 `unresolved` results; the transformer
returned 48 `changed` results. No direct declared-edit guard returned
`observed_preserved`.

### Annotation differences and remaining blockers

The isolated small model reproduced the incumbent's canonical annotations,
including parser identity, on all 48 texts. It also reproduced the recorded
case decisions. The historical legacy comparison passed for all 24 annotations
and all 24 case rows. The environment control therefore found no annotation
or decision difference on these inputs.

A separate post-hoc comparison read the saved annotations, without parsing
again. It checked all parser identities and compared complete Document content
after removing only `parser_identity`:

| Comparison against incumbent small | Identical Document content, out of 48 | Identical token byte spans | Identical sentence records |
|---|---:|---:|---:|
| Isolated small | 48 | 48 | 48 |
| Isolated medium | 43 | 48 | 48 |
| Isolated transformer | 39 | 48 | 46 |

Sentence records include the sentence's root as well as its boundaries. Of the
two transformer differences, only the paragraph-gap case changed token
partition, through sentence membership of its whitespace token. The reporting
complement case changed the sentence root without changing its boundaries.
Other differences included dependency heads, POS, detailed
tags, lemmas and morphology. For example, the medium and transformer models
included `rang` and `runs` as verbs in the supplementary temporal and
conditional probes where the small model did not. The rules still excluded
those scope-sensitive cases.

Every configuration retained the same central failure in all ten emitted
joins: a matched predicate changed from a sentence `ROOT` in the period form
to `ccomp` in the semicolon form. The unchanged split rule rejects `ccomp` and
expects an independent predicate attached through `parataxis` or `conj`.
The guard's licensed boundary exception also distinguishes those independent
relations from complements. Changing model size did not remove this conflict
on the tested alternatives.

There is also a distinct rule restriction. The supplementary pair beginning
`The teacher and the assistant packed the boxes.` had no join proposal under
any configuration. The saved parses contain coordinated subject material;
the current join rule rejects `cc` or `conj` anywhere across the sentence pair,
including noun coordination. That restriction deserves separate treatment
from the predicate attachment failure. The remaining known exclusions and the
previously reported uncertain-capitalization probe stay visible in their
original panels.

These observations support investigating a narrowly specified analysis of
clause boundaries and scope while retaining the existing results as a baseline.
They do not support relabeling every `ccomp` edge, relaxing the guard solely to
increase acceptance, or treating the tested models as equivalent parsers.
Any revised representation or rule needs new version identifiers and a frozen
paired-eligibility evaluation before preference estimation.

### Runtime and reproduction

Each row below is one recorded pass over the same 48 texts, in fixed variant
order on CPU. Parsing time includes interpreter/model startup, annotation
transport and writing the saved annotation files. It is not steady-state
parser throughput, and there were no timing repetitions or order randomization.

| Configuration | Parse and annotation-write seconds | Case-evaluation seconds |
|---|---:|---:|
| Incumbent small | 0.887 | 0.0156 |
| Isolated small | 0.988 | 0.0100 |
| Isolated medium | 1.139 | 0.0107 |
| Isolated transformer | 1.932 | 0.0107 |

The ignored study directory is
`data/author-corpora/blog-authorship-2004/parser-variants-v1/`.
`environment-reproduction.md` describes the separately installed, CPU-validated
environment, verified official model wheels and exact 58-package lock.
`variant-bindings-v1.json` binds all four wrappers, installed model assets and
parser identities. The unchanged incumbent environment remains available.

`run-command.sh` records the exact successful invocation. From the repository
root, use a fresh output directory to reproduce it:

```sh
grammar/target/release/slopninja-parser-variants \
  --protocol data/author-corpora/blog-authorship-2004/parser-variants-v1/protocol.json \
  --expected-protocol-sha256 592927853da397b1ffc1c5a284206c49945c94b9fb1ae4023c674f557d4a5353 \
  --repo "$PWD" \
  --out data/author-corpora/blog-authorship-2004/parser-variants-v1/run-reproduction
```

The report SHA256 is
`e639e405838088e69b81927e5f8df7e2ba1efb0ddaa50f0a907b1ad4e7616fe4`;
the receipt is
`9cacea706b4f56a681cce1ed477c45702eb964aab3398bc23df028c8adc637d0`.
`posthoc-annotations-v1/` contains the separate jq diagnostic, exact command,
input hashes, all annotation differences and saved predicate-transition
summaries. It changed no study result. There were no corpus reads, model fits,
author-proximity scores or detector calls in this comparison.

The independent stored-result consistency audit passed, with its source frozen
before outcome access. Its receipt is in
`independent-consistency-audit-v1/receipt.json`. Workspace formatting and Clippy
checks also passed, alongside 472 successful test invocations across 36 suites;
shared module tests run separately in multiple binaries, so this is not a count
of 472 distinct test definitions. The corresponding logs and counts are in
`final-rust-checks-v1/receipt.json`. These checks validate implementation and
saved-result consistency, without establishing semantic preservation or
detector improvement.
