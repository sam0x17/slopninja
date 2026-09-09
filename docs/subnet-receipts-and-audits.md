# Encrypted Pangram receipts and random audits

Proposal, 2026-09-09. Miners keep revisions and provider report links encrypted
for assigned validators. Public records contain salted commitments and encrypted
deliveries. The [report reader](../examples/verify_public_pangram.rs) checks a
retrieved provider response; encryption, assignment and on-chain commitments
are not implemented. Start with licensed public sources, while keeping each
new transformation private within the authorized evaluation group.

1. Publish the protocol, opaque task IDs, deadlines, scoring rules and auditor
   roster. Before assignments, validators register separate X25519 encryption
   keys in records signed by their Bittensor hotkeys, binding network, hotkey,
   epoch, key ID and expiry. Verify signatures and registration against finalized
   chain state. Never reinterpret a Bittensor signing key as an encryption key.
   Freeze the eligible key records before beacon selection.

2. Give source text and constraints only to authorized miners. Fix the question
   pool, source answers and evidence before miners commit; publish a salted
   commitment root, with questions kept private. The miner prepares a canonical,
   versioned payload containing task/source binding, exact source and candidate,
   report IDs/URLs, required score projections and raw-response hashes. Publish
   only `SHA256(domain || salt || encoded_payload)`, with a fresh secret
   32-byte salt and unambiguous encoding. Transmit the salt only inside encrypted
   deliveries. Unsalted text or score hashes permit guessing attacks. Allow one
   final commitment per assignment, finalized before the fixed future beacon.

3. Verify that beacon and derive a domain-separated seed binding protocol,
   epoch, submission commitments, roster, chain, round and signature. Sample
   submissions, questions and overlapping auditors without replacement using
   unbiased integer sampling. Question owners deliver selected questions and
   inclusion proofs only to assigned auditors, through encrypted channels.

4. After assignment, encrypt the complete payload and salt separately to each
   assigned validator's authenticated key. Proposed suite: HPKE Base mode with
   DHKEM(X25519, HKDF-SHA256), HKDF-SHA256 and ChaCha20Poly1305, plus a
   miner-hotkey signature over the envelope. That signature supplies sender
   authentication absent from Base mode. Use fresh encapsulation randomness
   per recipient. Bind opaque task ID, commitment, beacon round, recipient/key
   ID, epoch, expiry and protocol version in HPKE `info`/AEAD associated data
   and the signature. Associated data is visible; keep URLs, text and scores
   inside ciphertext. Verify sender signature, assignment and key validity
   before decryption; require the plaintext to open the prior commitment.
   Enforce replay tracking, delivery deadlines and size limits. HPKE supplies
   no general replay protection or recipient-compromise forward secrecy.
   [RFC 9180](https://www.rfc-editor.org/rfc/rfc9180.html),
   [Rust HPKE API](https://docs.rs/hpke/latest/hpke/).

5. Validators independently fetch decrypted report IDs over HTTPS from the
   allowlisted provider path. Check exact text, version, timestamp window and
   fractions. The private score projection binds its schema, task/report IDs,
   text hash, detector version and all three document fractions. Preserve raw
   JSON separately; incidental notes, credits and key order do not define score
   equality. Keep URLs, IDs, text, evidence spans and individual scores out of
   public logs, errors and artifacts. Publish only necessary metadata and
   disclosure-reviewed aggregates. Padding to fixed size buckets limits length
   leakage; encryption does not hide participant identities or traffic timing.

6. Auditors commit salted answers before seeing peers' answers. Answer openings,
   evidence and appeals remain encrypted for authorized reviewers; public
   commitments never require public plaintext disclosure. Check roles,
   quantities, negation, qualifications, attribution and quotations. Preserve
   disagreement and unknowns: sampled answers depend on question coverage and
   human or model judgment. Define delivery/response deadlines, nonresponse
   consequences and disputes in advance. Missing work stays in the denominator;
   no custom chain slashing is assumed.

Miners pay for private queries and every required report. Observed report GETs
need no API key or new inference charge; validators fund hosting and review.
The existing offline three-distinct-observation policy is unchanged. If retained
for the subnet, require three distinct completed reports per mandated text;
three reads of one report remain one observation. Receipts cannot reveal private
retries or establish that a miner supplied its first paid result.

The observed `web.pangram.com/api/history/<id>/` endpoint serves text and results
without authentication. Encrypting its URL prevents disclosure through our
transport; any validator receiving it can still share it, and Pangram knows the
text. Provider access remains public. Stronger privacy needs authenticated
private reports or a provider-supported proof/receipt route. The undocumented
web endpoint needs a stability and retention agreement. Its observed `model_id`
is null, and it removed a terminal newline. Require exact text equality;
normalization needs a versioned policy. Timestamps do not prove immutability,
and no signed or unique canonical result per text was established.

Pin drand's chain hash, public key, scheme and exact round; verify every
signature with a maintained client. Commitments must finalize before scheduled
release with a fixed margin. `LastStoredRound` may lag public randomness;
estimated block times cannot prove a round remains unknown. Predetermine
abort/defer behavior, with no favorable fallback round.
[Drand verification](https://docs.drand.love/developer/),
[timing specification](https://docs.drand.love/docs/specification/).

Weight commitments follow scoring, separately from audit-answer commitments.
Use the current SDK/runtime timelock version and epoch schedule rather than
hard-coding CRv4 timing. Commit-reveal delays weight visibility, without proving
audit correctness. Chain weights eventually become public; individual evidence
and provider links remain encrypted.
[Bittensor commit-reveal](https://www.bittensor.com/docs/hyperparameters/commit-reveal-weights-enabled),
[epoch scheduling](https://www.bittensor.com/docs/hyperparameters/commit-reveal-period).
