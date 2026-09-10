# Pangram benchmark, verification and API funding

Design decision, 2026-09-10. Pangram is the permanent external quality anchor
for A's origin detection and the required external benchmark for B's evasion.
A artifacts are public and immutable; B generation remains
private. Validators execute a common three-artifact subnet origin panel,
including the strongest qualified artifact from the previous completed A round.
Pangram cannot replace author-specific A evaluation or preservation review.

In each A scoring epoch, compare all admitted A artifacts with Pangram on the
same fresh, independently labeled hidden origin set. The operator funds the
shared provider observations; this API cost does not multiply with the number
of A artifacts. Freeze the reference set, provider version, repeat policy,
operating points, analysis and budget before evaluation. Publish the performance
gaps, uncertainty and coverage, including missing observations. Missing provider
evidence cannot support a current parity finding. Matching or beating Pangram
does not retire A's continuing external quality comparison.

Every B benchmark jointly measures author-style matching and evasion against
Pangram and the subnet panel. The launch schedule is one joint B round per
scoring epoch, with three candidate reports paid by the submitting miner and
a shared three-report source baseline paid by the operator. Reserve cohort,
comparison coverage, report budget, deadlines and validator capacity before
issue; epoch throughput remains to be measured. Private customer jobs remain
outside this disclosure and scoring process.

The [whitepaper cadence and parity policy](../whitepaper/sections/08-pangram-audits.tex)
requires operator-funded, independently labeled comparisons on held-out authors,
production histories and fresh B revisions before reducing B's per-candidate
external checks.
Freeze the A champion and proposed panel policy, human false-positive limits,
noninferiority margins and analysis plan before opening evaluation data.
Confirm on successive unreleased sets; agreement with Pangram is insufficient.
Launch scoring retains Pangram even after parity. A later policy must specify
the reduced B interval, periodic B checks, unchecked-candidate scoring and drift
response before activation. It must preserve A's continuing Pangram quality
comparison. Old reports cannot certify new text.

## What we measure


For origin detection, compare miners and Pangram against the same independently
documented production histories. Report calibration and human false-positive
rates alongside detection performance. Agreement with Pangram is not ground
truth and earns no separate reward. Author identification retains its own known
author labels because Pangram is not our author-attribution reference.
The fixed public probabilistic baseline remains A's reward normalization
reference. Pangram's content fractions cannot directly replace those
probability vectors. Reuse B reports only when their exact text and observations
satisfy the independent origin-evaluation protocol; selected successful
evasions alone cannot replace the labeled reference set.

For evasion, use the explicit `pangram-4` selector and retain the returned
version, exact submitted text, task IDs and complete results. Measure
`fraction_ai + fraction_ai_assisted`; the project target is strictly below
`0.10`, after preservation and quality review. Record the unchanged source
under the same round policy. Improvement before crossing the threshold is
useful development feedback; it must not be reported as a successful sub-10%
revision. Every B revision must also satisfy full meaning preservation,
readability and target tonal intent, with mandatory author-fit measurement.

The two launch tasks are A detection and B transformation. The deterministic
comparison schedule includes the strongest qualified prior-round A artifact,
hash-selects two other distinct qualified A UIDs, then hash-selects B entries
outside that panel. Qualify, freeze and cache the complete A artifacts before
B task disclosure or generation. Reserves apply only before issue. Require
distinct A UIDs and inference-content hashes, excluding owner/signature metadata;
an identical hash gets one credit entry and panel seat, selected by earliest
finalized accepted commitment and canonical UID tie-break. Near copies remain
an evaluation problem. Reserve full validator replay capacity before issue.
Every B entry receives the same panel, sources, briefs, authorized target-author
references, style calibration and frozen settings. Only exact UID exclusion is asserted; common
ownership remains possible. The [mechanism proposal](subnet-mechanisms.md)
defines persistent counters, domains, roster commitments and reserved capacity.

Keep Pangram's fraction `F` distinct from each artifact's document-origin
probability `H_j = p_j(model-only) + p_j(mixed)`. Use
`D_P = min_r F_r(source) - max_r F_r(revision)` and
`D_S = median_j(H_j(source) - H_j(revision))` over the three validator-executed A artifacts.
Evasion utility is `(D_P + D_S)/2` only when both differences are nonnegative,
otherwise zero. Strict detector success requires the semantic gate, Pangram below
`0.10`, a majority of panel artifacts below their own calibrated thresholds,
and a pass against the actual strongest artifact. Report its result separately;
it has one vote in the median, without a unilateral incremental-utility veto.
The sole B utility combines this detector component with mandatory author-fit
improvement. Raw regression in author fit, Pangram or the aggregate subnet
detector result sets the whole utility to zero under the
[reward rules](../whitepaper/sections/07-rewards.tex). A strict pass also requires
those nonregression checks; incremental utility does not establish absolute
author-fit success or a strict detector pass.

After issue, no model or threshold substitution is allowed. A validator-node
outage can use another approved runner executing the same artifact. An actual
reference-execution failure leaves the common comparison unresolved or void
under the fixed closure rule; it grants no B nonresponse or automatic pass.
An A endpoint has no authoritative response or fallback role. Three panel
entries limit one arbitrary outlier only under a bound of fewer than half
colluding, which public artifacts and hashes cannot establish. Keep targeted
collusion tests: validators can faithfully reproduce a deliberately permissive
model. Standard A/B native pools remain; experimental shared settlement is not adopted.

Authorized retired revisions can feed later training, with source families
excluded from private evaluation and production histories retained. Customer
text never enters this benchmark exchange automatically.

Pangram documents asynchronous task IDs and bulk item/result bindings. New
requests should name a model explicitly. A selector or returned version is the
identity the provider exposes, not cryptographic proof of immutable weights.
Version changes or unexplained control drift require a new benchmark epoch.
[Bulk API](https://docs.pangram.com/api-reference/bulk-api).

## The remaining trust

Pangram controls its model, availability and billing. Even perfect response
authentication would prove what Pangram returned, not independently reproduce
its proprietary computation or establish human authorship. Human or model
quality judges introduce another distinct trust assumption.

The public API documentation reviewed here describes API-key authentication and
JSON responses. I found no documented provider-signed result receipt. Treat such
receipts as a requested integration, not an existing capability. Ordinary HTTPS
protects a caller's session, but a copied JSON file, hash or validator signature
does not let another participant verify that Pangram produced the result.
[API documentation](https://docs.pangram.com/api-reference/introduction).

The first supported API feature to use is `public_dashboard_link: true`.
Our [public-result probe](pangram-public-results.md) retrieved the resulting
report and its complete analyzed text, fractions, version and timestamp without
credentials. Validators can fetch that stored observation without purchasing
another inference. The public web JSON endpoint remains an undocumented
implementation detail; obtain stable integration terms before deployment.
The report has no verified signature or immutability guarantee, and its version
does not identify immutable weights. Deliver the report locator and text in
separate encrypted evidence envelopes to every validator in the round's frozen
certifying set; publish the salted commitment and complete ciphertext under
the [service-evidence contract](service-evidence.md). The assignment is fixed
before task disclosure. Anyone obtaining the decrypted URL can still
read the hosted report, so this does not create provider-side access controls.

Prefer provider-signed receipts if Pangram offers them. A signed receipt should bind
the request body hash, model selector, task ID, response hash, returned version,
time and billable units. Verification would then be cheap for every validator,
while the subnet would still trust Pangram as the external benchmark provider.

Another possible route is authenticated TLS transcripts. TLSNotary is
implemented in Rust and supports proving server responses while hiding secrets
such as API keys. Its documented notarized mode requires trust in the notary;
multiple independent notaries can reduce collusion risk. It currently documents
TLS 1.2 support. An ignored local-notary prototype proved a GET of Pangram's
models list and produced a 5,513-byte presentation. This checks one transport
exchange, with important disclosure and trust limits described in the
[probe report](pangram-public-results.md#tls-fallback-probe). A complete
submission/result proof and independent-notary costs remain unmeasured.
[TLSNotary protocol and assumptions](https://tlsnotary.org/docs/intro/).

A local connection check on 2026-09-09 negotiated TLS 1.2 with a verified
certificate at `text.external-api.pangram.com`; an unauthenticated request
returned 401. This establishes that the endpoint accepted TLS 1.2 in that
check. It does not test TLSNotary, disclose a key or submit a detection task.

An acceptable proof must cover the real API hostname and certificate, exact
request text and model, accepted task ID, and that task's completed result.
For an asynchronous API, proving only a final status response is insufficient
unless it is securely linked to the submitted request. The proof must expose
all reward-relevant fields while hiding credentials. Bind its session to a
previously committed assignment so an old favorable result cannot be replayed.
Do not append a challenge nonce to the prose and thereby change the benchmark.

## Initial measurement protocol

1. Freeze the epoch roster, global quotas, common panel/batch schedule, source
   baselines, target references, style evaluator/calibration, settings and fixed
   deadlines. Qualify and cache complete immutable
   A artifacts, including all inference dependencies and reference-execution
   rules, before B generation. Before task disclosure, exclude the
   exact participating A/B UIDs and freeze the complete remaining validator set
   and its native effective-consensus-stake integers, with sum `W`, at a verified
   finalized snapshot. Bind runtime/storage schema, identities, generations and
   encryption keys. A relayer assertion alone is insufficient. Reserve the
   complete validator-executed scoring matrix, ciphertext publication, preparer
   work, and all-validator inspection separately from training/service tickets.
2. Each B miner commits one final revision, its assigned A artifact hashes,
   reference settings and seed schedule, full candidate origin-probability
   vectors, required source baselines, scalar projections and three selected
   Pangram reports per mandated text. No easier model, omitted panel cell or
   score from another text or seed is permitted. Other private attempts need not be disclosed; committed
   reports cannot be replaced afterward. Panel and preparer assignments are
   predictable from the committed schedule, not hidden until commitment.
3. Validators execute the same cached A panel after B commitments. Bind each
   artifact's content hash, reference runner/settings, text commitment and
   complete origin probabilities. Independently compare the canonical outputs
   and projections with B's committed self-scores. A hash, signature or claimed
   score cannot replace those forward passes or reduce their compute budget.
   B generator replay is not required; preservation `G`, public style `V` and
   Pangram evidence remain separate. Mix human/model controls and supply no
   undeclared UID, hidden-label or source/candidate-role inputs to the isolated
   runner. Private search results cannot replace this matrix. Commit replay
   evidence by its fixed block deadline.
4. Three hash-assigned preparers organize every scored submission's required
   checks, with fixed reserves; they have no final approval authority. No
   coordinator chooses preparers and no custom beacon or audit sample is used.
   Publish exact text, report locators, openings and panel evidence encrypted
   separately to every frozen certifier. Curator authorization covers that
   entire set. Follow the DATA-first, separately encrypted generation-witness
   procedure in [service-evidence-v1](service-evidence.md#data-first-confidential-witness-second),
   retaining complete ciphertext bytes and finalized inclusion proofs.
   Preparers commit before seeing peer openings and open privately to certifiers.
5. Every signing validator must inspect the actual evidence, reproduce the
   required DATA envelopes from their confidential witnesses, verify Pangram
   records through GETs, reproduce the common A panel with its exact execution
   bindings, and review whole-revision
   preservation. A preparer signature cannot replace this inspection. No fresh
   validator-paid Pangram inference is mandatory. Retain disagreements and
   certify the exact validity/quality verdict using distinct signer weight `w`
   with strict `3w>2W`. Assume dishonest weight below `W/3` in the conditional
   frozen set; missing keys, nonresponse, recusal and later role changes never
   reduce `W`. Use the [fixed closure rule](service-evidence.md#stage-transitions-and-certificates)
   for missing evidence or no quorum; neither grants approval.
   Keep hidden labels, other miners' candidates, unreleased sources and
   preservation adjudication restricted until retirement. B can execute public
   A locally on text it possesses, including its own candidates.
6. Publish commitments, ciphertexts, certificate verdicts and reviewed
   aggregates. Retain encrypted evidence and disputes. Apply the settled
   service-failure and unresolved gates before earned weights; bind the native
   settlement cycle to the closed measurement rather than assuming a one-epoch
   reveal delay. Private customer jobs stay outside this process. A customer's
   optional evidence packet to one chosen validator has its own limited scope
   and cannot authorize disclosure to the full benchmark set.

The existing offline contract requires three distinct observations for a passing
full-document result. Keep that policy explicit in cost estimates. Selection
from undisclosed private attempts can bias the committed set toward favorable
results, so a pass describes those observations rather than fresh-query success
probability. Three matching calls are not three independent proofs of a text's
origin. Disagreement between calls is not by itself evidence of validator fraud;
model variability, provider errors and version drift must remain distinguishable
from a forged record. Assign consequences only for objectively provable protocol violations
under a published rule, without assuming Bittensor supplies custom slashing.

The [receipt and audit proposal](subnet-receipts-and-audits.md) specifies
predictable preparer assignment, full review, and strict global weighted
certification. A certificate attests the specified verdict under the validator
honesty assumption; it does not prove Pangram's private computation or private
B generation. A's declared computation is directly replayed, with accuracy and
targeted behavior evaluated separately. Native Bittensor weight
commit-reveal remains a separate supported chain mechanism.
The current offline contract has not yet been adapted to public report IDs.

### Reproducibility spot checks

The [repeatability probe](pangram-repeatability.md) covered three texts, with
three fresh requests per text through one account. Document class fractions
matched within each text while auxiliary scores varied. The two selected
intermediate-fraction texts also matched their older document fractions;
same-task GET rechecks on the first text matched its original result. The
nearest observed fraction was 14.80%, with no observation within two percentage
points of 10%. This does not establish determinism near the reward boundary or
cross-account reproducibility. A verifier must keep these two checks separate:

- A GET of the original task retrieves that observation. Authenticated evidence
  can establish what the provider returned for the bound task; repeated GETs
  do not satisfy the distinct-observation requirement.
- A fresh POST under an independently operated auditor's own account measures
  agreement on the same request and adds another billable evaluation. It does
  not authenticate the original evaluator's record or prove that evaluator
  incurred a charge.

Before each round, freeze the API endpoint, exact request-body bytes and their
private hash, explicit model selector, required returned version, comparison fields,
and evaluation window. Preserve exact text bytes and bind assignment nonces
outside the prose. After measurement, the evaluator commits its own complete
response hash, task ID and score-payload hash inside a salted commitment by
the fixed evidence deadline. Review assignments follow the published hash schedule.
Deliver openings and evidence encrypted to every frozen certifier. Keep every accepted
task, failure and unresolved request for separately funded research in its
budget ledger; this does not require miners to disclose their private
pre-commitment attempts.

Compare a versioned projection of reward-relevant fields, including successful
completion, returned version and all three document fractions. Retain task IDs
and timestamps for provenance, while excluding them from cross-task score
equality. Auxiliary scores need their own declared comparison policy if used.
Any cross-task agreement tolerance would need separate measurements; this small
probe supplies none. Such research is optional and does not determine whether
the committed reports pass. An agreement tolerance never changes the strict
`fraction_ai + fraction_ai_assisted < 0.10` reward gate.

A suggested absolute tolerance of `0.02` (two percentage points) remains an
unvalidated candidate. For example, `9.5%` and `11%` differ by less than two
points, but the `11%` observation still fails the strict reward threshold.
Agreement and reward eligibility are separate decisions.

A wrong text, report identity, required version or time-window binding opens a
bounded dispute under the published policy. A different score from a fresh
request does not invalidate an otherwise valid committed report or establish
fraud. Preserve provider failures and changed or unavailable committed records
for resolution; do not replace them with a favorable new request after
commitment. Authenticated evidence of the original response remains the stronger
route to response provenance, with Pangram still trusted for its computation.

If a separate repeatability study samples `N` committed measurements at fraction
`q` with `r` fresh requests each, budget about `q * N * r` additional evaluations
at the applicable text-length rate. This is optional research, not a required
validation expense. Agreement cannot prove an immutable model or independently
reproduce the provider's proprietary computation.

## Who pays

Mandatory training-service tickets run only from A requesters to B providers
for authorized rewrites. They are free to requesters within a fixed global
budget; B providers bear that cost for emission eligibility. A publishes
artifacts and has no mandatory inference endpoint. Endpoint service does not
qualify the artifact; separately offered A paid hosting uses customer terms.
Validators bear authoritative A benchmark execution costs and reserve the
complete panel and review workload separately. B can run public A locally at
its own compute cost without a Pangram charge. Local A feedback does not replace
required external reports. Expected emissions do not guarantee cost recovery.

Training-service tickets use the next persistent A-requester-UID-slot index and
mandatory hash-ranked B providers with fixed fallbacks. The rank hashes domain,
chain, subnet,
requester UID, index and peer UID; the pinned roster root is excluded from the
rank. Scheduling metadata is inspectable, while text and salted openings stay
restricted. No nomination, skipped index, reroll or retry-created quota is
permitted. The [mechanism proposal](subnet-mechanisms.md) defines the shared
ledger, bounded sizes, capacity reservations and fallback response windows.

Obligations bind the UID and its actual mandatory role: A requester publication at
issue, or B provider response after timely global acceptance of the input at the
fixed start. Retain activated `A=S+F+U+V` for successes, attributable failures,
unresolved and platform-void obligations; inactive fallbacks remain reservations.
Set settled `N=S+F` and require `U=0`, `N>0`, and `10F<=N`. More than 10% failures
zeros earned A+B epoch credit; exactly 10% passes the failure-rate test,
and an actually assigned class with no settled work receives no automatic
service pass. Require the
aggregate gate and separate gates on each actually assigned A-requester or
B-provider class. A-provider and B-requester detector-call classes do not exist
and incur no zero-work penalty. Paid hosting replies or outgoing payloads cannot dilute provider
failures. Abandoned requester obligations count against the requester; a
provider lacking accepted input never activates. A successful fallback cannot
erase an earlier provider failure. Count duplicates once and require the full
publication, validity-certificate and dispute evidence specified in
[service-evidence-v1](service-evidence.md#deterministic-accounting-and-platform-failure).
No recipient accusation proves failure. The fixed global platform-incident
rule cannot selectively pardon miners, extend deadlines, or shrink `W`. Prior
actual request records determine the frozen next roster, with bounded real
probation work and no heartbeats. Close measurement before weight deadlines;
native payout lag prevents retroactive removal of already paid emissions.
Actual A-requester/B-provider class recovery also requires zero deficit under
`D_e = max(0, D_previous + 10*F_e - N_e)`, initially zero. Only same-class
protocol probation work reduces it; there is no age-out, other-class dilution,
or reset by key/service-version rotation. Unresolved and platform-void records
cannot retire the deficit; `U>0` blocks the class. Current gates still apply.
Private customer jobs enter none of these mandatory counts. B downloads and
executes public A locally without an A inference request; artifact distribution
creates no detector-call obligation. Authorized retired B outputs can feed A
training under the source-release rules.

Miners pay for private experimentation and the three selected Pangram candidate
reports committed before evaluation. Validators retrieve existing Pangram
reports without buying another inference under the behavior observed in our
probe. The subnet funds challenge baselines through a capped validation
allowance. Fresh reproducibility research needs separate
authorization and funding; the initial scoring policy does not require it.
Bootstrap that allowance from an explicit project budget; later fund it from service fees
and a published operating allocation. These are proposed funding sources, not
an automatic entitlement to a fraction of chain emissions.

For separately authorized research measurements, validators purchase their own
credits.
Reimburse scheduled work at a published rate and cap, using task records and
independent overlap checks.
Such reimbursement still relies on the initial measurement trust model. With
provider receipts or validated transcript proofs, reimbursement can depend on
publicly verifiable work instead. A miner-supplied invoice or claimed query count
is never sufficient. Do not expose a shared master API key to miners or route
all correctness decisions through one owner-operated gateway.

Pangram currently lists $0.05 per started 100-word unit for Pangram 4 and a 20%
bulk discount. Bulk jobs currently allow 1,000 billable units, with a minimum
one unit per item. Check actual billing and volume terms before allocating a
live budget. [API pricing](https://www.pangram.com/solutions/api),
[billing units](https://docs.pangram.com/api-reference/bulk-api).

For a planning example with 24 tasks, one source and one final revision per task,
exactly 500 billable words per text and three observations per text:

```text
24 * 2 * 3 * ceil(500 / 100) * $0.05 = $36 realtime
$36 * 0.8 = $28.80 at the listed bulk discount
```

This excludes human review, proof computation and optional research calls.
Validators verify the committed reports through GETs rather than repeating the
paid inference schedule. Actual lengths, failed-call billing and additional
miner-paid attempts determine total spending.

Enforce the allowance before dispatch. Prevent reimbursement of duplicate task
IDs and require assignment-wide accounting for funded requests, including failures.
Private miner-paid attempts require no reimbursement ledger or disclosure. If
the budget or provider is unavailable, defer or void affected work consistently; do not
replace Pangram silently with a cheaper local detector. Publicly auditable
treasury policy and independent signers reduce operational trust, but converting
funds into a centralized provider's prepaid credits remains an external action.

## Next implementation decision

Adapt the offline contract to the public-report schema and an explicit repeat
policy. Establish supported retrieval and retention terms, report replay
accounting, fixed task/question commitments, verifiable hash assignments,
and complete review capacity before a live tournament. Public retrieval is now demonstrated;
network incentives, automatic fidelity judgments and cryptographic offline
proofs remain separate work.
