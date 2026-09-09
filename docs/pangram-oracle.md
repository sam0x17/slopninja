# Pangram benchmark, verification and API funding

Design proposal, 2026-09-09. Pangram is the required external benchmark for
slopninja. Evasion must also target the strongest eligible independent subnet
origin detector from the previous completed detection round. Neither result
substitutes for the other.

## What we measure

For origin detection, compare miners and Pangram against the same independently
documented production histories. Report calibration and human false-positive
rates alongside detection performance. Agreement with Pangram is not ground
truth and earns no separate reward. Author identification retains its own known
author labels because Pangram is not our author-attribution reference.

For evasion, use the explicit `pangram-4` selector and retain the returned
version, exact submitted text, task IDs and complete results. Measure
`fraction_ai + fraction_ai_assisted`; the project target is strictly below
`0.10`, after preservation and quality review. Record the unchanged source
under the same round policy. Improvement before crossing the threshold is
useful development feedback; it must not be reported as a successful sub-10%
revision. Writing-improvement rewards remain based on the editorial brief.

Pin the subnet opponent before the transformation round using held-out origin
Brier performance and human false-positive qualification. Exclude self-scoring
and use the best eligible independent opponent under the published fallback
order; author-identification rank does not qualify an origin detector. Controls
and probing monitor drift, but a service version does not attest private weights.
Until a competitive incumbent qualifies, bootstrap rounds use a named reference
detector and report their results separately.

Keep Pangram's fraction `F` distinct from the subnet probability
`H = p(model-only) + p(mixed)`. Use
`D_P = min_r F_r(source) - max_r F_r(revision)` and
`D_S = H(source) - H(revision)`. Joint evasion utility is the mean of
`max(0, D_P)` and `max(0, D_S)` only when both raw differences are nonnegative;
otherwise it is zero. Strict joint success also requires preservation,
`max_r F_r(revision) < 0.10` and `H(revision) < tau_S`. Calibrate `tau_S` on
development data and freeze it before scoring; it is not automatically 10%.
Missing required subnet evidence cannot become a Pangram-only pass.

Authorized retired revisions feed later detector training and disjoint private
evaluation, retaining their recorded production histories. This benchmark flow
never automatically forwards confidential customer inputs to Pangram or the
subnet opponent.

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
an encrypted envelope addressed only to assigned validators; publish a salted
commitment before assignment. Anyone obtaining the decrypted URL can still
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

1. Commit the challenge, model/score policy, candidate budget and evaluation
   window, including the subnet opponent, its threshold, fallback policy and
   source-baseline evidence.
   Put source hashes and private text inside salted payload commitments;
   public unsalted hashes can expose guessable text.
2. The miner commits its final revision, selected Pangram reports and score
   projection before learning its assigned evaluators. Require three distinct
   completed reports per mandated text. Miners may pay for additional candidate
   and Pangram attempts before commitment; disclosure of other private attempts
   is not required. The committed set cannot change afterward.
3. After the candidate commitment, the benchmark coordinator obtains the required
   signed candidate response from the pinned subnet origin service. Bind the
   round, service identity and settings, request nonce, text commitment and full
   origin-probability vector. Private search responses cannot replace it. Commit
   this evidence before the predetermined audit beacon becomes available.
4. Select evaluators using that future randomness source, whose availability and
   resistance to manipulation are checked before deployment. Both candidate and
   required subnet-response commitments must precede its release.
5. Deliver committed content through recipient-specific authenticated encryption
   after assigning validators. They verify the payload commitment and retrieve
   the reports directly from Pangram, comparing exact text, public identity,
   version and time window. They also verify the committed subnet responses'
   signatures and bindings. Required subnet serving calls use the explicit round
   budget. Pangram overlap checks reuse the existing reports through GETs without
   another inference charge; fresh validator-paid Pangram inference is not
   required. Public retrieval establishes the provider's current record;
   offline cryptographic provenance remains absent.
6. Publish commitments and appropriately aggregated results. Keep report IDs,
   exact text, salts and detailed evidence with authorized auditors. A challenge process
   needs a defined appeal path and retained evidence before final rewards.

The existing offline contract requires three distinct observations for a passing
full-document result. Keep that policy explicit in cost estimates. Three
matching calls are not three independent proofs of a text's origin. Disagreement
between calls is not by itself evidence of validator fraud; model variability,
provider errors and version drift must remain distinguishable from a forged
record. Assign consequences only for objectively provable protocol violations
under a published rule, without assuming Bittensor supplies custom slashing.

The [receipt and audit proposal](subnet-receipts-and-audits.md) specifies future
drand selection and distinguishes it from Bittensor's weight commit-reveal.
The current offline contract has not yet been adapted to public report IDs.

### Reproducibility spot checks

The [three-request probe](pangram-repeatability.md) found equal document class
fractions but different auxiliary scores for the same input, account, selector
and returned version. Same-task GET rechecks matched the original result.
Cross-account reproducibility remains unmeasured. A verifier must keep these
two checks separate:

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
response hash, task ID and score-payload hash inside a salted commitment before
future randomness selects the audited records and independent auditors.
Deliver openings and evidence encrypted to those recipients. Keep every accepted
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

Miners pay for private experimentation and required public candidate reports.
Required subnet-opponent benchmark calls use an explicit round budget; private
search against detector services remains miner-funded.
Validators retrieve existing Pangram reports without buying another inference under
the behavior observed in our probe. The subnet funds challenge baselines through
a capped validation allowance. Fresh reproducibility research needs separate
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
accounting, fixed task/question commitments, and verified future-beacon
selection before a live tournament. Public retrieval is now demonstrated;
network incentives, automatic fidelity judgments and cryptographic offline
proofs remain separate work.
