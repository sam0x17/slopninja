# Mandatory A/B service evidence

Design update, September 11, 2026, whitepaper draft 0.17. B emissions,
mandatory rewrites and paid B jobs require qualified attested execution.
The DATA/witness transport below serves curator-authorized training tickets.
It must never export customer instance keys or grant automatic customer access.
The pilot does not implement attestation; emission eligibility additionally
requires the model/profile bindings and receipts in the
[model policy](../whitepaper/sections/03-models.tex).
The [whitepaper](../whitepaper/slop_ninja.pdf) is the canonical design.

This specifies the proposed `service-evidence-v1` transport and accounting pilot
for bounded authorized rewrite requests from A requesters to B providers.
A models are public; B generators remain private. Validators execute complete
frozen, qualified A artifacts for authoritative benchmark probabilities.
B rewrite responses supply training material for A.

The one-ticket exercise reviews its B rewrite. It implements neither an artifact runner nor the
common three-A source/candidate matrix. An emission-bearing benchmark requires
a separately frozen execution schedule and capacity budget. Private customer
jobs keep their separate paid lifecycle and never enter mandatory-service
counts. B inputs are encrypted to a verified key inside the qualified runtime,
without miner or host access. A customer may separately disclose evidence to
a chosen validator. This is a protocol
design; no mailbox deployment or throughput result is claimed.

The public ledger establishes who published which ciphertext bytes and when.
A certificate establishes the declared private validity or quality verdict
under the validator honesty assumption. Certified delivery of a B response
does not establish its revision quality.

Freeze epoch qualification and cache the complete public A panel artifacts before
B task disclosure or candidate generation. The execution manifest binds weights, features, tokenizer,
preprocessing and postprocessing, calibration, runtime, dependencies and every
parameter needed to reproduce the output. Validator execution receipts bind
that manifest, artifact hashes, canonical runtime/settings, input commitment and
complete probability output. B's scored submission includes full locally
computed vectors, the protocol-assigned A models/settings and the exact
candidate-input commitment. Validators re-execute and compare canonical outputs;
the submitted claims cannot replace their results.
Independent hidden qualification labels and targeted-trigger controls remain
necessary because a published artifact can reproduce deliberately biased behavior.
Training recipes and training data need not be public merely to publish the
complete executable artifact.
Validator execution has a separate funded capacity budget from rewrite-service
tickets. B miners can compute their own candidates' A scores
during search; those scores are not secret. Hidden labels, other miners'
candidates, unreleased sources and semantic-review evidence retain their
authorized recipient restrictions.

After issue, no model, version or threshold can change. A validator-node outage
permits rerunning the same cached artifact on a reserved approved runner.
Candidate-specific reference failure withholds only that entry's credit.
Certified shared failure affects every entry requiring that evidence; retain
assigned weights. Neither outcome implies a passing score or B nonresponse.
Deliberately triggered execution errors remain a
required pilot attack case. These execution rules do not change the fixed B
provider fallbacks on rewrite-service tickets.

## Frozen authority and parameters

Before task contents are disclosed, freeze the round's exact A/B participant UIDs and exclude those exact UIDs from its eligible validator list. For every remaining validator, bind the UID, registration generation, native hotkey, signing key, encryption key version, and nonnegative integer native effective consensus stake, including the runtime's native alpha/TAO scaling. Do not substitute equal UID weights or an application UID cap. The manifest identifies the finalized source block hash, runtime code hash, SCALE/storage schema, subnet, mechanism, native weight field and units, complete validator records, and their sum `W`. Initial registry loading and later snapshots require verified finalized state proofs or a chain-verified native snapshot. A relayer signature alone is insufficient.

Each final certificate needs distinct signers with total frozen weight `w` satisfying **`3w > 2W`**. Assume dishonest weight is strictly below `W/3` in this conditional, frozen validator set. Nonresponse, missing keys, recusal, delegation, and later key changes never reduce `W`. With `W=0`, issue nothing. Every validator must receive its confidential evidence envelope; three hash-assigned preparers organize checks and evidence but cannot approve a verdict themselves. Honest validators sign at most one verdict for a stage and independently check the evidence. Two conflicting strict quorums would overlap in more than `W/3`, contradicting that assumption if honest signers follow the rule. Duplicate signatures count once; any party may relay a valid certificate.

The following are actual pilot settings, rather than unspecified future values. An instance manifest must copy them and supply concrete chain/contract addresses, proof-verified snapshots, source authorizations, service hashes, task-schema hashes, fees and funded reservations. Missing fields prevent issue. A later change creates a new version and cannot alter issued tickets.

| Parameter | `service-evidence-v1` value |
| --- | --- |
| Measured epoch | 360 native block heights; issue offset `H=epoch_start` |
| Global ticket budget | One authorized rewrite: A requester to B provider |
| Permitted providers | Three B providers total: mandatory first provider and two ordered fallbacks |
| Input / output payload cap | 4,096 / 8,192 bytes, including task data and private metadata |
| Ciphertext chunk cap | 4,096 bytes; the final chunk may be shorter |
| DATA plus witness publication cap | 8 MiB per stage across all recipient copies |
| Epoch publication cap | 64 MiB across the ticket and all possible attempts |
| Request publication / certificate cutoff | `H+12` / `H+24` |
| B provider response window | 36 blocks |
| Certificate window after each response | 12 blocks |
| Preparer commitment / private opening cutoff | `H+180` / `H+192` |
| Final quality, incident, and accounting close | `H+204` |
| Certificate threshold | Strict `3w > 2W`; no rounding or responder renormalization |
| Availability / recovery | `10F <= N`, `U=0`, and `D=max(0,D_previous+10F-N)` |

At stage `j` in `1..3`, the fixed start is `H+24+48*(j-1)`, publication cutoff
is start plus 36 blocks, and certificate cutoff is start plus 48 blocks. The
final B certificate cutoff is `H+168`. Early declines do not pull later windows
forward. Unused stages remain reservations and earn no availability credit.
All ciphertext and witness copies must fit their stage's byte, transaction,
gas and verifier reservations before issue. Insufficient capacity prevents
that scheduled issue; it does not permit another peer draw. One ticket is a
small transport pilot, not enough evidence to qualify every subnet UID.
The whitepaper's design-history note identifies earlier fixtures;
they do not implement this directed rewrite pilot or the artifact runner.

The 360-block pilot requires a subnet with that measured tempo. A different tempo needs a separately versioned schedule. Settlement binds a particular later native payout cycle and its actual commit/reveal configuration; the remaining block heights are not a claim that native reveal delays fit inside this epoch. No issue is permitted if its declared settlement path cannot use the closed result in time. A finality stall can delay observing closure beyond that payout cycle; the settlement adapter must then apply its declared unresolved-round handling rather than fabricate a timely result.

## DATA first, confidential witness second

Use HPKE base mode with `DHKEM(X25519, HKDF-SHA256)` (`0x0020`), `HKDF-SHA256` (`0x0001`), and `ChaCha20Poly1305` (`0x0003`). Each recipient gets a fresh single-shot context at sequence zero, with independent ephemeral generation material. Authenticate the sender with its frozen registered signing key. Replay state belongs to the mailbox. These choices use the [HPKE specification](https://www.rfc-editor.org/rfc/rfc9180.html), rather than treating recipient encryption as sender authentication.

Encode protocol records using the core deterministic CBOR rules: definite lengths, shortest integers, deterministic map ordering, no duplicate keys, tags, floats, or indefinite items. Treat content as byte strings; never normalize text during validation. The schema fixes field types and rejects unknown fields. See [RFC 8949, section 4.2.1](https://www.rfc-editor.org/rfc/rfc8949.html#section-4.2.1).

Build each stage in this order:

1. Construct payload `P` and an independent 32-byte random salt. Commit `C=SHA256(encode("slopninja/service-evidence-v1/plaintext", salt, P))`. Curator authorization binds the task, source family, permitted use, payload commitment, and exact miner/requester/validator recipients. Benchmark publication must be authorized; no private customer payload can be relabeled as benchmark work.
2. For each required DATA recipient, form a context containing the protocol hash, chain, contract, ticket, stage, attempt, sender UID/generation, recipient UID/generation, frozen key version, and `C`. Use its encoded bytes as HPKE `info` and authenticated additional data, with distinct DATA and witness domains. Encrypt the same `P` to each DATA recipient. Input recipients are the requester and all three prescribed providers; response recipients are the requester and the actual responding provider. Duplicate recipient identities receive one copy. Hidden labels and private review answers use separately typed evidence packets addressed only to the frozen validators.
3. Sign a DATA manifest containing that context and every recipient's encapsulated key, ciphertext length, chunk count, and chunk hashes. Publish the complete ciphertext bytes as authenticated mailbox transactions/events. The contract retains the manifest digest and completion state; authenticated chain history retains the full payload bytes. A hash or an off-chain locator alone cannot complete this stage.
4. Only after the DATA manifest is fixed, construct a witness packet containing its digest, `P`, the salt, and the generation material for each DATA envelope's ephemeral key. Validators reproduce key derivation, encapsulation, and single-shot sealing under the pinned suite, then compare every ciphertext byte and recipient binding. They also check the authorization, commitment, and task schema. This proves consistency of the miner/requester copies to those validators; decrypting only their own copy would not establish it.
5. Encrypt that witness packet independently to every frozen eligible validator, publish every witness ciphertext, and sign a separate witness manifest referencing the DATA digest. No DATA field depends on a witness hash, and no witness contains its own envelope-generation secret. This avoids a hash cycle and recursive disclosure. A validator decrypts its own witness copy and verifies the DATA copies; a global certificate does not assert that every other validator could decrypt its witness copy.

Generation material is confidential capability to reconstruct the corresponding DATA encryption. Never publish it, the plaintext, salts, report URLs, or commitment openings. Do not reuse DATA generation material for witness envelopes or another recipient. Require the pinned HPKE implementation's key validation and error rules, and test witness reproduction against its deterministic test interface. The witness mechanism requires a separate implementation and security review; this document does not claim the existing receipt prototype implements it.

The source and result keys are frozen per ticket. Hotkey rotation, service upgrades, UID reuse, and replacement registrations cannot change an existing recipient or its obligations. Enroll encryption keys with a proof-of-possession challenge before the snapshot. Losing the private key afterward does not blame a sender whose DATA envelope was certified correct. New keys apply only to later tickets. Every authorized validator can see active benchmark evidence; validator confidentiality remains a trust assumption distinct from the weighted verdict assumption. Public ciphertext also remains exposed to future key compromise. Authorized archival nodes must retain complete chain evidence for replay; chain inclusion is not a promise that every public RPC retains historical payloads forever.

## Stage transitions and certificates

An authenticated decline included by the stage's publication cutoff is terminal
for that stage. A later payload publication cannot turn it into success; retain
the bytes as evidence and reject any acceptance certificate that ignores the
decline. A late decline cannot reopen closed accounting. The next fallback, if
required, still starts at its reserved time.

A stage identifier is `(protocol_hash, ticket_id, interface, role, attempt)`. The ticket fixes an A requester, prescribed B providers and the authorized rewrite task; it also pins the lifetime requester index, current registration generation, nonce, limits, keys, measurement epoch, and all deadlines. No requester's chosen submission block changes its assignment. The first valid manifest fixes the stage bytes. Byte-identical retries are idempotent; a different signed manifest cannot overwrite it and is retained as conflicting evidence. Malformed transactions do not reset the deadline or earn an extra attempt. Certificate evidence includes the complete canonical records through the stage publication cutoff; later revelations remain separate records and cannot silently rescore a closed stage.

| Stage condition at its fixed cutoff | Terminal result | Consequence |
| --- | --- | --- |
| Responsible sender explicitly declines, or complete authenticated DATA/witness publication is absent | `FAILED_PUBLICATION` | One attributable failure, subject only to the fixed platform-incident rule |
| Complete timely publication and a valid global `ACCEPT` certificate | `SUCCEEDED` | One settled success; valid request can activate its first provider |
| Global `REJECT_SENDER` certificate names authorization, schema, commitment, recipient-encryption, or conflicting-manifest failure | `FAILED_VALIDITY` | One sender failure; invalid request never activates its provider |
| Complete timely publication but no acceptable global validity verdict | `UNRESOLVED` | No success or attributed failure; stop dependent work and block the affected availability class |
| Applicable global platform-incident certificate | `PLATFORM_VOID` | Preserve the record and incident scope; no miner success/failure credit for the voided work |

A vote/certificate binds the full snapshot hash, stage, DATA and witness digests, evidence cutoff, verdict, reason code, and accounting scope. Quality certificates additionally bind each required check's verdict and the resulting exact task gate. Signatures over an evidence root without the verdict are insufficient. Validators wait for the publication cutoff to become final before signing its complete evidence. The contract counts signatures only from the frozen snapshot; an honest validator persists its signing lock across crashes and key changes. Invalid, stale, or conflicting certificates never default to acceptance. A second valid conflicting quorum is a safety incident: halt affected settlement rather than choosing whichever certificate a relayer supplies first.

The requester obligation activates on issue, including abandonment of an unfavorable assignment. The provider obligation activates at the fixed start only if the input has a timely global `ACCEPT`. An input rejection or unresolved input leaves provider reservations unactivated. Once activated, a provider must publish through its fixed response deadline. Failure permits only the next prescribed provider at its already reserved start. A validity dispute without an attributable failure does not authorize fallback. The request and commitment remain fixed across fallbacks. A later successful provider never erases an earlier provider's failure. Success cancels unused reservations; three failed attempts exhaust the ticket.

Late publication remains late even if its contents are valid. Recipient accusations and unrecorded private messages establish no terminal result. Missing or malformed witness publication is objectively distinguishable from a published witness that a validator cannot decrypt; the latter needs a global verdict or closes unresolved. The verifier's own missing key or local outage is not evidence of sender fault. Quality review stays separate: every scored submission receives all required checks, three preparers retain their commitments/openings and disagreements, and the global strict-weight certificate decides the final verdict. Insufficient candidate-specific quality evidence by `H+204` withholds only that candidate's assigned credit, retaining its unknown judgment and fixed weight. Complete competitors keep credit. Certified shared failures affect the full prescribed scope; neither outcome deletes attributable service failures. Native weight commit/reveal is a separate mechanism and adds no assignment beacon.

An A requester's publication failure remains a service-accounting event; it
cannot change the frozen benchmark artifact or its canonical result. Validator
runner failures never enter the rewrite-service ledger. Missing service-validity
certificates do enter it as unresolved obligations and block epoch eligibility
even without miner fault. If every miner is ineligible, the pool follows the
unallocated/burn policy. B quality scoring uses the committed candidate and does not
require replay of private generation. The mailbox deadlines and byte limits do
not establish that artifact distribution, canonical inference or full-matrix
execution fits the benchmark budget.

## Deterministic accounting and platform failure

For each applicable A-requester or B-provider class, retain activated obligations
`A=S+F+U+V`: certified successes, attributable failures, unresolved obligations,
and platform voids. Do not count inactive fallback reservations. Set settled
valid obligations `N=S+F`. The class passes availability only if `U=0`, `N>0`,
and `10F<=N`. Publish all counts, including `U` and `V`; excluding them from
settled `N` never hides missingness or gives automatic approval. Apply the same
test in aggregate and to each applicable class. A artifact publication and
qualification have separate eligibility gates. Private customer successes and
other classes cannot dilute these counts.

Update the persistent class deficit exactly once at close: `D=max(0,D_previous+10F-N)`. Require zero outstanding deficits, current class gates, and qualification before normal admission. Unresolved and platform-void work cannot retire a deficit. An epoch with no settled work leaves it unchanged. A closed unresolved class receives no emission eligibility that epoch; later recovery uses new scheduler-assigned probation work, not retroactive relabeling of the missing evidence. Dropping a role or changing keys within a registration does not erase its deficit.

Only successful transaction inclusion in the canonical finalized chain counts. Equality at a cutoff is timely; observations become terminal only after the relevant cutoff block is final. Block production and GRANDPA finality are separate in [Subtensor's consensus](https://www.bittensor.com/docs/concepts/chain-consensus). A local timeout, mempool receipt, failed transaction, ordinary fee underpayment, or allegation of censorship is not proof of publication or a platform exception. The normal rule assumes fair transaction inclusion and sufficiently prompt finality for the reserved windows; it does not prove either.

A finality stall pauses observation of final accounting, not the published block-height cutoffs. A global strict-weight incident certificate must identify the affected chain interval, evidence, and every intersecting scheduled ticket/comparison. It must be included by `H+204`; its containing block may finalize later. The manifest's rule applies to all activated obligations in those tickets, including successes and failures; it cannot name only favorable miners or pardon selected failures. Already closed epochs stay closed. No coordinator or three-preparer group can issue this certificate, shrink `W`, extend deadlines, or redraw peers. Without a timely certificate, validators apply the normal inclusion rule; missing global verdicts remain unresolved and the affected quality comparison cannot settle as a pass. This rule cannot guarantee relief from selective censorship or a stall that prevents timely certificate inclusion. The incident mechanism is a declared quorum trust boundary, not a cryptographic proof of censorship or elapsed wall-clock service time.

The pilot may store full ciphertext in transaction data/events with on-chain completion metadata, rather than permanent byte-for-byte contract storage. Gas, complete history retrieval, snapshot proofs, signature verification, byte reservations, and native settlement timing must be demonstrated on the selected runtime before issue. [Subtensor EVM support](https://www.bittensor.com/docs/guides/evm) permits a contract design; it does not prove this workload fits. [Current-state precompile views and project relays](https://www.bittensor.com/docs/guides/evm/precompile-design) also do not make a submitted historical snapshot consensus-authenticated. There are no live-chain or detector results for this proposed mailbox.
