# Pangram benchmark, verification and API funding

Design proposal, 2026-09-09. Pangram is the required external benchmark for
slopninja. Competing subnet detectors provide additional adversarial pressure;
they cannot replace the Pangram result or redefine a successful evasion.

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

Prefer provider-signed receipts if Pangram offers them. A receipt should bind
the request body hash, model selector, task ID, response hash, returned version,
time and billable units. Verification would then be cheap for every validator,
while the subnet would still trust Pangram as the external benchmark provider.

The alternative to investigate is authenticated TLS transcripts. TLSNotary is
implemented in Rust and supports proving server responses while hiding secrets
such as API keys. Its documented notarized mode requires trust in the notary;
multiple independent notaries can reduce collusion risk. It currently documents
TLS 1.2 support. We have not implemented or measured a Pangram proof, so neither
full protocol compatibility nor proof cost is established by this design.
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

1. Commit the challenge, source hash, model/score policy, candidate budget and
   evaluation window. Keep private text with authorized evaluators; a public
   commitment need not disclose the source.
2. The miner commits its final revision before learning its assigned evaluators.
   One final candidate per assignment limits paid search through validator APIs.
3. Select evaluators using a future randomness source whose availability and
   resistance to manipulation are checked before deployment. Each evaluator
   reserves a capped API allowance before submission.
4. Evaluate the exact committed revision through Pangram, preserving every
   accepted task, result, failure and unresolved request. Resume known tasks;
   never treat an uncertain submission as permission for an unrecorded retry.
5. Initially, use independently operated validators with their own API accounts,
   signed measurement records and randomly assigned overlapping checks. This
   relies on honest evaluators; it is not cryptographic response provenance.
   Replace duplicated trust checks with verified receipts/transcripts only after
   an end-to-end proof implementation has been demonstrated.
6. Publish verifiable commitments, usage accounting and aggregate results.
   Share exact protected text only with authorized auditors. A challenge process
   needs a defined appeal path and retained evidence before final rewards.

The existing offline contract requires three distinct observations for a passing
full-document result. Keep that policy explicit in cost estimates. Three
matching calls are not three independent proofs of a text's origin. Disagreement
between calls is not by itself evidence of validator fraud; model variability,
provider errors and version drift must remain distinguishable from a forged
record. Assign consequences only for objectively provable protocol violations
under a published rule, without assuming Bittensor supplies custom slashing.

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
hash, explicit model selector, required returned version, comparison fields,
and evaluation window. Preserve exact text bytes and bind assignment nonces
outside the prose. After measurement, the evaluator commits its own complete
response hash, task ID and score-payload hash before future randomness selects
the audited records and independent auditors. Keep every accepted task,
failure and unresolved request in the budget ledger.

Compare a versioned projection of reward-relevant fields, including successful
completion, returned version and all three document fractions. Retain task IDs
and timestamps for provenance, while excluding them from cross-task score
equality. Auxiliary scores need their own declared comparison policy if used.
Any tolerances must be justified on separate measurements and frozen before
scoring; this small probe supplies none. An audit tolerance never changes the
strict `fraction_ai + fraction_ai_assisted < 0.10` reward gate.

A suggested absolute tolerance of `0.02` (two percentage points) remains an
unvalidated candidate. For example, `9.5%` and `11%` differ by less than two
points, but the `11%` observation still fails the strict reward threshold.
Agreement and reward eligibility are separate decisions.

A mismatch opens a bounded dispute. Distinguish wrong request bindings, changed
versions, provider failures and same-version score disagreement. Retain the
original evidence, use a capped reconciliation schedule with contemporary
controls, and defer unresolved work under the published epoch policy. Numeric
disagreement alone does not establish fraud. Authenticated evidence of the
original response remains the stronger route to response provenance, with
Pangram still trusted for its computation.

For `N` committed measurements, audit fraction `q`, and `r` fresh requests per
selected measurement, reserve about `q * N * r` additional evaluations at the
applicable text-length rate, plus capped reconciliation costs. These checks
supplement the existing repeat budget. Agreement cannot prove an immutable
model or independently reproduce the provider's proprietary computation.

## Who pays

Use separate development and scored-evaluation budgets. Miners pay for their
private experimentation. The subnet funds the limited, randomly assigned
benchmark measurements through a capped validation allowance. Bootstrap that
allowance from an explicit project budget; later fund it from service fees
and a published operating allocation. These are proposed funding sources, not
an automatic entitlement to a fraction of chain emissions.

Initially, validators purchase their own credits. Reimburse scheduled work at a
published rate and cap, using task receipts and independent overlap checks.
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

This excludes human review, proof computation and extra auditor calls. If each
of three validators repeats that entire schedule independently, API cost triples.
Authenticated reusable receipts could avoid that duplication; they would not
remove any separately required fresh-repeat budget. Actual lengths, failed-call
billing and the agreed repeat policy determine the final cost.

Enforce the allowance before dispatch. Prevent reimbursement of duplicate task
IDs and require assignment-wide accounting, including failures. If the budget
or provider is unavailable, defer or void affected work consistently; do not
replace Pangram silently with a cheaper local detector. Publicly auditable
treasury policy and independent signers reduce operational trust, but converting
funds into a centralized provider's prepaid credits remains an external action.

## Next implementation decision

Test the API-proof route on a synthetic, nonprivate request before choosing the
live architecture. Establish request/result binding, replay resistance, key
privacy, verifier assumptions and measured proof cost. In parallel, specify the
project's initial spending cap and the evaluator reimbursement policy. This
determines whether the first tournament can use reusable proofs or must begin
with the explicitly weaker model of independent paid measurements.
