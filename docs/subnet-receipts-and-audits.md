# Encrypted Pangram receipts and assigned audits

Public A/private B design, 2026-09-10. A miners publish complete immutable
inference artifacts; validators execute them for benchmark probabilities.
B miners keep generation private. Revisions and provider report links are
encrypted for every validator in the round's frozen certifying set. Public records
contain salted commitments and complete encrypted deliveries. The
[mandatory training-service evidence contract](service-evidence.md) specifies the
mailbox, generation witnesses, strict weighted certificates, and closure.
The [report reader](../examples/verify_public_pangram.rs) checks a
retrieved provider response. A [Rust encryption prototype](receipt-envelope.md)
now signs recipient keys, encrypts exact payloads and verifies commitment
openings against caller-supplied assignments. Chain registration, deterministic
assignment, generation-witness verification, and persistent replay accounting
remain unimplemented. Start with licensed public sources whose authorization
covers every frozen certifier, while keeping each new transformation private
within that authorized evaluation group.

This is the benchmark evidence flow. Under the [whitepaper PDF](../whitepaper/slop_ninja.pdf)
([LaTeX source](../whitepaper/main.tex)),
B customer requests are encrypted to the verified key inside their qualified
runtime and do not enter these audits. The miner receives no input key or
plaintext output copy. Ordinary A hosting requires explicit operator-access
terms. Customers can separately send evidence to one chosen validator;
that requires a customer-directed packet with its own recipient and scope.

1. Freeze qualified epoch rosters from completed actual request records, global
   quotas, capacities, canonical counter schedules, and fixed block deadlines
   for candidates, evidence, preparer commitments/openings, certificates and
   the declared native settlement cycle. Use the actual pilot values in
   [service-evidence-v1](service-evidence.md#frozen-authority-and-parameters).
   Register separate X25519 keys signed by current Bittensor hotkeys, binding
   network, registration generation, epoch, key ID and expiry. Verify against
   finalized chain state; signing keys are not encryption keys. Before task
   disclosure, exclude the exact A/B participants from the round's certifying
   set and freeze all remaining native effective-consensus-stake integers,
   their sum `W`, generations and keys. Verify the finalized snapshot and its
   runtime/storage schema by state proof or a chain-verified native snapshot.
   A relayer's signature alone cannot establish those records.

2. Derive B training-service providers for each A requester from its next
   lifetime UID-slot index
   by canonical `SHA256(domain, chain, netuid, requester_UID, index, peer_UID)`.
   Exclude the exact requester UID; the first peer is mandatory and later
   ranked entries are fixed fallbacks. Pin the roster root in the ticket but
   exclude it from the rank. No skipping, counter reset, nomination or reroll
   is allowed. Retries reuse the same record, index and quota. Public metadata
   permits assignment checks; text and salted openings remain private.

3. A separate shared comparison counter selects a common public A panel and B batch.
   Include the strongest qualified prior-round A detector and two distinct
   hash-ranked A UIDs, then hash-rank B entries outside the panel. Qualify,
   freeze and cache complete artifacts before B task disclosure or generation;
   reserves apply only before issue. Require distinct UIDs and inference-content
   hashes excluding owner/signature metadata. Identical content receives one
   credit entry and panel seat, using earliest finalized accepted commitment
   and canonical UID tie-break. Near copies and shared ownership remain possible.
   Freeze execution settings and reserve validator storage and replay capacity
   for the complete common source/candidate matrix. Publication rights and
   availability are admission requirements. Only exact UID exclusions are proved.
   Every B entry uses identical sources, briefs, modes and detector settings.

4. Give permitted text and constraints only to authorized recipients. Commit
   the question pool and source evidence. B miners commit one final canonical
   payload containing source/candidate binding, exact final text, assigned A
   panel artifact hashes, reference settings and seed schedule, full candidate
   origin-probability vectors, required source baselines, scalar projections,
   Pangram report IDs/URLs and raw-response hashes by the fixed deadline.
   The full-batch A panel is pinned before B task disclosure. B cannot choose
   an easier model, omit a required cell or use a score for another text or seed.
   Use the service-evidence contract's versioned deterministic encoding and
   fresh secret 32-byte salt. Publish the salted commitment and complete DATA
   ciphertexts, never plaintext or unsalted text/score hashes. The DATA
   manifest fixes bytes before the separate confidential witness packet is
   constructed; hashes alone cannot complete delivery.

5. Validators execute the frozen A artifacts on the complete source/candidate
   matrix after B commitments. Bind artifact hashes, reference runner/settings,
   exact text commitments and full probabilities. Compare the canonical outputs
   and projections with B's committed self-scores. Claimed scores, hashes or
   signatures do not verify the computation; validators still incur the full
   A forward-pass workload. Every credited B entry separately requires its
   verified qualified-runtime/model execution receipt; no private B weight
   download or validator generation replay is required.
   Preservation `G`, public style `V` and Pangram evidence remain separate.
   The isolated runner receives
   no undeclared UID, hidden-label or source/candidate-role inputs. An A endpoint
   response cannot supply an authoritative probability. No artifact or threshold
   changes after issue: a validator-node outage uses another approved runner
   with the same artifact. Candidate-specific reference failure withholds only
   that entry's credit; certified shared failure affects every entry requiring
   that evidence. Keep assigned weights. Neither outcome implies B nonresponse
   or an automatic pass. Missing actual
   strongest evidence prevents the corresponding strict claim. Commit all
   execution evidence by the fixed deadline.

6. Review every scored submission and all required checks at launch. Hash-rank
   eligible validator UIDs using a separate review domain, chain, subnet,
   shared comparison index, submitting UID and candidate preparer UID, with UID
   tie-breaking. Assign three distinct evidence preparers and fixed reserves
   from the already frozen eligible set. They organize evidence and retain
   independent checks; they cannot approve final validity or quality. The
   schedule is predictable;
   there is no hidden checker, audit sample or custom drand dependency.
   Common ownership and dishonest review remain possible.

7. Encrypt the complete evidence and salt separately to every frozen certifier.
   The existing prototype uses HPKE Base with DHKEM(X25519, HKDF-SHA256),
   HKDF-SHA256 and ChaCha20Poly1305, plus a miner-hotkey envelope signature.
   Use fresh encapsulation per recipient. The service-evidence assignment
   binding includes protocol, chain/contract, ticket/stage/attempt, commitment,
   sender/recipient generations and key versions in HPKE `info`, authenticated
   additional data, and the signature. After DATA is fixed, publish separate
   validator-encrypted witness packets containing its digest, plaintext, salt,
   and each DATA envelope's ephemeral generation material. A validator must
   reproduce every required miner/requester DATA copy, not merely decrypt its
   own evidence. Witness envelopes do not include their own generation secrets;
   DATA does not refer back to a witness hash. These bindings and witness checks
   still require implementation and do not change the existing prototype.
   Associated data is visible; keep report URLs, text and scores encrypted.
   Verify signatures, assignments and keys before decryption, then verify the
   commitment opening, complete ciphertext publication, and finalized inclusion
   proof. Enforce replay and size limits. HPKE provides no general
   replay protection or recipient-compromise forward secrecy.
   [RFC 9180](https://www.rfc-editor.org/rfc/rfc9180.html),
   [Rust HPKE API](https://docs.rs/hpke/latest/hpke/).

8. Every validator that signs must inspect the actual evidence. Fetch report
   IDs from the allowlisted provider over HTTPS. Verify exact
   analyzed text, version, time window and all three fractions. Preserve raw
   JSON separately from the versioned score projection. Reproduce the public A
   panel and verify artifact, runner, input and output bindings, then review the entire revision for claims,
   quantities, negation, uncertainty, attribution and readability. Preparers
   commit before peer openings; all openings and disputes stay encrypted to
   every frozen certifier. A final certificate binds the exact verdict,
   snapshot, stage and evidence, with distinct signer weight `w` satisfying
   strict `3w > 2W`. Assume dishonest weight below `W/3` in this conditional
   set. Preparer signatures or a certificate over an evidence hash alone are
   insufficient. Missing keys, nonresponse, recusal, or a replacement preparer
   never shrink `W`. Retain every observation and use the
   [fixed certificate/closure rule](service-evidence.md#stage-transitions-and-certificates)
   for disagreement or missing evidence; neither defaults to approval.

Panel reward uses the median of three paired detector improvements, with
Pangram improvement separately nonnegative. Strict joint success additionally
requires preservation, Pangram below 10%, a panel majority below their frozen
calibrated thresholds and a pass against the actual strongest artifact.
[Scoring details](subnet-mechanisms.md#2-author-transformation-and-detector-evasion).
Pangram is an external origin check; it cannot replace author-specific A scores
or preservation review. Public A replay does not establish accuracy or owner
independence: a committed model may favor an allied B model. Hidden-label and
targeted collusion tests remain necessary. Standard A/B native pools remain;
experimental shared settlement is not adopted.

Mandatory training-service tickets run only from A requesters to B providers
for authorized rewrites. These consume bounded allocations free to requesters;
B providers bear those costs from expected emissions. Validators reserve and
bear full benchmark A execution and review
costs separately. B can execute public A locally on its own candidates at its
own compute cost, without paying Pangram for that feedback. Such local feedback
does not replace required external reports. A publishes artifacts without a
mandatory inference endpoint; endpoint service does not qualify its artifact.
Artifact downloads and B's local A execution create no detector-call tickets.
Authorized retired B outputs may feed later A training. Optional paid A hosting
uses separate customer terms, encrypted assigned-provider delivery and optional
chosen-validator inspection. Training-service
tickets bind UID/index, generation, roster, peers, authorization, sizes, nonce
and deadlines in one replay-protected ledger. Counter state persists through
UID reuse, but new owners do not inherit old performance records. Issued
requester abandonment consumes the index and counts against the requester;
a provider cannot fail on a payload never validly delivered.

Each obligation binds the UID and its actual mandatory A-requester or B-provider
role. The requester obligation begins at issue; a B provider activates only
after timely globally certified valid input at its fixed start. Retain
activated `A=S+F+U+V`, separating success,
attributable failure, unresolved and platform-void records; inactive fallback
reservations are separate. Set settled `N=S+F`. Require `U=0`, `N>0`, and
`10*F<=N` for availability. More than 10% failures zeros earned A+B epoch credit.
Require
this gate in aggregate and separately for each actually assigned A-requester
or B-provider class. A-provider and B-requester detector-call classes do not
exist and incur no zero-work penalty. Paid hosting replies or outgoing payloads
cannot dilute provider failures. An actually assigned class with no settled work
receives no automatic service pass. Unsolicited traffic and duplicate retries create
no new obligation. Invalid requester publication cannot activate a provider,
but the issued requester obligation still closes under the fixed rule.
Impossible windows prevent issue. Retain complete publication, certificate,
and dispute evidence; a complaint alone cannot prove failure. Eligibility uses
closed
prior request windows plus bounded real probation assignments, without
heartbeats. Close measurement before weight deadlines and demonstrate the
native payout lag; already distributed emissions cannot be clawed back.
Retain each actual A-requester or B-provider class's recovery deficit from an initial zero:
`D_e = max(0, D_previous + 10*F_e - N_e)`. Normal eligibility also requires zero
deficit; only same-class protocol probation work can retire it. It does not age
out or reset through key/service-version rotation, and zero workload leaves it
unchanged. Unresolved/void records supply no deficit-recovery credit, and `U>0`
blocks the affected class. A successful fallback leaves earlier provider
failures intact. Retain all supporting records. Use the
[finality and platform-incident rule](service-evidence.md#deterministic-accounting-and-platform-failure):
strict global certificates must cover the full prescribed incident scope;
local timeouts or accusations cannot selectively pardon failures or extend
deadlines. Private customer work never enters mandatory `N` or `F`.

Keep hidden labels, unreleased sources, other miners' candidates and
preservation adjudication restricted until retirement. Public A scores on text
B possesses can be computed by B at any time. Publish necessary scheduling
metadata, ciphertexts and reviewed aggregates. Keep provider locators, text and
scores out of plaintext logs and public plaintext artifacts. Every frozen
certifier is authorized to inspect active evidence; no preparer can widen a
customer's separate chosen-validator disclosure. Encryption does not hide all
identities or
timing or by itself establish protected execution. B emissions and mandatory
rewrites additionally require the qualified-runtime/version receipt.

Miners pay for private queries beyond those tickets and required Pangram
candidate reports. Source reports use the capped allowance in the
[oracle funding proposal](pangram-oracle.md#who-pays). Observed report GETs need
no API key or new inference charge; validators fund hosting and review.
The proposed subnet requires three distinct completed reports per mandated text;
three reads of one report remain one observation. Miners may pay for additional
provider calls for the same final text and select reports before commitment.
Generation and candidate selection must stay inside the qualified invocation.
Scoring uses only the committed report set; miners need not disclose every
provider call, and validators need not purchase fresh inference. Selection can bias
that set toward favorable results; a pass applies to those committed
observations and does not estimate fresh-query success probability. The
[repeatability probe](pangram-repeatability.md) covers three texts through one
account and supplies no determinism evidence near the 10% boundary.

The observed `web.pangram.com/api/history/<id>/` endpoint serves text and results
without authentication. Encrypting its URL prevents disclosure through our
transport; any validator receiving it can still share it, and Pangram knows the
text. Provider access remains public. Stronger privacy needs authenticated
private reports or a provider-supported proof/receipt route. The undocumented
web endpoint needs a stability and retention agreement. Its observed `model_id`
is null, and it removed a terminal newline. Require exact text equality;
normalization needs a versioned policy. Timestamps do not prove immutability,
and no signed or unique canonical result per text was established.

The launch adds no external beacon wait. Predictable full review avoids a
publicly identifiable unreviewed subset; it does not guarantee that validators
detect every defect or establish the below-one-third dishonest-weight bound.
Fund every signer's actual inspection as well as preparation and publication.
Later sampling requires a separate security/cost argument.

Weight commitments follow scoring, separately from audit-answer commitments.
Use the supported SDK/runtime timelock and epoch schedule, including any native beacon dependency, rather than
hard-coding CRv4 timing. Commit-reveal delays weight visibility, without proving
audit correctness. Chain weights eventually become public; individual evidence
and provider links remain encrypted.
[Bittensor commit-reveal](https://www.bittensor.com/docs/hyperparameters/commit-reveal-weights-enabled),
[epoch scheduling](https://www.bittensor.com/docs/hyperparameters/commit-reveal-period).
