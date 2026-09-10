# Settlement gameability hypothesis

This is a proposal comparison, not an adopted settlement policy. The Rust
experiment finds that complementary integer credits conserve a jointly owned
A/B pair's allocation under fixed admission, slots and routing. A simplified
native-emissions counterexample shows why that result does not establish
conservation of actual alpha payouts.

[Rust source](../experiments/settlement-gameability/src/main.rs),
[results](../experiments/settlement-gameability/results.json), and
[artifact hashes](../experiments/settlement-gameability/artifacts.sha256) are
retained. No chain, miner, model, detector API or private data was used. Synthetic
quality and validity verdicts are inputs to the model; the program does not
establish their truth.

## Fixed-credit hypothesis

Each mechanism receives exactly 60,000 unsigned 16-bit credit units: 48,000 for
independent quality and 12,000 for paired adversarial work. Before answers,
divide the reserve among the frozen cells by integer quotient; assign remainder
units in the already fixed canonical slot order. Both mechanisms reserve the
same integer `c` for a cell. For seven cells, the pots are
`[1715, 1715, 1714, 1714, 1714, 1714, 1714]`.

A pair is admitted only when preservation passes, the independent transformation
quality requirement is positive, both endpoints are eligible, and all required
evidence is complete. These conditions include the mode's required external
improvement. An unchanged candidate with no positive improvement fails this
admission fixture. Gates apply outside the credit split, so a rejected candidate
does not award A a full pot.

For an admitted cell, let `L` be the candidate's normalized multiclass Brier loss
against an independently established production label. Let `w` be a bounded
quality factor fixed before A's reply, independent of that reply. An eventual
policy would specify its transformation/Pangram calculation; this experiment
supplies synthetic fixed factors. Set `b=w*L`, then compute:

```text
B_credit = floor(c*b)
A_credit = c-B_credit
```

For a rejected cell, both credits are zero. Add every unearned unit, including
independent-quality residue, to the burn destination. Each mechanism's complete
target vector remains exactly 60,000; no successful-miner renormalization follows.
One admitted cell pays `c` from its combined `2c` reservation and burns `c`.
If all reserved cells pass, 10% of the combined policy allocation burns. More
burns when admission or independent-quality requirements fail.

The grid checked 1,921,920 binary cases: probabilities in thousandths, both known
labels, six pots, five weights and all 32 combinations of the admission inputs.
All 60,060 admitted cases conserved `c`, including 39,902 cases with a fractional
B allocation before flooring. All rejected cases paid zero. Another 693
three-class cases also conserved the pot. Allocation examples with 0, 1, 7, 11
and 127 cells preserved both 60,000-unit rows. These finite checks supplement
the identity `A_credit+B_credit=c`; they are not a proof of the eventual adapter.

## Comparisons that break conservation or incentives

With separate normalization over earned A and B credits, moving a pair from
`(12000,0)` to `(9000,3000)` increases its combined allocation from 10.00000% to
10.83591%, despite the unchanged raw sum. Each mechanism contains 48,000 other
earned credits in this counterexample.

Adding an ordinary Brier penalty to the old linear evasion bonus also leaves
a profitable deviation. For a known model-origin candidate, changing the
reported probability from 1 to 0.99 loses 0.0001 Brier utility while gaining
0.005 under a linear half-weight B bonus. With equal coefficients, the coalition
gains 0.0049 utility units. This is an algebraic counterexample, not a calibrated
monetary estimate.

Source-loss differences create another incentive. Hold candidate loss at 0.25.
Increasing source loss from 0 to 1 reduces `max(0,L(candidate)-L(source))` from
0.25 to 0, raising A's complementary credit from 9,000 to 12,000. Candidate-only
loss leaves A at 9,000 in both cases. Source deltas and panel medians therefore
remain diagnostic in this hypothesis.

## Native-emissions limit

The official algorithm row-normalizes surviving validator weights, clips them
at stake-weighted consensus, and normalizes the resulting ranks into miner
incentives. Owner-directed miner incentive is withheld under the native
burn/recycle policy. [Official emissions specification](https://www.bittensor.com/docs/concepts/emissions).

The simplified model gives 70% of stake identical policy rows and 30% a malicious
row that assigns everything to the coalition. Coordinates are coalition miner,
other miners and burn destination. The honest majority fixes each column's
median; malicious entries above it are clipped. Entries below it remain lower.

| Candidate probability for its true label | A target row | B target row | Combined pair credits | Coalition share after simplified transform |
| --- | --- | --- | ---: | ---: |
| 1.0 | `[12000,48000,0]` | `[0,48000,12000]` | 12,000 | 13.15789% |
| 0.5 | `[9000,48000,3000]` | `[3000,48000,9000]` | 12,000 | 13.56362% |

The mechanisms have equal shares and the malicious rows remain unchanged.
With coalition target fraction `q`, its normalized incentive is
`q/(0.7+0.3*q)`. This nonlinear transformation breaks the complementary-credit
invariant across mechanisms. The increase is 0.40572 percentage points of the
combined modeled miner allocation.

This program does not run native Yuma. It omits runtime fixed-point arithmetic,
permits, self-weight and registration filtering, activity changes, bonds,
validator dividends, pruning and emission-supply changes. It supplies a
counterexample to inferring actual-alpha conservation from raw-credit
conservation; it does not estimate a deployed attack's proceeds.

## Certificates and service accounting

The certificate model uses frozen integer weight `W` and strict acceptance
`3*support>2*W`. Among 1,332 admissible pairs of vote sets across four small
weighted rosters, no conflicting certificates occurred when dishonest weight
was at most one-third and honest signers never signed both decisions.

Exactly one-third withholding breaks liveness: support 2 of total 3 fails the
strict threshold. Safety still holds at that boundary. With weights `[2,1,1]`,
the dishonest weight 2 can sign both decisions and obtain one different honest
signer for each; both supports are 3 of 4 and certify. Shrinking the denominator
to responders is unsafe even below one-third: with frozen weight 4 and dishonest
weight 1, two separate two-signer groups can each appear unanimous after their
denominators shrink. Neither passes against the frozen denominator.

Service fixtures use `N=S+F`, require `U=0` and positive settled work, and apply
`10F<=N`. One failure passes with ten obligations and fails with nine. Nine
successes plus one unresolved obligation fail availability. Completing a failed
100-obligation epoch with 11 failures leaves recovery deficit 10; abandoning all
100 leaves 900. Ten later successes clear deficit 10, while no work preserves it.
Three identical input records count once. A rejected input produces one requester
failure and no activated provider obligation; unresolved input also leaves the
provider unactivated. These fixtures consume abstract authoritative verdicts,
not ciphertext, signatures or chain inclusion proofs.

## Remaining assumptions and reproduction

The conserved pair credit does not prevent a coalition from choosing the cheapest
admissible work. It also does not establish owner independence. In a fixed
eight-cell allocation, ownership of one, two or four endpoint pairs gives 1,500,
3,000 or 6,000 paired credits while the global paired payout stays 12,000.
Acquiring additional scheduled identities can increase an owner's share.

The invariant assumes the paired response cannot change admission, routing,
slot counts, independent rewards, qualification or eligibility. It does not
establish globally truthful forecasting under hidden label-dependent quality
weights, honest semantic review, provenance truth or private-model execution.
Native settlement remains a separate verification requirement.

Run `sh experiments/settlement-gameability/run.sh` from the repository root.
The isolated manifest and lockfile pin dependencies. On a machine without cached
crates, first run `cargo fetch --locked --manifest-path
experiments/settlement-gameability/Cargo.toml` on one line. The completed run passed
format checking, seven focused tests and Clippy with warnings denied, generated
the JSON results, and bound the experiment files with SHA-256. No wider repository
suite or live service test was run.
