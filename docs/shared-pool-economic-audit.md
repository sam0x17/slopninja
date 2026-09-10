# Shared-pool economic audit

Complementary A/B credits in one native pool can preserve the coalition's
maximum miner incentive under fixed filtering and stake. Extending that claim
to miner receipts plus validator dividends requires a separate argument.
Historical bonds give a counterexample even with 20% dishonest active stake.

These are algebraic fixtures, not deployed attacks or a full-runtime replay.
They use the sparse epoch branches at upstream commit
`67dcf7f791dc495064c293f080a0702cb433e51e`. The
[epoch implementation](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/subtensor/src/epoch/run_epoch.rs#L779)
selects different legacy and Yuma3 bond calculations. Both contribute validator
dividends to epoch emission. Integer rounding, dispatch permissions and the
selected subnet's actual configuration require independent verification.

## Fixed miner incentive, changing legacy dividends

Two validator UIDs have active stakes `s=.8` and `t=.2`. The coalition owns the
second validator and both A/B miner UIDs; all validator and miner UIDs are
distinct. All honest weights equal `h=(a, .2-a, .8)` on A, B and outsider O.
The coalition validator submits `m=(.5,.5,0)`. Consensus clips each column at
`h`; use fully clipped weights for bonds. All registrations, permits, stakes,
quality gates and outside credits stay fixed.

The ranks are `(a,.2-a,.64)`, with sum `Z=.84`. Hence the coalition's combined
miner incentive is always `I_C=.2/.84=5/21`.

For the legacy branch, take prior column-normalized coalition bond shares
`(.2,0,0)`, with the honest validator holding each complement. Let the new-data
EMA fraction be `lambda=.1`. Current coalition bond deltas are `(.2,.2,0)`, so
updated bond shares are `(.2,.02,0)`. The coalition dividend is

```text
D_M = (.2*a + .02*(.2-a)) / .84
```

Both incentive and dividend vectors sum to one in this fixture. The epoch
routine therefore assigns coalition gross share `(I_C+D_M)/2` before integer
conversion and downstream distribution.

| Honest A/B credits | Coalition miner incentive | Coalition dividend | Combined epoch share |
| --- | ---: | ---: | ---: |
| `.05 / .15` | `5/21` | `13/840` | `71/560 = .1267857143` |
| `.15 / .05` | `5/21` | `31/840` | `11/80 = .1375` |

The gain is `3/280`, or about 1.07143 percentage points. No false preservation
certificate is needed: this fixture changes only the internal allocation.
Treat the validator's economic receipts as coalition-owned; a real deployment
must account for delegation and distribution rather than equating validator
control with ownership of every dividend unit.

This also changes the maximum over current malicious rows. Write
`w_j=min(m_j,h_j)`. Twice the combined share equals `N/Z`, where

```text
Z = .8 + .2*(w_A+w_B+w_O)
N = .16 + .144*a + .256*w_A + .22*w_B + .02*w_O
```

For either displayed `a`, the ratio increases with `w_A` and `w_B`, and decreases
with `w_O` throughout their permitted ranges. The maximum is attained at
`w_A=a`, `w_B=.2-a`, `w_O=0`, which the submitted row realizes. Its value is
`(.204+.18*a)/1.68`; the optimal combined envelope changes with the split.

The historical asymmetry has an ordinary construction: keep 20% coalition
stake, let it support A but not B or O, and retain honest consensus rows. The
legacy branch reaches coalition shares `(.2,0,0)` from empty bonds after its
first column normalization in exact arithmetic, and preserves them under that
history. Column-max scaling for storage and column normalization on reload
preserve the relative shares before integer rounding. The strict gain persists
under small perturbations; exact native values require replay of those storage
round trips. A minimum nonzero-weight count can require small positive weights
in place of zeros, which also needs a configured fixture.

## Yuma3 needs its own calculation

Use the same current rows and stakes. With Liquid Alpha disabled and new-data
fraction `.1`, take historical raw bonds `H=(.1,.1,.8)` and `M=(.1,0,0)`.
These are stationary clipped-weight limits of the same A-only support history
when the old honest row is `(.1,.1,.8)`.

After updating and normalizing bond columns, M's A share is `.5`, its B share is
`.1*b/(.09+.2*b)`, and its outsider share is zero, where `b=.2-a`. Thus

```text
T_M = (.5*a + (.1*b/(.09+.2*b))*b) / .84
D_M = .2*T_M / (.8*(1-T_M)+.2*T_M)
```

| Honest A/B credits | `T_M` | `D_M` | Combined epoch share |
| --- | ---: | ---: | ---: |
| `.05 / .15` | `5/96` | `5/369` | `325/2583` |
| `.15 / .05` | `31/336` | `31/1251` | `1151/8757` |

The current malicious row is unchanged and still maximizes miner incentive.
These two values alone do not establish the maximum combined Yuma3 return over
all malicious rows. Liquid-Alpha-enabled settings also need their own update
and optimization; legacy bond shares cannot be inserted into that branch as
though their storage geometry were identical.

## UID filtering can break the miner assumptions

Now let A be the coalition validator's own UID. Ordinary self-weight removal
prevents that validator from supporting A. Its best remaining row supports B.
The ranks become `(.8*a,.2-a,.64)`, so combined A/B miner incentive is

```text
I_C = (.2-.2*a) / (.84-.2*a)
```

Moving `a=.15` to `.05` increases this from `17/81` to `19/83`, although the
intended A/B credit sum stays `.2`. This violates the disjoint-UID assumption.
It explains why that condition must hold at native settlement, including
permit changes, rather than only in the application's initial roster.

The [same sparse epoch path](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/subtensor/src/epoch/run_epoch.rs#L666)
filters active stake, self-weights and stale registration references before
normalization. Its owner exception also needs explicit treatment. Fixed raw
application credits do not by themselves establish fixed surviving rows.

## Configuration to investigate

The [constant-alpha calculation](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/subtensor/src/epoch/run_epoch.rs#L1158)
sets the new-data fraction to `1-bonds_moving_average/1_000_000`. In the legacy
branch, zero moving average and fully clipped bonds remove the historical
term. The pinned administration code permits owner/root settings of
[`bonds_moving_average=0`](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/admin-utils/src/lib.rs#L920),
[`bonds_penalty=65535`](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/admin-utils/src/lib.rs#L959)
and [`yuma3_enabled=false`](https://github.com/opentensor/subtensor/blob/67dcf7f791dc495064c293f080a0702cb433e51e/pallets/admin-utils/src/lib.rs#L1724),
subject to owner rate limits and the administration window. No live setting
was queried or changed. This is a candidate configuration for runtime tests.

For that current-only legacy model, let `U_C` and `U_O` be total clipped
malicious weight on coalition and outside miners. Then

```text
combined_share = (s*c + 2*t*U_C + t*U_O) / (2*(s+t*(U_C+U_O)))
```

Its maximum depends only on `c` and `t`: `(c+t)/2` when `c<=s`, and
`(1+t)*c/(2*(s+t*c))` when `c>=s`. The maximizing validator row may support
outside miners to earn dividends. This removes the displayed legacy history
attack under the fixed-state assumptions. It makes no claim about Yuma3,
ongoing stake changes, net profit, private execution or live parameter access.

Before selecting a policy, replay finite bond histories and optimize actual
miner plus economically owned validator receipts under the chosen native
branch. Include native integer operations, registration and permit changes,
burn treatment and delegation. A proof about miner incentives alone cannot
authorize a claim about that larger sum.
