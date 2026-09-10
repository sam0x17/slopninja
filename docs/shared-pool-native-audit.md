# Shared-pool native arithmetic audit

The fixed combined A/B miner-credit bound does not, by itself, bound combined miner and validator receipts. The pinned native functions reproduce a historical-bond counterexample in both classic Yuma and Yuma3 with liquid alpha disabled. A separate current-only classic configuration removes this particular historical-bond effect under the experiment's fixed-state assumptions.

This audit pins upstream Subtensor commit [`67dcf7f791dc495064c293f080a0702cb433e51e`](https://github.com/opentensor/subtensor/tree/67dcf7f791dc495064c293f080a0702cb433e51e), selected with `git ls-remote HEAD`. It identifies repository code, not a deployed chain runtime or live subnet configuration. No chain state, funds, models, or detector APIs were accessed.

## What executes

[`native.rs`](../experiments/shared-pool-gameability/src/native.rs) exposes `run() -> serde_json::Value`. It calls the complete pinned `math.rs` with five import-path substitutions. The function bodies are unchanged. The adapter uses `substrate-fixed` 0.6.0 at commit `d5f70362f2e05b5f33fb51cd7baa825323e4e6c5`, `num-traits` 0.2.19, and `log` 0.4.28, matching the relevant upstream lock entries. A small compatibility module copies the used primitive and fixed-point division method bodies from `safe-math`. The SDK's `CheckedAdd` is a reexport of the same `num-traits` trait. The bundle retains the original files and Apache-2.0 license; its [manifest](../experiments/shared-pool-gameability/upstream/manifest.json) records hashes, source URLs, dependencies, and the reexport evidence.

The adapter follows the sparse `epoch_mechanism` branch's arithmetic order after the runtime filters. It does not compile or execute the full pallet, extrinsic checks, storage access, state transitions, or final coinbase distribution. Each fixture assumes two active, permitted validators on separate UIDs from the three miners, no owner exception, no stale weights, no registration or commitment mask, and fixed effective stake in ratio 8:2. The source of this arithmetic is the [sparse epoch implementation](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/subtensor/src/epoch/run_epoch.rs#L609-L1121).

The fixture parameters are explicit: kappa `32767 / 65535`, bonds penalty `65535 / 65535`, and liquid alpha disabled. Historical cases use bonds moving average `900000 / 1000000`, leaving a new-bond fraction near 0.1. Current-only cases use moving average 0. These are selected experiment settings; the audit makes no claim that a live subnet uses them.

## Native paths that matter

The setter max-upscales submitted u16 weights and rounds. The epoch row-normalizes the surviving stored values. It calculates the stake-weighted median for each destination, clips each column to that consensus, computes stake-weighted ranks, and normalizes ranks into miner incentives. Bond storage uses a different conversion: fixed-point proportions multiplied by 65535 and truncated. Emission shares become integer amounts through `I96F32` multiplication and truncation. All these conversions execute in the replay. [Weight setter](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/subtensor/src/subnets/weights.rs#L704-L820), [conversion functions](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/subtensor/src/epoch/math.rs#L20-L142).

With fully clipped weights denoted C, active stake s, and miner incentives I:

| Branch | Bond update and dividend calculation |
|---|---|
| Classic | Column-normalize old bonds. Form and column-normalize `C_ij * s_i`. Apply EMA, then column-normalize again. Dividend is `sum_j B_ij * I_j`, normalized across validators. Column-max-upscale the new bonds for storage. |
| Yuma3, liquid alpha off | Load historical raw bond proportions. Apply EMA directly between C and those raw bonds. Column-normalize a copy for the dividend calculation. Compute `T_i = sum_j B_ij * I_j`, then normalize `s_i * T_i` across validators. Store raw EMA proportions. |

The two branches therefore require different historical-state fixtures. Yuma3 with enabled liquid alpha has additional per-edge rates and consensus-history choices; this replay does not test that branch. [Branch definitions](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/subtensor/src/epoch/run_epoch.rs#L857-L968), [liquid-alpha dispatch](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/subtensor/src/epoch/run_epoch.rs#L1381-L1440).

The ordinary nonzero epoch normalizes incentive plus dividend together. With each summing to one, that gives approximately half to miner incentives and half to validator dividends before integer truncation. This is the epoch routine's supplied emission budget, not a claim about every component of chain issuance or a coalition's net profit. The zero-emission branch instead falls back to active stake, or unmasked normalized stake if active stake is zero. [Emission calculation](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/subtensor/src/epoch/run_epoch.rs#L975-L1032).

## Exact replay results

The honest row is `(A=.05, B=.15, outside=.8)` or its A/B swap. The coalition validator submits `(.5, .5, 0)`. Its active stake is .2. Classic historical column shares for the coalition are `(.2, 0, 0)`, with honest complements `(.8, 1, 1)`. The Yuma3 fixture instead uses historical raw bonds `H=(.1,.1,.8)` and `M=(.1,0,0)`. Each history is encoded into native u16 storage before loading. The saved output records every input row, fixed-point bit pattern, old and new bond, incentive, dividend, and integer emission.

| Native branch | Honest A/B inputs | Coalition miner incentive | Coalition validator dividend | Gross epoch share | Units from 1 billion |
|---|---:|---:|---:|---:|---:|
| Classic, historical | .05 / .15 | 0.238098006 | 0.015475978 | 0.126786991 | 126,786,990 |
| Classic, historical | .15 / .05 | 0.238098006 | 0.036904013 | 0.137501009 | 137,501,008 |
| Yuma3, historical | .05 / .15 | 0.238098006 | 0.013550700 | 0.125824353 | 125,824,352 |
| Yuma3, historical | .15 / .05 | 0.238098006 | 0.024780554 | 0.131439279 | 131,439,278 |

The coalition miner incentive has exactly the same `I32F32` bit pattern, `1022623147`, in all four rows. Gross integer receipts rise by 10,714,018 classic units or 5,614,926 Yuma3 units. The increase comes from validator dividends. This demonstrates a state-dependent payout channel if a coalition can shift the honest A/B split; it does not establish how cheaply it can cause that shift or acquire the assumed bond history.

For current-only classic bonds, the concentrated coalition row yields gross share `0.142858803`. Matching the honest row instead yields `0.200001220`, while serving as a validator for outside miners too. Both are invariant under the chosen .05/.15 swap. A bounded 19-split sweep, with nominal A+B=.2 and A from .01 through .19, finds matching-row gross shares from `0.199996337` to `0.200001220`. That small spread comes from u16/fixed-point rounding; nominal decimal conservation is not exact encoded conservation. The sweep is a rounding probe and comparison of two declared rows, not a search over all possible native u16 matrices. The separate ideal-arithmetic search appears in the parent experiment's results.

## Filters and identity assumptions

A deployment must account for the following state before invoking any fixed-row bound:

- Current validator permits and activity filter active stake; next-epoch permits are selected separately. The subnet owner has explicit threshold, permit, and diagonal-weight exceptions. Ordinary selfweights are masked. The replay uses no owner UID or selfweights.
- A destination registered at or after a validator's last update is masked: `last_update <= registration`. Commit-reveal adds a separate mask when the earliest relevant commit predates registration: `commit_block < registration`. Remaining rows are renormalized, so a UID change can alter allocations to surviving targets.
- Bonds to new registrations are masked when `last_tempo <= registration`. Losing a validator permit clears its stored bond row. These conditions can invalidate a proposed persistent-state counterexample or change the inputs to a supposed fixed-credit guarantee.
- Reusing a UID creates a new logical neuron. Replacement changes hotkey membership and registration block, removes its outgoing weights and bonds, zeros incoming weights, resets its permit and recorded scores, and initializes its last-update timing. A bare UID is not a stable counterparty identity across registrations.

These are source findings, not simulated churn outcomes. Snapshot the effective stake returned by the runtime, permits, registration versions, commitments, mechanism and owner settings along with the weight and bond rows. The replay does not reconstruct effective stake from live alpha/TAO holdings or model stake delegation, fees, ownership, or payout beneficiaries. [Sparse filtering](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/subtensor/src/epoch/run_epoch.rs#L638-L825), [UID replacement](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/subtensor/src/subnets/uids.rs#L20-L137).

## Reproduce and scope

From the repository root, after fetching the pinned Cargo dependencies once:

```sh
experiments/shared-pool-gameability/upstream/verify.sh
experiments/shared-pool-gameability/run.sh
jq '.pinned_native_math' experiments/shared-pool-gameability/results.json
```

The first command verifies all original source hashes and the import-only transformation. The second runs the isolated manifest's formatting, tests, Clippy, release experiment, and artifact hashing. Three native-focused tests cover source preservation and rounding, equal miner bits with unequal historical dividends, and the current-only row comparison. No full-chain build is required.

These fixtures establish arithmetic counterexamples and a restricted current-only comparison. Any launch claim still needs the exact configured native branch, encoded rows, stake and eligibility transitions, failure accounting, and a treatment of validator receipts. Conserving A/B miner credits alone leaves the demonstrated historical-dividend channel open.
