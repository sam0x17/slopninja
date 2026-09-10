# Shared-pool settlement experiment

Shared settlement removes the earlier split-pool miner-incentive attack under
fixed-state assumptions. It does not establish resistance to profitable
collusion. Historical validator dividends provide another counterexample, and
the native integer representation introduces smaller split-dependent changes
even with the history term disabled.

The experiment therefore supports further evaluation of a specific
current-only legacy configuration. It does not authorize live rewards or change
the whitepaper's proposed A/B mechanism mapping. The separately selected
[public-detector design](public-model-evaluation.md) removes private A
responses as the scoring authority instead.

## Reproduce

```sh
cargo fetch --locked --manifest-path experiments/shared-pool-gameability/Cargo.toml
bash experiments/shared-pool-gameability/run.sh
cd experiments/shared-pool-gameability
shasum -a 256 -c artifacts.sha256
```

The isolated Rust crate writes [results.json](../experiments/shared-pool-gameability/results.json).
The run script checks formatting, nine focused tests and all-target Clippy,
then records the toolchain, sources and output hashes. It makes no live chain,
model or Pangram calls. Fixtures use synthetic credits, weights and costs.
The [native audit](shared-pool-native-audit.md) identifies the exact upstream
functions and the limits of the adapter; the
[economic audit](shared-pool-economic-audit.md) gives the algebra.

## What shared settlement fixes

The [earlier experiment](settlement-gameability.md) used two separately
normalized native pools. An allied A/B pair could move its fixed total raw
credit between those pools and increase its combined miner-incentive share
from 13.15789% to 13.56362% in a simplified clipping model. The new experiment
reproduces that counterexample.

For one shared pool, let all honest validators submit the same complete
post-filter weight row `h`. Let their active stake be `s`, and the coalition's
total credit in that row be `c`. Keep membership, stake, outside credits and
the clipping ceilings fixed. The maximum coalition miner-incentive share is

```text
c / (s + (1-s)*c)
```

Redistributing `c` among coalition A/B coordinates leaves that maximum
unchanged. The experiment checks 3,245,130 malicious-row cases across six stake
fractions, five coalition-credit fractions and 21 internal splits, and checks
an explicit maximizing row for every setting. The largest floating-point
excess over the bound is below `3e-16`.

This is a statement about the optimized envelope. A fixed arbitrary malicious
row can still receive more after an internal redistribution. The invariant
also requires that private pair outputs cannot change admission, outside
allocations, future qualification or another reward stream.

## Validator dividends defeat the broader claim

Use honest stake `.8`, coalition stake `.2`, and distinct validator and miner
UIDs. The coalition economically owns both A/B miners and the malicious
validator's dividend receipts. It assigns equal weight to A and B. Honest
weights move from `(.05,.15,.8)` to `(.15,.05,.8)` on A, B and an outsider.
The coalition's combined miner incentive stays fixed. Its historical bond
position favors A, so its dividend income rises.

The native fixtures execute unchanged production math function bodies from
[Subtensor revision 67dcf7f](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/subtensor/src/epoch/run_epoch.rs).
The adapter supplies explicit inputs and reproduces the sparse branch's
orchestration. It includes weight encoding, fixed-point arithmetic, stored
bond quantization and integer epoch allocations. It does not execute the SDK
runtime or downstream stake distribution. For a synthetic epoch budget of
1,000,000,000 units:

| Bond calculation | Combined coalition units before | After | Gain |
| --- | ---: | ---: | ---: |
| Legacy, 10% new bond data | 126,786,990 | 137,501,008 | 10,714,018 |
| Yuma3, Liquid Alpha off, 10% new data | 125,824,352 | 131,439,278 | 5,614,926 |

The legacy gain is about 1.0714 percentage points of the epoch budget. The
economic audit proves that the ideal legacy fixture changes the maximum over
malicious rows, rather than merely improving an inefficient fixed row. The
Yuma3 fixture establishes a fixed-row counterexample; it does not solve that
branch's optimization problem. Liquid Alpha enabled was not tested.

Ownership of all modeled validator receipts is an assumption. Delegation,
fees, subnet-owner allocation and downstream distribution must be included
before interpreting these values as an operator's alpha profit.

## Current-only legacy candidate

The pinned administration code permits the candidate settings
`bonds_moving_average=0`, `bonds_penalty=65535`, and `yuma3_enabled=false`,
subject to its administration permissions and timing. The
[economic audit](shared-pool-economic-audit.md#configuration-to-investigate)
links those setters. No live subnet configuration was queried or changed.

With current, fully clipped legacy bonds, the ideal maximum of coalition
miner receipts plus its validator dividends is

```text
(c + 1-s) / 2                         when c <= s
c*(2-s) / (2*(s+(1-s)*c))              when c > s
```

Both depend on total coalition credit and stake, with no internal A/B split
term. The experiment checks 20 stake/credit settings, 11 splits each and
5,151 malicious rows per split, plus explicit continuous maximizers.
Observed split spread at those maximizers is below `2e-16`.

Optimizing only miner income would miss the maximizing strategy for `c<=s`:
the malicious validator can match the complete honest row and earn dividends
on outsiders. At `s=.8,c=.2`, this gives combined ideal share `.2`; supporting
only the coalition gives about `.142857`.

The native before/after swap gives identical integer units with current-only
bonds, for both displayed validator strategies. A separate 19-split sweep
finds residual encoding sensitivity:

| Validator strategy | Minimum units | Maximum units | Spread |
| --- | ---: | ---: | ---: |
| Match the honest row | 199,996,335 | 200,001,219 | 4,884 |
| Equal A/B weights, no outside support | 142,852,158 | 142,858,802 | 6,644 |

The matching-row spread is 0.0004884 percentage points of the synthetic budget.
These are two strategy families, not a search over all native integer rows.
Max-upscaling and rounding raw weight coordinates can change the normalized
coalition sum. An encoded-credit rule and a worst-case rounding bound remain
necessary. The original exact payout-invariance claim is unsupported.

## Failures, costs and changing state

The default experiment is a separate hypothetical funded pot, not a native
emission simulation. Its pot is 100, successful awards are A=40 and B=60,
and independently certified valid work by a nondefaulting partner receives
at most 25 after a single attributable default. It enumerates all eight
sponsor/A/B ownership subsets with each role's cost in `{0,25,100,200}`.

A sponsor that also owns A can gain 35 by causing a refund when A's cost is
zero. Irreversibly funding that pot removes this fixture's refund gain.
However, jointly owned A/B can still gain 125 by skipping A's work when that
work costs 200, without a default bond. Irreversible funding plus a separately
reserved 200-unit bond for each role and ticket blocks every controlled
attributable default in the bounded grid. Three concurrent two-role tickets
would require 1,200 reserved units; reusing one 400-unit reservation is
undercollateralized.

The bond rule depends on independently certifiable fault. An on-time shortcut
that leaves all certified evidence unchanged can save private execution cost
without triggering the rule. Bad quality or an unattributed void cannot be
treated as proven execution fraud. These fixtures exclude gas, liquidity,
future eligibility and off-protocol payments.

The economic audit also supplies a self-weight counterexample when a miner
and coalition validator share a native UID. Application-level role separation
must survive native filtering and permit changes. The Rust sensitivity cases
show that changed active stake or removing an outsider and renormalizing the
remaining credits also breaks the fixed-state assumptions. They do not show
how an attacker can cause those state changes.

## Decision

Shared settlement with ordinary historical dividends does not solve the
collusion problem. Current-only legacy bonds remove the displayed history
attack, but leave rounding, native lifecycle behavior and net-cost incentives
to establish. Removing bond history also changes the incentive for validators
to identify good miners early; that tradeoff needs measurement.

The tested configuration remains a research candidate. The selected public A
design has validators compute benchmark probabilities directly, removing the
private reported probability as an authority. It still needs independent
detector qualification and tests for targeted backdoors and copied-model
incentives. It does not require publishing B's generator or customer text.
