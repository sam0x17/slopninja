# Paid asynchronous inference

Design update, September 11, 2026, whitepaper draft 0.17.
Every paid B job requires qualified attested execution with independent
model/customer key release. B emissions and mandatory rewrite service require
the same qualified version and workload profile. Ordinary A hosting remains an
option with explicit customer acceptance. B weights remain private.
The [whitepaper](../whitepaper/slop_ninja.pdf), including its
[model policy](../whitepaper/sections/03-models.tex) and
[customer protocol](../whitepaper/sections/05-confidentiality.tex),
defines the canonical design. The implementation remains unqualified.

Proposal, updated 2026-09-11. The [whitepaper PDF](../whitepaper/slop_ninja.pdf)
([LaTeX source](../whitepaper/main.tex)) is the canonical design draft.
Customers buy asynchronous jobs at miner-quoted prices in subnet alpha from
providers hosting public A detectors or private B transformation models.
A hosting is optional; publishing a qualified artifact does not require an A
inference endpoint. Offers, assignments, commitments, escrow
and settlement belong on chain; B execution runs on qualified confidential
hardware, owned or rented by the miner. This repository
has no deployed job contract or paid serving endpoint yet.

Initial paid offers cover two task interfaces: A for author/origin detection,
and B for joint author-style transformation and detector evasion under the source
and brief. B targets both author fit and evasion of Pangram and the subnet's
qualified detectors. Audience, tone, readability and fidelity belong in
the transformation brief and its acceptance criteria.

Confidential B jobs pursue both objectives through local generation and
evaluation. Direct Pangram verification uses authorized benchmarks; a private
job cannot claim its own verified Pangram result without a separately authorized
disclosure. The joint objective grants no additional recipient access.

Both interfaces can earn hosted inference fees. A customers pay for compute
and delivery and can instead run the published model themselves. This outside
option can put downward pressure on hosted A prices; the protocol sets no
fixed A discount or A/B price ratio. Both prices come from competing quotes
denominated in alpha.

## Customer confidentiality

Every paid B job requires a qualified confidential runtime. The model owner
releases encrypted model material only after verifying that runtime; the customer
independently verifies its fresh attestation, loaded model commitment and job key.
The customer encrypts the source, references, profiles and brief to that key.
The operator cannot substitute an ordinary server key. It receives ciphertext
and permitted metadata, with no customer input key or plaintext output copy.

The public runtime must cover the complete CPU/GPU path, constrain private
weights to approved model data, deny content-bearing egress and telemetry, and
isolate and clear job state. No operator shell or guest administrator may inspect
the protected workload. Fixed workload buckets and response slots bound exposed
size/timing metadata. The trusted client must also disable active markup and
automatic remote fetches in model outputs. These are qualification requirements,
not capabilities demonstrated by this repository.

A weights remain public. A customer can run them locally, select a qualified
confidential service, or explicitly accept operator access through ordinary
hosting. Every offer must identify which policy applies.

The accepted runtime encrypts the result to the customer's delivery key. No
validator, support operator, storage provider or external model service receives
a default input key or output copy. Customer jobs cannot call Pangram, train on
the text, or enter benchmarks automatically. Authorized benchmarks have their
own specified recipients and external-evaluation permissions.

A customer can separately disclose selected evidence to one chosen validator.
Reassignment requires a new customer-signed assignment, freshly verified
instance and new envelope. A host cannot forward the job to an unapproved key.
Revocation cannot undo any disclosure already authorized under an earlier flow.

Hardware and measured software remain trusted. Attestation does not prove
editorial quality, prevent denial of service, or eliminate hardware side
channels. The full requirements and physical threat limits are in the
[customer protocol](../whitepaper/sections/05-confidentiality.tex).

## What belongs on chain

| Component | Proposed location | Evidence available |
| --- | --- | --- |
| Offers, assigned keys, acceptance and deadlines | Contract state/events | Agreed service and authorized participants |
| Deposits, delivery certificates, payment and timeout refunds | Contract | B payment requires a verified runtime receipt, complete ciphertext and strict weighted certificate |
| Salted input/result commitments and envelope hashes | Contract | Binding to exact bytes when an authorized recipient checks an opening |
| Source, references, profiles, brief and result | Encrypted transport/storage | Plaintext available only to the designated endpoints |
| Hosted inference and B search | Qualified confidential hardware for B; declared hosting mode for A | B receipt binds model/runtime and bounded workload; output quality requires independent evaluation |

All content-bearing fields stay inside the envelope. Public offer and job
metadata must not include excerpts, private profile values or the brief.
Keep commitment salts and openings encrypted as well. Bare ciphertext or a
signature alone cannot establish correct execution and delivery. The receipt,
attestation and publication checks below provide that evidence under their
stated assumptions; writing quality remains separate.

The initial bounded B pilot publishes the complete padded result and receipt
in one completion transaction through the job contract. Measure 4, 16 and 64 KiB
result envelopes, receipt overhead and finalized-history retrieval on local/test
chain before choosing the maximum. Input transport may remain off-chain.
On-chain transport exposes size, timing and counterparties and permanently
retains ciphertext that could become readable after key compromise. Off-chain
encrypted result storage needs a separately qualified availability protocol;
it cannot substitute for the pilot's complete on-chain result.
Do not promise a fixed fee or treat block capacity as a per-subnet allowance.

Subtensor's EVM supports the proposed contract route without a new runtime
pallet. Deployment permissions and the alpha settlement adapter still need
implementation checks. Native hotkey commitments are quota-limited replacement
metadata, suitable for an epoch root rather than an append-only job database.
[Official EVM guide](https://www.bittensor.com/docs/guides/evm),
[deployment guide](https://www.bittensor.com/docs/guides/evm/deploy-a-contract),
[commitment types](https://github.com/RaoFoundation/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/commitments/src/types.rs).

## Offer and job lifecycle

A signed offer binds the miner hotkey, authorized settlement identity, encryption
key, task schema, model/version and qualified runtime evidence, total alpha
quote for a bounded job,
available capacity, input/output limits, quote expiry, acceptance, delivery and
certification deadlines, and refund policy. B reservations also bind the verified
instance keys, accepted security policy and frozen paid-certifier snapshot.
Customers choose among
compatible offers using benchmark quality, deadline and price. Miners may change
future offers as demand, queue length, costs and competition change. There is
no owner-set base price, utilization target or global price-adjustment parameter.

An atomic reservation consumes offered capacity and locks the quoted alpha
amount and terms. Expired or exhausted offers cannot be filled, and later
repricing cannot change reserved jobs. The customer signs the assignment and
binds its delivery key. The B profile binds the bounded internal search and
padded workload; payment uses the reserved quote without exposing
content-dependent token counts. The service market determines
the alpha amount separately from the alpha/TAO exchange market.

Quote discovery uses public service descriptions and size/deadline metadata
the customer chooses to reveal. It sends no source, references, profiles or
brief plaintext to prospective bidders. The B envelope is addressed only to
the verified instance key. If the provider cannot serve under the reserved
terms, the job follows its rejection/refund policy; it cannot silently reprice
after decryption.

The proposed adapter would custody unlocked, transferable same-subnet alpha
using the staking-v2 interface and contract-held native stake. An allowance
alone does not reserve funds. Bind the subnet, hotkey position and alpha amount
in native units at `10^-9` precision. The service price and network/transfer fees
must be separate, with a named fee payer; funding must credit the quoted
principal. Release and refund must reconcile actual balances, transfer
restrictions and fees, keeping incidental accrual separate from job principal.
This adapter and accounting remain unimplemented and must be tested before
customer funds are accepted; no alpha ERC-20 interface is assumed.
[Staking-v2 interface](https://www.bittensor.com/docs/guides/evm/precompiles/staking-v2),
[stake from EVM](https://www.bittensor.com/docs/guides/evm/stake-from-evm).

The contract must authenticate the hotkey-to-EVM-address association. Copying
or truncating identifiers is not authorization. The documented sr25519
precompile verifies a signed 32-byte digest, whereas the existing envelope
prototype signs a variable-length message. Define versioned contract-signing
bytes and verify interoperability before binding either format to escrow.
[Key verification guide](https://www.bittensor.com/docs/guides/evm/verify-keys).
Rust remains the service/client language; Solidity is the proposed EVM-contract
exception.

For B, the customer independently verifies fresh attestation and the instance's
job encryption and receipt-signing keys before signing the reservation or
disclosing input. The model/profile, security-policy and revocation snapshot,
maximum evidence age and job duration must still qualify at acceptance.
Later updates stop new acceptances; they do not retroactively change settlement
for an accepted bounded job. A discovered vulnerability remains a security
incident and cannot be repaired by a payment decision.

```mermaid
sequenceDiagram
    participant C as Customer
    participant E as Job contract
    participant M as Miner or relayer
    participant R as Qualified B runtime
    participant V as Frozen validators
    C->>R: Verify runtime, model and job keys
    C->>E: Sign reservation with keys, terms and roster; fund escrow
    C->>R: Customer-encrypted input
    R->>E: Signed acceptance through host relay
    Note over R: Run agreed model and budget; encrypt result to customer
    R->>M: Complete padded ciphertext and signed receipt at fixed slot
    M->>E: Publish complete ciphertext and receipt by delivery cutoff
    V->>E: Read finalized publication and accepted job
    Note over V: Verify attestation, receipt and complete encrypted delivery
    alt Timely certificate with more than two-thirds of frozen weight
        V->>E: Relay DELIVERED certificate
        E->>M: Pay the fixed miner payee
    else Acceptance, delivery or certification cutoff missed
        C->>E: Request settlement after cutoff finality
        E->>C: Refund principal
    end
    C->>E: Retrieve ciphertext and decrypt locally; no acknowledgment needed
```

The protected runtime validates the input commitment opening, schema, limits and
customer-signed output key before signing acceptance. It then executes the
registered model and complete workload profile and encrypts the result itself.
Only successful completion of that procedure may produce the payment receipt.
It cannot sign externally supplied text or pretend that a failed or incomplete
run succeeded. A poor revision may still be a correctly executed result.

The initial completion transaction contains the entire padded ciphertext and
signed receipt. The contract checks length, hash and the pinned receipt
signature, then records retrievable bytes in calldata or events. Partial
uploads, hashes and HTTP URLs cannot establish delivery. Invalid submissions
must not lock out a valid completion. Any party may relay exact signed evidence;
the job's payee and output key never change. A different instance, key,
model/profile or assignment requires new customer authorization and a new job.

## Payment without plaintext arbitration

B pays for the agreed execution and retrievable encrypted result. Its
`DELIVERED` certificate replaces customer acknowledgment. Both the runtime
receipt and complete ciphertext are required; a validator opinion cannot waive
the contract's signature, binding, publication or deadline checks.

The receipt binds:

- Protocol, chain, subnet, contract, job, assignment generation, nonce and offer.
- Signed acceptance and salted input commitment.
- Miner/payee, loaded model content, runtime/policy and bounded workload profile.
- Attested instance evidence, pinned receipt key and customer output key.
- Exact padded ciphertext hash and length, and the fixed deadlines.

The instance creates and protects its receipt key inside the accepted runtime.
Validators independently verify its binding to the approved loader and complete
CPU/GPU path, model, policy, freshness and accepted revocation collateral. They
inspect acceptance, the completion receipt and finalized publication of all
ciphertext bytes. They receive no input/result plaintext, raw text hashes,
commitment openings, private weights or customer keys.
[Remote attestation architecture](https://www.rfc-editor.org/rfc/rfc9334.html).

### Frozen validator authority

Before reservation, derive the complete paid-certifier roster from the epoch's
protocol-defined validator eligibility and proof-verified finalized native state.
Do not select a subset by responses to this job. Use the
[snapshot authentication rules](service-evidence.md#frozen-authority-and-parameters).
Bind validator identities, registration generations, signing keys and native
integer effective consensus weights. Exclude the exact serving-miner UID and
any customer UID linked by authenticated chain identity. Do not let the customer,
miner or a relayer select an easier subset. Assume dishonest weight is strictly
below one-third of this conditional set; different keys do not prove independent
ownership.

Let total frozen weight be `W>0`. Each signer independently checks the evidence.
A certificate requires distinct eligible signers with total agreeing weight
`w` satisfying `3w>2W`. Missing, abstaining, recused or offline validators
remain in `W`. The certificate binds the job/offer, roster, accepted policy,
receipt digest, completion transaction and ciphertext hash/length. The contract
checks the certificate signatures and weights against that snapshot as well
as the runtime receipt and recorded publication. Exact retries are idempotent;
replacement outputs, keys and rosters cannot change the job.

This roster authorizes public receipt verification only. Benchmark DATA/witness
envelopes must never give it customer plaintext access. Reserve verification
capacity and identify its funder before accepting a job. Any verification fee
must appear in the quote; certification creates no automatic extra fee or
third reward pool. Paid traffic does not increase benchmark rewards or dilute
mandatory-service failures.

### Deadlines and terminal accounting

Each reservation fixes `H_a < H_d < H_c`: acceptance, delivery and certification
cutoffs, plus a fixed padded response-release slot and bounded publication
window. The qualified policy supplies minimum execution, inclusion and
verification windows. Missing capacity or incompatible limits prevent
reservation. The runtime releases its padded response at the fixed slot; host
delay cannot change its hash or extend the deadline.

| Condition | Outcome |
| --- | --- |
| No timely signed acceptance | Refund after acceptance cutoff finality |
| Accepted, but no timely valid completion publication | Refund after delivery cutoff finality |
| Timely completion and timely valid weighted certificate | Pay the fixed miner payee |
| Timely completion, but no timely certificate | Refund after certification cutoff finality; retain `CERTIFICATION_TIMEOUT` |
| Late, duplicate or mismatched evidence | Cannot replace the result or cause another settlement |

Successful canonical inclusion at a cutoff is timely. Validators certify only
after acceptance and delivery evidence are final. Certificate inclusion must be
by `H_c`; the outcome becomes terminal after finality. A late certificate cannot
recover refunded principal. A miner, customer or keeper submits settlement
transactions. The contract permits exactly one payment or refund, with no
deadline extensions through replacement evidence or editorial disputes.
Finality stalls may delay terminal settlement.

With honest, available quorum and timely chain inclusion, a customer cannot
consume the output and veto payment. Validator outages, censorship or lack of
agreeing weight can still refund a delivered job. The miner bears that bounded
risk. A certification timeout is distinct from proven execution failure and
does not automatically alter mandatory-service records or emission credit.
No fallback may shrink the frozen denominator or draw new judges.

### Scope of the payment guarantee

The runtime receipt establishes execution under the accepted hardware/software
assumptions. The validator certificate establishes the required evidence and
complete encrypted delivery under the quorum assumption. Neither certifies
semantic fidelity, author fit, a Pangram pass or editorial satisfaction.
Private jobs never enter benchmark review automatically.

Ordinary A hosting has no protected execution receipt. Its separately disclosed
pilot policy retains customer acknowledgment and bounded no-ack refunds, with
miner nonpayment risk. Qualified attested A hosting may use the same delivery
procedure for its declared A computation. A publication and emission eligibility
require neither paid hosting nor attestation.

## Optional customer-directed inspection

A customer can send a separate evidence packet to **one explicitly chosen
validator**. Authenticate that validator's encryption key before sending. The
customer signs and encrypts the packet, binding the chain, contract, job ID,
review context, recipient key, scope and nonce. Include the exact relevant
evidence and any commitment salts/openings needed for the requested checks.
The original input envelope and accepted execution policy remain unchanged.

The chosen validator sees only what the customer supplies. There is no standing
access, validator broadcast, miner rewrapping or automatic escalation. Disclosure
cannot be revoked after the validator has read it. Partial or redacted evidence
may not open the original whole-payload commitment; a hash of an excerpt does
not establish that it came from that committed payload. Incomplete evidence
cannot prove whole-job fidelity. A review must state which bindings it checked
and which context was unavailable.

Inspection is advisory by default. Bind any opinion to the job, evidence packet
and scope. For the initial B escrow, execution/delivery certificates determine
settlement. Additional editorial refunds or revisions follow separately accepted
terms; refunds are separate transfers and cannot reopen settled escrow.
Selecting a reviewer cannot redirect the original principal, change deadlines
or grant automatic access to undisclosed material.

## Emissions, fees and serving

Benchmark emissions and customer revenue remain separate. Emissions reward
measured performance on authorized benchmark assignments, not claimed GPU
hours or customer transaction volume. A miner could buy its own jobs, so those
payments must not increase benchmark rewards. Benchmark validators may review
benchmark text under its own access policy.

The pilot assigns A to on-chain mechanism 0 and B to mechanism 1, with 50% of
emissions each. Editorial quality is part of B's gates and brief requirements.
The [reward rules](../whitepaper/sections/07-rewards.tex) also require fulfillment
of bounded mandatory free B rewrite service for A training requesters.
Only the actual A-requester and B-provider classes count; there is no mandatory
A-provider inference or B-requester prediction class. Removed classes do not
create zero-workload failures. A artifact qualification and validator benchmark
replay remain separate. More than 10% failed valid assigned task obligations
gives zero earned A and B credit for the measured epoch, with separate mandatory
gates for each interface and actually assigned requester/provider role, with no
automatic pass without assigned work. Each obligation binds its UID and role. Exactly
10% passes this availability gate. Paid jobs cannot dilute mandatory-service
failures. Measurement closes before the relevant weight deadlines; the published
payout lag does not claw back emissions already distributed.
Each mandatory interface/role must also clear its persistent recovery deficit
through actual same-class protocol probation work. Paid success, another service
class, elapsed time or key/service-version rotation cannot clear that deficit.

The alpha service quote covers local inference/search and agreed storage costs;
the offer separately identifies network/transfer fees and their payer. Miners
and validators separately fund their benchmark submissions and
review, including any required Pangram calls. Default private customer quotes
include neither automatic detector submissions nor validator reading fees.

Publish aggregate benchmark performance and public offers. Private customer
outputs and profiles stay out of leaderboards. Every credited B candidate and
mandatory B rewrite requires a qualified-runtime execution receipt. A paid offer
may cite a benchmark only for the same model/runtime and bounded search profile;
a cheaper or changed version needs its own evidence. Qualification is a binary
admission gate and earns no quality bonus. Attestation and benchmark evaluation
remain separate checks, both requiring implementation.

Serving can use signed `POST /jobs` with `202 Accepted`, followed by authenticated
polling or encrypted retrieval. Application transport remains the subnet's
responsibility; do not build around the removed Axon/Dendrite server
implementation. [SDK migration guide](https://www.bittensor.com/docs/migration#axon-dendrite-and-synapse-are-gone).

## Smallest next experiment

Use synthetic jobs and test funds to check alpha custody, exact balances,
quote expiry, capacity reservation and exactly one terminal outcome. Exercise
valid receipt/certificate payment while the customer sends no acknowledgment;
wrong runtime/model/input/output-key bindings; fake, stale and replayed receipts;
missing or altered ciphertext; insufficient and duplicate signer weight;
receipt-only and certificate-only attempts; certificate delay; and refund/payment
races at every cutoff. Invalid uploads must not consume the valid result slot.
Check that another relayer cannot change the payee.

Keep plaintext at the customer and runtime. Test verifier evidence and key
revocation at acceptance, subsequent updates, instance restart, fixed padded
release, archived ciphertext retrieval and refusal to substitute off-chain URLs.
Measure gas and verification capacity at the supported result sizes. Record
valid delivered jobs refunded for certification timeout separately from
execution failure. Ordinary A acknowledgment tests remain a separate mode.
These are required pilot checks, not tests implemented by this document.

The existing [receipt envelope](receipt-envelope.md) is a cryptographic building
block. Its validator-recipient benchmark schema is not a customer-job envelope
or escrow implementation. A customer-specific schema must enforce the verified
B instance input key and customer result key before deployment.
