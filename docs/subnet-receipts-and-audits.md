# Encrypted Pangram receipts and assigned audits

Proposal, 2026-09-09. Miners keep revisions and provider report links encrypted
for assigned validators. Public records contain salted commitments and encrypted
deliveries. The [report reader](../examples/verify_public_pangram.rs) checks a
retrieved provider response. A [Rust encryption prototype](receipt-envelope.md)
now signs recipient keys, encrypts exact payloads and verifies commitment
openings against caller-supplied assignments. Chain registration, deterministic
assignment and persistent replay accounting remain unimplemented.
Start with licensed public sources, while keeping each
new transformation private within the authorized evaluation group.

This is the benchmark evidence flow. Under the [whitepaper PDF](../whitepaper/slop_ninja.pdf)
([LaTeX source](../whitepaper/main.tex)),
customer requests are encrypted only for the assigned miner and do not enter
these audits. Customers can separately send evidence to one chosen validator;
that requires a customer-directed packet with its own recipient and scope.

1. Freeze qualified epoch rosters from completed actual request records, global
   quotas, capacities, canonical counter schedules, and fixed block deadlines
   for candidates, evidence, review commitments/openings, disputes and weights.
   Register separate X25519 keys signed by current Bittensor hotkeys, binding
   network, registration generation, epoch, key ID and expiry. Verify against
   finalized chain state; signing keys are not encryption keys.

2. Derive training counterparts from the next lifetime requester-UID-slot index
   by canonical `SHA256(domain, chain, netuid, requester_UID, index, peer_UID)`.
   Exclude the exact requester UID; the first peer is mandatory and later
   ranked entries are fixed fallbacks. Pin the roster root in the ticket but
   exclude it from the rank. No skipping, counter reset, nomination or reroll
   is allowed. Retries reuse the same record, index and quota. Public metadata
   permits assignment checks; text and salted openings remain private.

3. A separate shared comparison counter selects a common A panel and B batch.
   Include the strongest qualified prior-round A detector and two distinct
   hash-ranked A UIDs, then hash-rank B entries outside the panel. Filter the A
   reserve ranking against all B batch UIDs and the initial panel. Freeze that
   residual order and capacity before issuing tickets; insufficient capacity
   prevents issuance without a new B draw. Freeze all
   settings and reserve capacity for the complete common source/candidate
   matrix. Only exact UID exclusions are proved; ownership may be shared.
   Every B entry uses identical sources, briefs, modes and detector settings.

4. Give permitted text and constraints only to authorized recipients. Commit
   the question pool and source evidence. B miners commit one final canonical
   payload containing source/candidate binding, exact text, Pangram report
   IDs/URLs, score projections and raw-response hashes by the fixed deadline.
   Publish only `SHA256(domain || salt || encoded_payload)` with a fresh
   secret 32-byte salt and unambiguous encoding. Keep the salt encrypted;
   unsalted text and score hashes permit guessing attacks.

5. Obtain the panel's required signed response matrix after B commitments,
   binding service/settings, nonce, text commitment and full probabilities.
   Conceal text-to-B-UID mappings and source/candidate roles through a grading
   relay with mixed human/model controls. A fixed replacement recomputes every
   affected source and candidate cell for the batch, or voids the comparison.
   Fallback stages have reserved capacity and minimum response windows. Missing
   actual strongest evidence prevents a strongest-detector pass. Disagreement
   alone does not justify replacement or establish fraud. Commit all evidence
   by the fixed evidence deadline.

6. Review every scored submission and all required checks at launch. Hash-rank
   eligible validator UIDs using a separate review domain, chain, subnet,
   shared comparison index, submitting UID and reviewer UID, with UID
   tie-breaking. Assign three distinct reviewers and fixed reserves, excluding
   exact miner UIDs involved in the comparison. The schedule is predictable;
   there is no hidden checker, audit sample or custom drand dependency.
   Common ownership and dishonest review remain possible.

7. Encrypt the complete payload and salt separately to assigned validators.
   The existing prototype uses HPKE Base with DHKEM(X25519, HKDF-SHA256),
   HKDF-SHA256 and ChaCha20Poly1305, plus a miner-hotkey envelope signature.
   Use fresh encapsulation per recipient. The proposed assignment binding
   includes opaque task ID, commitment, comparison index, recipient/key ID,
   registration generation, epoch, expiry and protocol version in HPKE
   `info`/AEAD associated data and the signature. This binding still requires
   implementation and does not change the existing prototype schema.
   Associated data is visible; keep report URLs, text and scores encrypted.
   Verify signatures, assignments and keys before decryption, then verify the
   commitment opening. Enforce replay and size limits. HPKE provides no general
   replay protection or recipient-compromise forward secrecy.
   [RFC 9180](https://www.rfc-editor.org/rfc/rfc9180.html),
   [Rust HPKE API](https://docs.rs/hpke/latest/hpke/).

8. Fetch report IDs from the allowlisted provider over HTTPS. Verify exact
   analyzed text, version, time window and all three fractions. Preserve raw
   JSON separately from the versioned score projection. Verify panel signatures
   and source/candidate bindings, then review the entire revision for claims,
   quantities, negation, uncertainty, attribution and readability. Reviewers
   commit before peer openings; all openings and disputes stay encrypted to
   authorized reviewers. Missing reviews cannot default to approval. Resolve
   disagreements before weight deadlines, retaining every observation.

Panel reward uses the median of three paired detector improvements, with
Pangram improvement separately nonnegative. Strict joint success additionally
requires preservation, Pangram below 10%, a panel majority below their frozen
calibrated thresholds and a pass against the actual strongest service.
[Scoring details](subnet-mechanisms.md#2-author-transformation-and-detector-evasion).

Required A/B services consume bounded mandatory allocations free to requesters;
serving miners bear their costs from expected emissions. Full benchmark matrix
and review reservations are separate from private training/search indices.
Tickets bind UID/index, generation, roster, peers, authorization, sizes, nonce
and deadlines in one replay-protected ledger. Counter state persists through
UID reuse, but new owners do not inherit old performance records. Issued
requester abandonment consumes the index and counts against the requester;
a provider cannot fail on a payload never validly delivered.

Each obligation binds the UID and requester/provider role. For measured `N > 0`
valid obligations and `F` failures, `10*F <= N` passes
availability. More than 10% failures zeros earned A+B epoch credit. Require
this gate in aggregate and separately for mandatory service on each committed
interface and actually assigned role; paid/cheap replies or outgoing payloads
cannot dilute provider failures. Zero
workload is no automatic pass. Exclude duplicates, unsolicited or malformed
requests and impossible deadlines; retain independent delivery evidence and
disputes. A complaint alone cannot prove failure. Eligibility uses closed
prior request windows plus bounded real probation assignments, without
heartbeats. Close measurement before weight deadlines and demonstrate the
native payout lag; already distributed emissions cannot be clawed back.
Retain each mandatory interface/role's recovery deficit from an initial zero:
`D_e = max(0, D_previous + 10*F_e - N_e)`. Normal eligibility also requires zero
deficit; only same-class protocol probation work can retire it. It does not age
out or reset through key/service-version rotation, and zero workload leaves it
unchanged. Retain the supporting signed task and delivery records.

Withhold detailed active-test evidence from revision miners until retirement;
each panel service knows its own response. Publish only necessary scheduling
metadata and reviewed aggregates. Keep provider locators, text and scores out
of logs and public artifacts. Encryption does not hide all identities or
timing, prove endpoint nondisclosure or establish private model execution.

Miners pay for private queries beyond those tickets and required Pangram
candidate reports. Source reports use the capped allowance in the
[oracle funding proposal](pangram-oracle.md#who-pays). Observed report GETs need
no API key or new inference charge; validators fund hosting and review.
The proposed subnet requires three distinct completed reports per mandated text;
three reads of one report remain one observation. Miners may pay for additional
attempts and select their reports before commitment. Scoring uses only the
committed set; miners need not disclose all private attempts, and validators
need not purchase fresh inference. Selection from undisclosed attempts can bias
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
publicly identifiable unreviewed subset; it does not guarantee that reviewers
detect every defect. Later sampling requires a separate security/cost argument.

Weight commitments follow scoring, separately from audit-answer commitments.
Use the supported SDK/runtime timelock and epoch schedule, including any native beacon dependency, rather than
hard-coding CRv4 timing. Commit-reveal delays weight visibility, without proving
audit correctness. Chain weights eventually become public; individual evidence
and provider links remain encrypted.
[Bittensor commit-reveal](https://www.bittensor.com/docs/hyperparameters/commit-reveal-weights-enabled),
[epoch scheduling](https://www.bittensor.com/docs/hyperparameters/commit-reveal-period).
