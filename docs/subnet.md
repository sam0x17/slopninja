# Adversarial text-revision subnet

The [three-task mechanism proposal](subnet-mechanisms.md) extends this direction
to author/origin detection, author transformation and writing improvement. It
describes a future competition; the contract below remains the implemented
offline revision benchmark.

The target service is a miner that revises prose under a preservation contract: retain the argument, details, and intended tone; improve readability; and repeatedly score below 10% AI plus assisted text. Competing miners may use word and grammar statistics, rules, prompted models, trained models, or combinations. We can compare all of them against the same frozen challenges and validator rules.

[src/subnet.rs](../src/subnet.rs) implements the first **offline reference contract in Rust**. It validates challenge/submission bindings and calculates a binary benchmark reward from recorded detector observations and a human quality audit. There is no network server, wallet, registration, signing, or weight submission in this module. Automated semantic preservation is not established.

In Bittensor, validators evaluate miners and submit relative weights; Yuma Consensus combines validator assessments. This makes the quality of our evaluator a separate engineering problem. The chain does not check whether a revision preserved a philosophical qualification or a study result. See the current official [validator guide](https://www.bittensor.com/docs/guides/validating), [weight transaction contract](https://www.bittensor.com/docs/tx/set-weights), and [consensus description](https://www.bittensor.com/docs/internals/consensus). These references were checked on 2026-09-08.

## Rust API

```rust
pub fn validate_submission(
    challenge: &serde_json::Value,
    submission: &serde_json::Value,
) -> anyhow::Result<serde_json::Value>;

pub fn reward(
    challenge: &serde_json::Value,
    submission: &serde_json::Value,
    detector_records: &[serde_json::Value],
    quality_audit: &serde_json::Value,
) -> anyhow::Result<serde_json::Value>;
```

Both functions are pure and perform no requests. Malformed data or mismatched bindings return an error. Missing quality review, too few observations, or a failed quality/detector requirement produces reward `0`. Reward `1` means the supplied evidence passed the offline reference requirements. Every report includes `network_ready: false`, `replay_checked: false`, and `global_budget_verified: false`.

## Challenge

The following is a schema example; replace the hash placeholder with the SHA-256 of the exact UTF-8 source string before validation.

```json
{
  "protocol_version": "unslop-v1",
  "challenge_id": "benchmark-2026-09-08-001",
  "nonce": "fresh-unpredictable-validator-assignment-nonce",
  "source_text": "The exact source text, including its original line endings.",
  "source_sha256": "<sha256 of source_text>",
  "domain": "philosophy",
  "register": "book-preface",
  "tone": "Direct and engaged, with precise qualifications.",
  "group_id": "book-and-source-family-id",
  "split": "test",
  "full_document": true,
  "budgets": {
    "max_candidates": 4,
    "max_detector_queries": 6
  },
  "detector": {
    "name": "pangram",
    "model": "pangram-4",
    "version": "4.0",
    "threshold": 0.1,
    "min_repeats": 3
  },
  "protected_invariants": [
    {
      "id": "qualification-1",
      "requirement": "Preserve the possibility claim without turning it into certainty."
    }
  ]
}
```

The validator freezes the selector and returned detector version before the round. The threshold must be greater than zero and at most `0.10`; the comparison is strictly less than the threshold. At least three distinct observations are required, and the query budget must accommodate them. A changed detector version invalidates the evidence for that challenge instead of silently redefining the benchmark.

Every challenge includes a source-family ID, domain, register, intended tone, and explicit split (`train`, `dev`, `test`, or `exploratory`). The benchmark scheduler must keep related sources in one split. A stateless function cannot discover that two different family IDs actually refer to the same book.

`protected_invariants` is an explicit array, which may be empty. Each entry has a unique `id` and a nonempty `requirement`; additional fields are opaque metadata for the reviewer or a future semantic validator. The current code verifies the inventory's structure and binds its hash to the audit. It does **not** evaluate those requirements automatically. Keep private tests and reference annotations in the validator's local challenge record; do not expose the full record indiscriminately to miners.

## Submission

```json
{
  "protocol_version": "unslop-v1",
  "challenge_id": "benchmark-2026-09-08-001",
  "nonce": "fresh-unpredictable-validator-assignment-nonce",
  "source_sha256": "<same exact source hash>",
  "candidate_id": "miner-candidate-001",
  "candidate_index": 1,
  "revised_text": "The exact revised prose.",
  "revision_sha256": "<sha256 of revised_text>",
  "process_version": "word-grammar-ranker-v1",
  "declared_detector_queries": 0
}
```

`candidate_index` is one-based and must fit `max_candidates`. Hashes cover the exact strings: no trimming, Unicode normalization, or line-ending conversion. The code requires nonempty prose and an explicit process version. It allows an unchanged revision for controls where editing would damage suitable prose, and reports `unchanged_revision` so benchmark analysis can account for that case.

The declared query count is a disclosure, not proof. A miner who admits exceeding the limit fails validation. A declaration of zero earns nothing by itself. The reward function independently checks the supplied observations against the query cap, but cannot see withheld queries, other submissions, or other accounts. Cumulative limits require a validator-controlled ledger and a defined allocation of miner experimentation and validator measurement costs.

## Detector observations and quality audit

Use the complete record format described in [evaluation.md](evaluation.md). Every record must include the exact revision in `submitted_text`, its `sha256`, the expected `detector`, `model`, and response `version`, and `full_document: true`. The validator also attaches the matching `challenge_id`, `nonce`, and `candidate_id`. These annotations bind local evidence; they are not signatures or independent proof that a detector returned the JSON.

The reference scorer rejects duplicate task IDs, mismatched text, version changes, and excess supplied observations. It uses every supplied distinct observation. A successful candidate requires:

```text
number of observations >= challenge.detector.min_repeats >= 3
every observation: fraction_ai + fraction_ai_assisted < threshold <= 0.10
every required quality attestation: true
```

The offline scorer uses every supplied observation; failed or malformed records
cause an error rather than a passing score. Its query-cap fields describe this
legacy contract. The current [whitepaper](../whitepaper/main.tex) instead permits
additional miner-paid attempts before commitment and scores the selected,
committed reports. It requires neither disclosure of all private attempts nor
fresh validator-paid inference. Adapting the offline contract to that report
commitment policy remains implementation work.

The quality audit contains these bindings and literal boolean attestations:

```json
{
  "challenge_id": "benchmark-2026-09-08-001",
  "nonce": "fresh-unpredictable-validator-assignment-nonce",
  "source_sha256": "<exact source hash>",
  "candidate_sha256": "<exact revision hash>",
  "protected_invariants_sha256": "<returned by validate_submission>",
  "reviewed_by_human": true,
  "readability": true,
  "argumentation": true,
  "detail": true,
  "tone": true,
  "protected_invariants_preserved": true,
  "full_document": true
}
```

The inventory checksum uses SHA-256 over this crate's compact `serde_json` serialization of `protected_invariants`, with object keys ordered and array order retained. Use the checksum returned by `validate_submission` to avoid serialization differences. It is a local review binding, separate from any future signed HTTP body.

An empty audit produces reward zero. A nonempty audit with wrong source, candidate, nonce, or inventory bindings is an error. A low detector score with an omitted claim, changed number, lost qualification, or damaged tone must receive a failed quality attestation and reward zero. The function trusts the supplied review; it cannot establish the reviewer's identity or whether the review actually happened.

## Network responsibilities

Calling `validate_submission` twice with identical values returns the same result. This is intentional: the module has no persistent state and cannot reject replay by itself. Before using reference rewards in a live validator, implement:

1. Authenticated miner identity and assignment, exact-body request signing, fresh transport nonces, and durable replay tracking shared across validator workers.
2. A challenge ledger keyed by assignment and miner identity, with deadlines, candidate receipts, cumulative counts, and all detector task IDs. Reserve budget before dispatch; a declared counter cannot prevent multiple submissions from all claiming to be candidate one.
3. Validator-owned detector requests or authenticated evidence, with no miner-controlled filtering of observations. Distinguish resuming an existing task from requesting a new repeat.
4. Private rotating source families, held-out domains, and independent quality audits. Publish development challenges while keeping live holdouts and future assignments private.

Current Bittensor documentation describes a language-independent signed HTTP format that binds the method, target, exact body hash, sender, receiver, and nonce, and requires replay storage. It also supplies Rust implementation guidance and test vectors. That is a suitable transport reference when building the adapter; this module has not implemented it. Our challenge nonce is an application assignment identifier and is separate from the transport nonce. See [signed requests](https://www.bittensor.com/docs/guides/signed-requests).

A production validator would aggregate accepted benchmark outcomes into miner scores, then use the chain's current weight-submission rules. A local reward of `1` is neither a token amount nor a weight transaction. Operational permits, registration, and weight constraints must be checked when the adapter is built. See [set-weights](https://www.bittensor.com/docs/tx/set-weights).

## Making the validator economical

The human audit is the current benchmark reference for testing whether an automatic evaluator agrees with careful reading. Reviewing every submission manually would not support a high-volume service. Before replacing that gate, calibrate a semantic evaluator against blinded human judgments across domains, error types, and held-out source families. Include factual omissions, changed quantifiers, citation damage, fabricated explanations, and superficially fluent but less readable prose. Measure false acceptance, false rejection, disagreement, and evaluation cost.

Possible automatic components include exact quote/number checks, source-to-revision claim alignment, entailment in both directions, and independent readability/tone judges. These are candidates to evaluate; none establishes fidelity merely by being included. Multiple judges may share the same blind spots, and an adversarial miner will search for them. Retain sampled human audits and an explicit dispute path after automation.

Run competing baselines with equal candidate and query budgets, report pass rates on untouched source groups, and account for detector variability and outages. Only then can we assess whether miners can produce useful revisions and validators can reward them consistently at an acceptable cost. The current contract makes that local comparison executable; reliable sub-10% performance and automated semantic validation remain research tasks.
