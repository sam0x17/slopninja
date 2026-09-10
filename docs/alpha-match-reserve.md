# Funded alpha matches: design option

This option is pending a funding decision. It is not an adopted launch policy
or an implemented contract. Launch scope remains A detection and B
transformation; a match reserve would pay the adversarial interaction between
them, without adding another task or chain mechanism.

The [settlement experiment](settlement-gameability.md) shows why complementary
credits alone cannot protect combined actual payouts after native weight
normalization. The proposed alternative locks each match's alpha principal
before either endpoint answers and settles its two awards directly.

## Proposed separation

Native A emissions would reward independent labeled author/origin evaluation.
Native B emissions would reward certified preservation and public style and
Pangram improvement. The private A responses from paired matches would affect
only their locked match pots. They could not change independent native scores,
pass bonuses, qualification, slot counts, strongest-A ranking or service
priority. Service validity and actual failures would still follow the declared
availability rules; a favorable or unfavorable probability is not a failure.

Every B candidate would still face the common panel, including the strongest
qualified A service. Retain source/candidate responses and joint pass rates.
Each payable candidate/detector cell would have a preassigned pot; source
response differences and panel medians would remain diagnostics. A cannot
increase its award by deliberately worsening a source-baseline prediction.

For a fixed cell, let `c` be the funded pot in integer alpha units and `m` its
joint admission decision. Admission requires certified B preservation, positive
independent improvement for the requested mode, both endpoints' eligibility,
an independently supported production label, and complete valid evidence.
Apply this gate to both endpoints before computing either award.

One concrete split to evaluate is:

```text
J = V                         for author-style mode
J = max(0, delta_P)            for evasion mode
J = (V + max(0, delta_P)) / 2  for combined mode
```

Combined mode also requires raw style nonregression and `delta_P >= 0`.
Here `V` is the [frozen style utility](reward-adjudication.md), and `delta_P`
uses the required committed source and candidate Pangram observations. `J`
and the committed candidate, brief and evaluator inputs must be fixed before
the required A response. Later preservation review remains blinded to detector
scores; its certificate and service closure determine final joint admission.
An unchanged candidate with no improvement has `J=0` and
fails admission. For the candidate's normalized Brier loss `L` against the
supported production label, compute exact rational `b=J*L`, then:

```text
if m:
    B_award = floor(c*b)
    A_award = c-B_award
else:
    A_award = B_award = 0
```

For any admitted fixed match, `A_award+B_award=c`. Changing an internally
coordinated A probability only divides the same principal. A rejected pair
cannot award A the full pot. The scheme does not assert that all hidden,
label-dependent multipliers preserve globally proper forecasting; A's separate
mixed-origin evaluation retains its ordinary proper score.

## Funding and closure

A named sponsor would deposit transferable subnet alpha into a separate match
contract. Sponsorship could come from an operator, validators or another party;
the design cannot presume access to their future native emissions. A revocable
allowance would not count as funding. Customer escrow would remain separate.

Before issuing a round, freeze its funded budget, canonical cell order, pots,
recipients, endpoint generations and settlement deadlines. Divide the budget
by quotient and remainder in that order. Insufficient funding prevents issue;
answers never increase the budget or cause a new assignment. A rejected or
void cell would return principal to its original sponsor after closure, with
no within-round redistribution to successful miners. Successful awards would
be finalized atomically under the approved strict weighted certificate rules.
Retried settlement must be idempotent, and withdrawal cannot reverse a verdict.

Alpha is a subnet stake position. The proposed adapter would use contract
custody and same-subnet stake transfers, with exact native units, separate fee
funding and separately accounted stake accrual. Both transfers must succeed or
the transaction must revert without consuming the pot. These behaviors require
runtime tests; no ERC-20 balance or automatic diversion of network emissions
is assumed. [Bittensor staking guide](https://www.bittensor.com/docs/guides/evm/stake-from-evm),
[staking-v2 interface](https://www.bittensor.com/docs/guides/evm/precompiles/staking-v2).

## Limits to test before selection

The identity protects only the admitted pair's fixed principal against changes
to its internal response. It does not prove private execution, honest semantic
review, independent UID ownership or truthful provenance. Common owners can
still prefer the cheapest admissible B output, acquire more scheduled identities,
recognize benchmark texts, or exploit external commercial consequences.
The proposed sponsor refund creates a concrete failure incentive. A sponsor
who also owns A can withhold on an expected B win, recover the pot, and deny
the honest B payment. This can occur within the 10% failure allowance; joint
epoch eligibility does not close it. Sponsor+B has the symmetric incentive.
This refund policy therefore remains unresolved and is unsuitable for adoption
without a further change. Irrevocably retaining failed principal for a frozen
future global budget would remove immediate sponsor recovery, but would still
need analysis of timing, future allocation capture and deliberate voids.

Sponsor withdrawal permissions and transfer-failure deadlines must also prevent
one endpoint from stranding its honest partner's award. Future stake, reputation
and registration effects require separate analysis. Fixed admitted-pot
conservation alone supplies no broader claim about profitable deviations.

Adoption would require a named source of launch funding and a bounded reserve,
then a simulation of the complete scoring and refund rules followed by the
alpha custody pilot. Without that choice, the monetary connection between
private A outputs and B remains unresolved in the whitepaper.
