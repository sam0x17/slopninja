# Design consistency and incentives review

September 11, 2026. Reviewed whitepaper draft 0.15 at `c0c9ed6`; corrections
appear in draft 0.16; draft 0.17 adopts receipt-certified B payment. Scope covers the LaTeX paper, investor brief and current
model, payment, reward and evidence specifications. Historical experiment reports
remain descriptions of their implemented policies.

The revised design addresses the investor's confidentiality and commercial
questions at the specification level. It requires qualified private B execution
for emissions as well as customer jobs, links advertised performance to the
served model and budget, and limits unresolved-candidate withholding to that
candidate. It does not establish a secure implementation, profitable operation
or collusion-resistant native payouts. Paid settlement is now specified and
still needs implementation and qualification.

## Investor questions

| Question | Design response | Evidence still required |
| --- | --- | --- |
| Can a B miner read and trade on an unpublished announcement? | The customer releases text only to a verified job key inside the accepted runtime. The miner and host receive no customer key or plaintext copy. | Independently reviewed client, CPU/GPU configuration, model loader, denied egress, isolated state and hostile-model tests. Physical and side-channel exclusions must be accepted explicitly. |
| Can the latest B weights remain private too? | The owner separately provisions encrypted weights to the measured loader. Customers receive revisions, with no weight inspection interface. No mandatory monthly public release. | Independent key-release tests, authenticated model loading and version-bound receipts; behavioral imitation through purchased outputs remains possible. |
| Which product drives commercial demand? | B is the proposed primary paid product. A is a public detector resource with optional paid hosting. | Interviews, recurring editorial workflows, acceptance rates, editor time and repeat purchases. No demonstrated media buyer demand is claimed. |
| How is value captured? | Miners receive inference fees and may earn emissions. A separate application may charge disclosed team or usage fees. | Paid serving margin, subsidy accounting, application pricing and explicit subnet-slot ownership/lease terms. The protocol creates no automatic Slop Ninja Research royalty. |

The exact runtime and key-release requirements are in
[confidential customer inference](../whitepaper/sections/05-confidentiality.tex).
Hardware qualification must distinguish host software access from excluded
physical attacks. H100 uses hardware firewalls for GPU memory and encrypted
CPU/GPU transfer; its HBM is not encrypted. AMD also documents a physical attack
outside its SEV-SNP threat model. These examples prevent a blanket claim of
protection from every machine owner.
[NVIDIA hardware description](https://developer.nvidia.com/blog/?p=68661),
[AMD security bulletin](https://www.amd.com/en/resources/product-security/bulletin/amd-sb-3024.html).

## Incentive corrections

### Emissions must support a service customers can use

Previously, ordinary B generation could earn benchmark credit even though paid B
required confidential execution. A miner could optimize for emissions without
providing the qualified service needed for the commercial product.

Every credited B entry now requires attestation, its registered model/profile,
and a verified generation receipt. Mandatory free B rewrites use the same
qualified version/profile and receipts. Ordinary generation remains research
without emission credit. Attestation earns no quality bonus; meaning,
readability, target tone, author fit, Pangram and public A evaluation remain
separate requirements.

A benchmark now measures one bounded invocation: generation, internal search,
scoring and selection all occur inside the measured runtime. Its immutable task
context fixes the seed. Repeated execution, including after restart, must
reproduce the same result within the qualified profile. External best-of-many
generation cannot supply the final candidate. Additional Pangram observations
may still be selected for that same final text under the declared report policy.

Paid offers may cite a benchmark only for the same model, runtime and workload
profile. Cheaper search, changed prompts or newer weights require their own
evidence. Receipts do not prevent benchmark recognition, guarantee unseen-input
performance or establish that deterministic serving is economical. Those remain
qualification and evaluation requirements.

### An unresolved entry cannot erase completed rivals

The old whole-batch rule admitted a direct disruption strategy. Consider fixed
credits in two batches:

| Batch | Common owner's entry | Competitor |
| --- | ---: | ---: |
| First | 0 | 1 |
| Second | 1 | 1 |

The owner's share is `1/(0+1+1+1) = 1/3`. Making the first batch unresolved
and voiding it would raise that share to `1/(1+1) = 1/2`, without improving
either owned entry. Under the adopted rule, withholding only the unresolved
entry leaves the share at `1/3`.

An unresolved entry now retains `G=null` and contributes zero for its assigned
slot. Other complete entries keep their credit. Failed, missing, unresolved and
scoped-void slots retain their fixed assigned aggregation weights. This prevents
removing difficult cases from a miner's denominator.

A certified shared source/evaluator/platform failure affects every entry
requiring that evidence. A minority allegation or candidate-triggered error
cannot create a global exception. Conflicting strict certificates still halt
affected settlement. Semantic uncertainty alone creates no service failure;
independently attributable nonresponse retains its existing consequences.

This example verifies one fixed-credit exploit, not native reward equilibrium.
Selective validator non-certification remains possible when honest reviewers
disagree or are unavailable. Test reviewer coverage, ambiguous candidates,
errors misclassified as global incidents, common ownership and actual payout
normalization before launch.

### Costs and recipients must be explicit

| Participant or business | Receipts | Costs to measure |
| --- | --- | --- |
| B miner | Earned B emissions; independently quoted customer fees | Training, qualified generation, reserved/padded serving, mandatory rewrites, benchmark Pangram reports, delivery and fees |
| A miner | Earned A emissions; optional hosting fees | Training, artifact qualification/distribution and assigned requester work |
| Validator / benchmark operator | Its allocated rewards or named funding | Complete public A matrix, attestation checks, review, evidence transport and retrieval; separately budgeted source and anchor Pangram calls |
| Subnet owner | Native owner allocation under ownership/lease terms | Agreed operation and funding obligations |
| Slop Ninja application | Only separately disclosed application fees | Customer workflow, integrations, support and purchased inference |

Customer volume cannot increase benchmark rewards because miners can buy their
own jobs. Paid successes cannot dilute mandatory-service failures. A free-request
quota needs a measured, affordable cost; expected emissions alone do not prove
cost recovery. Report recurring paid serving margin separately from training
and benchmark subsidies. Native owner allocation and UID renumbering behavior
are documented in the [Bittensor subnet guide](https://www.bittensor.com/docs/guides/subnets);
neither establishes an investor's ownership or revenue rights.

## B payment decision, adopted in draft 0.17

Paid B now settles on a TEE-signed execution receipt, complete customer-encrypted
result publication and a validator delivery certificate. No customer
acknowledgment is required. The runtime signs only the result it generated and
encrypted for the accepted input under the agreed model/profile. Validators
check attestation and receipt bindings and the complete finalized ciphertext;
they receive no plaintext, weights, commitment openings or customer keys.

The paid-certifier roster is frozen from authenticated native state before
reservation, excluding the exact serving-miner UID and an authenticated linked
customer UID. The certificate requires `3w>2W`, with missing signers retained in
`W` and the same conditional dishonest-weight assumption below `W/3`.
Contract checks still enforce the runtime signature, job bindings, complete
ciphertext and deadlines; a quorum cannot waive them.

The bounded pilot publishes the full padded output in one completion transaction.
The job fixes acceptance, delivery and certificate cutoffs, with exactly one
payment or refund. Failure to certify a timely delivered result refunds the
customer at the final cutoff; retain `CERTIFICATION_TIMEOUT` separately from
proven execution failure. This removes the customer's unilateral payment veto
under honest, available quorum, but leaves miner exposure to certification
outages and chain censorship. Verification capacity and funding must be reserved
before accepting a job.

The accepted security-policy/revocation snapshot and job limits fix the
verification test. Later updates stop new acceptances and do not rewrite settled
or accepted job terms. Newly discovered vulnerabilities remain security incidents.
A changed key, instance or model cannot silently replace the accepted job.

Receipts certify computation and encrypted availability; editorial quality still
requires independent evaluation. Optional customer-chosen inspection follows a
separate explicit disclosure. Any additional editorial refund is a separate
transfer under agreed terms and does not reopen the initial B escrow.
Ordinary A hosting retains its separately disclosed acknowledgment policy;
qualified attested A hosting may use the receipt procedure.

The [paid-inference specification](paid-inference.md) and
[whitepaper settlement section](../whitepaper/sections/06-paid-inference.tex)
define the adopted rule. The alpha adapter, receipt verifier, publication,
certificate handling and client still require implementation and testing.

## Required evidence before launch

- Qualify the private B runtime and customer verifier against host attacks,
  malicious weights, false receipts, rollback, remote-resource output injection,
  cross-job leakage and protected CPU/GPU offload. Measure startup, padding and
  cost for the proposed model sizes.
- Demonstrate semantic-review agreement and capacity, public style-metric
  resistance to adaptive optimization, candidate-only closure, shared-failure
  attribution and the fixed denominator in the actual reward allocation.
- Test public A backdoors, near-copy models, related UIDs, predictable assignment
  gaming and selective withholding. Public models and attestation do not prove
  independent ownership or honest validator majorities.
- Validate the complete scheduling and review budget, Pangram integration,
  mandatory rewrite quotas, persistent recovery deficits and native payout lag.
  Preserve protocol counters and deficits across native UID renumbering.
- Implement and qualify paid settlement, alpha custody/accounting, complete
  encrypted delivery, frozen validator authority, deadline races and commercial
  purchase terms before accepting customer funds.
- Run a scoped editorial pilot with authorized material and independent writer
  judgments; measure both required detector outcomes only where external
  disclosure is authorized. Do not present benchmark success as a private
  customer's verified Pangram pass.

## Validation

This is a document and design review. The fixed-credit counterexample was
checked arithmetically; no new simulation, TEE deployment, escrow execution,
model training or live Pangram experiment was performed. Draft 0.17 builds
from the checked-in LaTeX as a 41-page PDF with no build warnings. Changed
payment, participant and trust pages were inspected, 179 local links resolved,
and the diff passed whitespace checks. The existing Rust experiments retain
their recorded scope and do not implement the revised launch protocol.
