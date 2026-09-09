# Anchored lexical frames

This study asks whether a lexical frame ledger can retain possible predicate
uses and their observed argument bindings. The preceding boundary comparison
gave the same local structure for ordinary clauses and reporting tails. A
frame resource can add documented possibilities, but it cannot decide which
use a writer intended or supply an automatic edit license.

Use one pinned snapshot of the official VerbNet 3.4 XML resource. Bind its
repository commit, archive, consumed XML files, license and conversion source.
Keep every member's declared class memberships, sense identifiers and inherited
frame alternatives. Parent membership must not imply membership in every child
class. Preserve declarations and inheritance provenance so each effective
frame and role restriction can be traced to the source XML.

## Evidence and uncertainty

The representation starts with unchanged parser Documents. Each observed
predicate keeps its byte span, normalized parser lemma, raw head and relation,
particles, finite/operator evidence, and anchored dependent graph. Every
dependent and its raw relation remains visible, including adjuncts and
unimplemented constructions. A syntactic subject is not assigned an Agent role
until a particular resource frame proposes that assignment.

| Observation | Interpretation |
| --- | --- |
| Supported syntactic binding | The implemented slot and parser tests support a particular assignment of anchored arguments to one frame. |
| Contradicted syntactic binding | A checked field conflicts with that assignment; retain the conflicting evidence. |
| Unresolved syntactic binding | A required observation or mapping is missing or unimplemented. |
| Full compatibility | A separate assessment that also accounts for resource restrictions and other unimplemented requirements. |

A supported syntactic binding may still have unresolved full compatibility.
For example, an unimplemented selectional restriction stays unresolved. POS
does not establish animacy, agency or a semantic type. The matcher must not
convert an unimplemented restriction to false, discard that frame, or ignore
the restriction to call the result complete. A known contradiction and an
unknown field may coexist; retain both instead of allowing missing evidence
to hide a checked mismatch.

Frame syntax, argument order and explicit prepositions can constrain supported
bindings. The implementation must declare its exact supported XML symbols and
parser-label mapping before evaluation. Passive remapping, extraction,
raising/control, omitted arguments, backward discourse complements and unknown
syntax stay unresolved unless the frozen implementation explicitly supports
them. A required slot that is not observed is not thereby optional or filled
by a neighboring clause. Unexpected observed arguments must remain in the
ledger and affect completeness; they cannot disappear as unused parser data.

The initial mapping accepts an active finite `VERB` head with one consistent
finite carrier. A preceding nominal `nsubj` can fill the first NP slot. A
following nominal `dobj` or `obj` can fill an unmarked object slot. An explicit
preposition binds a following `prep`/`pobj` pair, or an `obl` with one preceding
`case` marker. The resource preposition must be one alphabetic literal;
multiword and alternation notation remains unresolved as a whole.

A following finite `ccomp` with its own preceding nominal subject can supply
a proposition slot. The resource's positive `that_comp` restriction requires
an attached `that` marker (`mark` or `complm`), retained with its byte anchor.
A bare complement does not satisfy that restriction. Negative `sentential`
restrictions are checked only for a uniquely observed non-complement binding.
Other restrictions and unsupported XML attributes remain explicit. Bindings
must follow the resource's ordered syntax; multiple unmarked postverbal NP
slots remain unresolved. The parser's head lemma is never silently expanded
into a multiword resource member from particles.

Retain all admissible resource alternatives and their separate bindings. An
unmatched lemma, ambiguous sense or unresolved construction must not collapse
to a uniquely asserted sense. Always retain the possibility of a use missing
from the resource. A supported assignment is neither a grammaticality verdict
nor proof of meaning, attribution, reference, commitment or tonal intent.

## Frozen engineering fixtures

The fixed budget is eight development lemmas selected from four classes,
and twelve different lemmas selected from six other classes. Each
confirmation lemma receives four synthetic texts: two explicit argument uses,
one controlled participant or proposition-binding change, and one incomplete
or ambiguous use. Four additional lemmas receive four texts each while all
their resource entries are masked. The mask simulates missing coverage; it
does not claim those lemmas are absent from VerbNet. The maximum is 32
development texts and 64 confirmation or masked texts. All 96 texts freeze
together and run once. The development label identifies a coverage group;
this version makes no changes from its results before evaluating the other
groups.

Select and bind the resource classes and lemmas before parsing. Read every
inherited alternative for each selected member. Check all memberships before
describing a class as held out: selecting different root classes alone does
not prevent a polysemous lemma from sharing another development class. Retain
any such overlap explicitly or revise the allocation before the fixture
freeze. Previously inspected boundary lemmas are developmental and cannot
silently become new confirmation lemmas.

Each fixture records literal predicate and argument spans, its construction
intent, resource-backed admissible alternatives, expected unresolved fields,
and any explicit byte patches linking a controlled change. Alternatives may
be sets rather than a single frame. An agent reviewer checks the XML support
and exact anchors before outputs. These are assistant-constructed engineering
expectations. No independent human semantic ratings have been obtained, and
the fixtures must not be described as human gold or unambiguous semantic truth.

The initial checklist covers inheritance and polysemy, direct nominal objects,
preposition identity, finite complement bindings where supported, repeated
predicates with different participants, participant/proposition relocation,
missing arguments, unmatched entries and unsupported syntax. Include matched
unknown controls by withholding every class/sense entry of each masked lemma.
Unknown coverage must not be confused with syntactic contradiction.

Use the fixed incumbent parser once per distinct full text in lexical SHA256
order. Preserve parse errors, failed mappings and every alternative. Freeze
the parser assets, resource, matcher, fixtures and allocation before any of
these outcomes are read. Do not select a parser, widen the mapping, repair expected
labels or replace difficult cases after confirmation results.

## What the results can establish

Report all cases by lemma, selected class and fixture category. Separate
resource coverage, supported/contradicted/unresolved syntactic bindings,
unresolved full compatibility, expected alternatives retained or lost, and
exact participant/proposition bindings. Missing and sparse cases stay in the
denominators. Report engineering expectation mismatches and unavailable
expectations separately. Several frames for one predicate are alternatives,
not independent observations.

The useful result would be supported, correctly anchored syntactic evidence
in each selected confirmation class while preserving explicit unknowns. Full
semantic compatibility is not required merely to demonstrate that narrower
capability. Retaining every frame without binding its arguments is insufficient.
A purportedly complete binding with a wrong participant, a controlled binding
change lost by the ledger, or an unknown use converted into a unique asserted
sense falsifies the corresponding claim. Stop and report such failures; do
not broaden the matcher until a predeclared success threshold is reached.

The comparison with existing word and operator counts is descriptive. Equal
counts cannot certify retained associations, and different counts cannot
certify correct meaning. No corpus allocation, model fitting, author-style
evaluation, detector scoring or automatic edit authorization belongs to this
stage. Existing feature families, model weights, rewrite rules and claim guards
remain frozen. Any later count projection or edit gate needs a separate design.

Each lemma also has one controlled source/candidate relation, for 24 exact
patch relations in total. Compare supported role/argument identities and raw
dependent identities separately, excluding source hashes and absolute offsets
from the comparison signatures. Report unavailable lexical bindings separately
from observed equality, including masked cases. A flat proposition-head
identity cannot by itself preserve the proposition's internal participants;
its linked predicate and subtree evidence must remain visible. These comparisons
test retained associations, without supplying semantic-equivalence judgments.
The compact signatures identify tokens by lemma, text and POS, so repeated
identical tokens can still hide differences in occurrence identity. Equal
signatures do not prove equality of arbitrary dependency graphs. The full
ledger retains the original token indices and byte spans for that reason.

## Frozen results

The single run found all 96 designated predicates without parse or observation
errors. It reported at least one complete syntactic frame for 41 targets. The
ledger also includes 16 helper predicates inside the synthetic texts; they are
excluded from the target counts below.

| Fixture group | Targets | Targets with complete syntax in any resource class | Expectations met | Mismatched |
| --- | ---: | ---: | ---: | ---: |
| Development | 32 | 8 | 110 | 22 |
| Confirmation | 48 | 33 | 201 | 23 |
| Masked entries | 16 | 0 | 32 | 0 |
| Total | 96 | 41 | 343 | 45 |

The 388 expectations mix lookup, retention and binding checks. Their aggregate
match rate is not an estimate of semantic accuracy. All 45 mismatches concern
role-binding expectations, across 26 cases. No expectation was unavailable.

| Engineering expectation | Met | Mismatched | Total |
| --- | ---: | ---: | ---: |
| Exact predicate span and lemma | 96 | 0 | 96 |
| Every resource frame retained | 80 | 0 | 80 |
| Specified role and argument binding | 121 | 45 | 166 |
| Specified frame remains syntactically incomplete | 30 | 0 | 30 |
| Masked lookup remains unknown | 16 | 0 | 16 |

The 30 incompleteness checks all require `syntax_binding_complete:false` for
specified frames. Another frame may still complete. Role-binding checks also
have different requirements: some permit a retained partial binding, while
others require the whole specified frame to be syntactically complete. A failed
role check therefore does not necessarily mean the matcher assigned that
argument to a wrong role.

Restricting confirmation coverage to the selected root class and its subclasses
gives 32 complete targets. The additional complete target in the table above is
an `assemble` use under a different resource membership. Each selected class
has at least one explicit or controlled-change case with a complete expected
frame whose specified argument anchors all match.

| Selected confirmation class | Complete targets in that class, of 8 | Explicit/change cases with a complete expected frame and all specified roles matched, of 6 |
| --- | ---: | ---: |
| `admire-31.2` | 4 | 4 |
| `amuse-31.1` | 8 | 6 |
| `build-26.1` | 6 | 6 |
| `chase-51.6` | 5 | 5 |
| `conjecture-29.5` | 3 | 3 |
| `learn-14` | 6 | 6 |

Across all 112 observed predicates, the ledger retained 652 frame alternatives:
70 had checked syntax and 582 had unresolved syntax. All 652 had unresolved
full compatibility. These are repeated alternatives across texts, not 652
independent examples. No frame received a unique sense decision or an automatic
edit license; the masked targets retained unknown lookup status.

All 24 controlled source/candidate relations changed the raw dependent
signature. Complete frame-binding signatures changed for 11 relations and
were unavailable for 13; none was observed unchanged. Fourteen relations
preserved the alphabetic-word occurrence multiset. Within those 14, all raw
signatures changed, seven complete binding signatures changed and seven were
unavailable. This demonstrates retained association differences in those saved
records. It does not establish semantic fidelity, and unavailable bindings are
not counted as successful detections. The repeated templates and frames also
preclude treating these counts as independent accuracy trials.

The result supports continued use of the anchored ledger as inspectable
syntactic evidence. Coverage remains uneven, and neither the engineering
checks nor unresolved resource restrictions establish intended meaning. The
frozen parser, matcher and fixtures remain unchanged after this run. Any repair
needs a separate study with new cases.

## Reproduction

The public [resource manifest](../grammar/resources/verbnet-3.4-manifest-v1.json)
pins VerbNet 3.4 at commit
`ae8e9cfdc2c0d3414b748763612f1a0a34194cc1`. Its SHA256 is
`1051972db94caaca527cabbb69f40102ecb1a7a54a883fe6afca81d648c3bcb4`.
Retrieve the archive and license from its recorded URLs and place the declared
files at the manifest's paths. Import verifies their hashes and preserves the
XML; it does not download a newer resource or invoke NLP.

From the repository root:

```sh
study=data/author-corpora/blog-authorship-2004/lexical-frames-v1
cargo build --manifest-path grammar/Cargo.toml --release \
  --bin slopninja-lexical-frames
grammar/target/release/slopninja-lexical-frames import \
  --repo "$PWD" \
  --manifest grammar/resources/verbnet-3.4-manifest-v1.json \
  --expected-manifest-sha256 1051972db94caaca527cabbb69f40102ecb1a7a54a883fe6afca81d648c3bcb4 \
  --out "$study/import-new.json"
```

The local `freeze-study-v1.sh` binds the parser assets, source and executable,
resource and fixture into a study protocol. Independent review records remain
separate receipts. Run requires that protocol's recorded digest, the exact
bound assets and a fresh output directory:

```sh
grammar/target/release/slopninja-lexical-frames run \
  --repo "$PWD" \
  --protocol "$study/protocol.json" \
  --expected-protocol-sha256 10e7d5c2c2db956bf12e9cd17aa9057c8b1dcab8859aa0784612e8cd4a5b594f \
  --out "$study/run-new"
```

The metadata-only fixture builder and its receipt remain under
`fixture-build-v1`. It reads the pinned XML independently of the production
importer, verifies all selected memberships and inherited frame IDs, and checks
exact source/candidate patches. The separate index comparison agrees on those
memberships for all 24 selected lemmas. The public fixture contains 96 distinct
texts; none is an exact copy of a resource example. These checks include no
parser calls or independent human ratings.

The executed results are retained in `run-v1/report.json`, with SHA256
`ee6c8e5294e3d2d31200d9cf178858d1eeb6f2ec6c639904d7d45afcf6622759`.
The expectation-kind and selected-class tables above are recomputed from the
saved case records in `documentation-counts-v1`. That calculation was written
after results were available; it is descriptive bookkeeping, with no new
parser calls or model execution.

The separate post-outcome audit in `independent-saved-audit-v1` verified the
bound artifacts and all 652 alternatives against the original XML. It checked
all 70 complete frames against the saved token graphs, including 98 complete
role slots with explicit fixture participant assertions, without finding a
conflicting participant binding. It reproduced the 388 expectation results
and 24 pair signatures without rerunning the parser. This checks saved evidence
and engineering assertions; no human semantic ratings were added.

Formatting, all 553 Rust tests across 38 suites, Clippy and the release build
passed. The test run included 20 installed-parser integration tests.
