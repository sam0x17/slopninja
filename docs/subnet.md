# Possible Bittensor subnet

Text revision is a plausible miner task: a miner receives a passage and returns a revision under an explicit preservation and cost contract. Validators would judge the result and submit miner weights. Bittensor's Yuma Consensus combines validator assessments; the chain does not supply a prose-quality evaluator. See the [official consensus documentation](https://www.bittensor.com/docs/internals/consensus). The hard part for this project is developing an affordable, repeatable evaluator whose rewards match the desired writing quality.

This is a design proposal. There is no subnet code, registration, wallet setup, or deployment in the current experiment.

## Proposed task contract

A challenge should contain a unique nonce, the source text, its language and register, the intended audience and tone, a length tolerance, protected quotations/citations, and a deadline. Validators should retain a private inventory of claims, qualifications, argument relations, and details. Miners return the exact revised text, a source-to-revision change map, the transformation/model version, latency, and declared resource use. Validators must assess the text independently of those declarations.

Miners could run rule-based edits, prompted models, trained models, or combinations. Keep the protocol independent of a particular implementation. Before building network infrastructure, run the same contract locally against several competing baselines and measure validator disagreement.

## Evaluation and rewards

Apply quality requirements before rewarding detector performance:

1. Reject missing or altered claims, quotations, numbers, and citations. Use deterministic checks for exact invariants, semantic review for the argument, and sampled human audits for disputed cases.
2. Require a blinded readability and tone assessment relative to the original. Do not reward shorter text merely for being shorter. Use multiple independent judges and audit disagreement; one model's preferred style is not the definition of readable prose.
3. Measure the detector target on the same normalized reading text, under fixed versions and repeated calls. Keep the raw output alongside normalization so invisible-character tricks, duplicated text, or malformed formatting do not earn credit.
4. Score latency and cost only among revisions that pass preservation and readability requirements. Cap the number of candidates and detector queries allowed per challenge.

A possible initial objective is the pass rate on unseen source groups, where a pass requires all quality gates and an upper-tail repeated detector score below 10%. Fix the AI/assisted interpretation and aggregation rule in advance. Choosing the minimum of many detector runs would reward luck and excessive queries. This objective is our proposed experiment, not a feature supplied by Bittensor.

## Resistance to reward manipulation

Use private, rotating challenge families, fresh assignments, and held-out domains. Keep related excerpts in one family so miners cannot train on a neighboring paragraph and appear to generalize. Include human-written passages that should need little editing, deliberately poor rewrites, omissions disguised as summaries, and changes to modal verbs or causal claims.

Cache identical text/version detector results for cost control, while reserving a random repeated subset to measure detector variability. Bind submissions to challenge nonces and deadlines to reject replay. Limit access to detailed validator feedback on live holdouts; release separate development tasks for improvement. Monitor copied outputs and near duplicates, but distinguish a legitimate shared solution from evidence of task leakage.

Validators should log the original, candidate, evaluator versions, every detector observation, quality judgments, and resulting reward. Publish enough audit material to investigate inconsistent scoring without releasing the next private challenge set. An external detector creates a recurring query cost and a dependency on its availability and revisions; evaluate alternative detectors and an outage policy before relying on it for continuous rewards.

## Decision before deployment

Proceed only after a local benchmark shows preservation, readability improvement, and repeatable detector performance on unseen sources at an acceptable evaluation cost. Quantify disagreements between validators and how often human review is needed. A miner market is useful only if validators can distinguish better revisions reliably enough to reward them.

At deployment time, recheck the current requirements for permits, weights, and emissions. Bittensor documents these as part of its [emissions and epoch process](https://www.bittensor.com/docs/concepts/emissions); they are operational constraints, not evidence that this task will be economical. The present repo makes no revenue or token-value assumption.
