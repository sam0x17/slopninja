# Style measurement and final review

Design specification for the A/B launch. This defines B's target-style utility
`V`, preservation/readability/tone gate `G`, and judgment closure. It adds no
writing-improvement service or separate quality bonus. A artifacts are public
and validators execute them for benchmark probabilities. B generators remain
private; their committed revisions supply the text evaluated here.
Every B task must preserve the source's full meaning and fulfill the task's
target tonal intent. Positive utility requires independent semantic certification;
style proximity and detector evasion cannot compensate for a preservation
failure.
The [whitepaper section](../whitepaper/sections/07a-adjudication.tex) carries the
same rules. Authority, encrypted publication, service fault attribution and
incident handling follow [mandatory-service evidence](../whitepaper/sections/08a-service-evidence.tex).

Freeze the qualified epoch roster and cache complete A panel artifacts before
B task disclosure or candidate generation. Model manifests bind weights, features, tokenizer,
preprocessing and postprocessing, calibration and all runtime parameters and
dependencies needed for canonical execution. Validator receipts bind the model
and manifest hashes, runtime/settings, input commitment and complete probability
output. B submits full locally computed vectors bound to the protocol-assigned
A models/settings and exact candidate-input commitment. Validators re-execute
and compare canonical outputs; no mandatory A inference-service call supplies
a score. Hidden independent author/origin labels and
targeted-trigger controls qualify published artifacts; replay alone does not
establish detector accuracy or absence of deliberately targeted behavior.
B miners can run those public models on their own candidates during search.
Their scores cannot be withheld as secret evaluation feedback. Hidden labels,
other miners' candidates, unreleased sources and semantic-review evidence
remain restricted to their authorized recipients. Validator execution has its
own capacity budget, separate from directed A-requester/B-provider rewrite tickets.
After issue, a validator outage permits only execution of the same cached
artifact on a reserved approved runner, with no model, version or threshold
substitution. Canonical reference failure leaves the entire comparison
unresolved or void under fixed closure; it cannot become a pass or B nonresponse.

## Frozen numerical inputs

This measurement applies when the task requests target-author fit. Evasion-only
tasks record `not_requested`, with absent numeric style fields; they need no
target-reference or calibration input.

Before issue, the instance manifest must identify a permitted public
word/grammar author-distance artifact. Pin its artifact and parameter hashes,
parser and feature schemas, reference-pooling rule, executable/runtime,
arithmetic, rounding, resource limits and test vectors. Its function
`d_theta(text, target_references)` returns a nonnegative integer distance or a
declared error status. Concrete artifact selection and validation are required
implementation inputs; this specification does not claim that a released or
trained evaluator already meets them. Private B generator weights never define
this public style evaluator.

Compute a nonempty calibration multiset `C = [c_1, ..., c_M]` from development
queries and independent reference works by their attributed authors, using that
same artifact and pooling rule. Development source families remain separate
from fitting and evaluation. Bind provenance, source hashes and the complete
integer-distance multiset. Repeated distances remain repeated observations.
Each task fixes its calibration ID before candidate generation.

For `Q = 1_000_000` and integer distance `d`, define:

```text
greater = count(c in C where c > d)
equal = count(c in C where c == d)
q = floor(Q * (2*greater + equal) / (2*M))
```

Use integer arithmetic with sufficient width and the indicated floor operation.
The empirical survival rank gives exact ties half credit before quantization.
Smaller distances cannot reduce `q`; distances below all calibration values
score `Q`, and those above all values score zero. This is a distance rank, not
an authorship probability or a preservation result.

Score source `x` and candidate `z` against the same target references, artifact
and calibration. Let their scores be `q_x` and `q_z`:

```text
if q_x == Q:
    V_numerator = 0
    V_denominator = 1
else:
    V_numerator = max(0, q_z - q_x)
    V_denominator = Q - q_x
V = V_numerator / V_denominator
```

Store the rational in reduced form, with positive denominator and zero encoded
as `0/1`; retain exact rational arithmetic through the frozen task aggregation.
Do not round each candidate into policy credits before aggregation. The final
allocation rule specifies any conversion to integer credit. Equality earns
zero. A source already at `Q` has zero reward headroom. A task requiring no
style regression separately checks `q_z >= q_x`; clipping `V` cannot erase that
requirement.

For style-scored tasks, source and reference scoring must succeed before issue. A declared
deterministic `unscorable_candidate` result yields zero style utility, with its
status retained. Evaluator disagreement, unavailable shared evidence or an
execution/version failure does not invent a score: it invokes unresolved
comparison closure. Numerical failure alone does not establish attributable
service nonresponse. Test copying, topic substitution and meaning-breaking
candidates before relying on this artifact for rewards.

## Semantic fields

The source's full meaning must remain intact. Before generation, resolve the
target tonal intent from the brief and, where supplied, the requested authorial
style and authorized reference samples; freeze it in the task rubric. A dramatic authorial-style
change may authorize a different tone without a separate tone instruction.
Source tone remains the default for aspects the request leaves unchanged.
Greater stylistic force cannot strengthen factual certainty or obligations.
The committed brief also identifies protected information and readability
requirements. The rubric and evidence
include the full source and candidate; a checklist does not authorize ignoring
other material claims. Tone review applies the frozen rubric and its source-tone
defaults throughout the candidate; reviewers cannot add new stylistic preferences
after generation. Each final semantic ballot is `PASS`,
`FAIL` or `ABSTAIN`.

Before task issue, the brief identifies required cleanup of unwanted filler,
repetition or formulaic phrasing under `R`, and intentional voice features to
preserve under `T`, including any specified informality or idiosyncrasies.
Cleanup must retain the protected information.

| Field | PASS condition | FAIL condition |
| --- | --- | --- |
| `P`, preservation | The source's full meaning remains intact, including all claims, entities, quantities, negation, uncertainty, citations, attribution and argument relations | Any material omission, altered meaning, unsupported addition or prohibited reference copying |
| `R`, readability | No material readability regression against the source, and every stated readability requirement, including required cleanup, is satisfied | A material regression or a violated readability requirement |
| `T`, tone | The frozen target tonal intent, tone/audience constraints and intended voice features are satisfied | The revision violates the target tonal intent or any tone/audience constraint |

Insufficient evidence or unresolved interpretation, including uncertainty about
the target tonal intent, requires `ABSTAIN`. This field always requires a
semantic vote, including when the request contains no separate tone instruction.
An unrequested tone reversal can defeat the writing brief while retaining literal
claims, so we require this vote and measure its added review cost and void
exposure in the pilot.
A failure ballot names rubric items and source/candidate byte
spans where applicable; omissions can identify source spans and surrounding
candidate locations. Additional preference judgments, ties and praise remain
diagnostic and earn no extra utility. Detector probabilities supply none of
these field verdicts.

## Global certification

Three hash-assigned preparers organize the evidence dossier and identify
disagreements. They cannot finalize outcomes. Every certifying validator must
inspect the authorized source, references, brief, candidate and rubric and
attest the semantic field itself. The review client presents semantic evidence
first. Honest signers commit their field ballots before inspecting detector
scores or numerical style results; those checks can follow under the same final
closure deadline. Giving the full validator set evidence access does not
cryptographically guarantee this blinding. Certifying a tally from three
reviewers is insufficient.

Use the proof-verified validator snapshot and integer effective weights defined
by the service-evidence protocol, frozen before task contents are revealed.
For total eligible weight `W`, a field's `PASS` or `FAIL` certificate requires
distinct eligible signers with agreeing weight `w` such that:

```text
3*w > 2*W
```

Positive `W`, qualified recipient keys and complete publication/verification
capacity are preconditions for issue.

The denominator includes missing, abstaining, recused and nonresponsive
validators. It cannot shrink after assignment or a key change. The conditional
security assumption is dishonest eligible weight strictly below `W/3`, with
sufficient honest participation for closure. Honest validators do not sign
conflicting final verdicts. Two strict quorums intersect in more than `W/3`
weight; conflicting certificates halt the affected settlement.

## Fixed pilot deadlines

Use the epoch origin `H`, canonical finalized inclusion and deadline-equality
rule from the service-evidence protocol. Complete evidence and publication
capacity must be reserved before issue.

| Stage | Inclusion deadline |
| --- | --- |
| Preparers commit their dossiers | `H+180` |
| Preparers open evidence privately | `H+192` |
| Evidence-correction window closes | `H+196` |
| Validators commit final field ballots | `H+198` |
| Validators open final ballots privately | `H+202` |
| Global field certificates and incident close | `H+204` |

Corrections may repair evidence references or identify contradictions in the
already committed material. They cannot replace the candidate, brief, target
references, rubric, evaluator or calibration. After the correction cutoff, each
validator has one final ballot per field. The first canonical timely commitment
is authoritative. Identical retries are idempotent; conflicting signed ballots
remain evidence and cannot replace it. An absent, late or invalid opening earns
no agreeing weight and leaves the validator in `W`.

These deadlines specify the pilot; they do not demonstrate that full global
semantic review fits its publication, gas or verification budget. Failure to
reserve the required capacity prevents issue. Finality stalls, certified
platform incidents and payout-cycle delays use the service-evidence rules.
Candidate certification by `H+72` through `H+168` leaves 30 to 126 blocks
before final ballot commitment: approximately 6 to 25 minutes at 12 seconds
per block. Certifiers must begin from the authorized source and candidate
as evidence becomes available. Waiting for dossier opening at `H+192`
would leave only six blocks for review.

## Closure and record format

At the final deadline, first check for conflicting certificates. A conflict
halts settlement. Otherwise:

```text
if any of P, R, T has a valid FAIL certificate:
    outcome = FAIL
    G = 0
else if all of P, R, T have valid PASS certificates:
    outcome = PASS
    G = 1
else:
    outcome = UNRESOLVED
    G = null
```

All three fields require certification, including target-tone compliance when
the request adds no separate tone instruction. A field lacking a quorum does not
default to either `PASS` or `FAIL`. An unresolved candidate judgment
voids its matched source/brief/mode comparison for the whole B batch. Preserve
every candidate and review in reporting; do not selectively remove a difficult
case or draw different reviewers to obtain a preferred verdict. This quality
void does not erase independently attributable service failures. Missing miner
submissions retain the service-evidence protocol's nonresponse treatment.

Whole-batch closure prevents validators from using unresolved judgments to
exclude only one competitor's candidate. It also gives a B submitter a
possible disruption strategy.
A chosen candidate may split honest judgments or induce abstention until
neither `PASS` nor `FAIL` exceeds `2*W/3` on a required field. If no other
field has a `FAIL` certificate, the comparison is void. Dissent need not
cause a void: more than two-thirds agreeing on failure certifies `G=0`.
A dishonest minority below `W/3` cannot block a certificate when all
remaining weight agrees and participates.

An unresolved semantic judgment adds no service failure or recovery deficit.
Generation and report costs still apply. A UID expecting zero skill can
nevertheless remove competitors' positive skill from that comparison and
increase its owner's normalized share through sibling UIDs scoring elsewhere.
This economic risk remains unresolved. The [launch pilot](../whitepaper/sections/11-launch.tex)
reserves four source/brief comparisons covering attribution/negation,
qualifications, readability and tone, with clear-pass, clear-fail and attempted
unresolved variants for each. Include a requested dramatic tonal shift as an
intended pass, and certainty changes, unrequested tone reversal and loss of
required informality as violations. Briefs without separate tone instructions
must still receive target-tone certification, using the requested authorial
style or the source-tone default as appropriate. Preserve field-level weighted ballots, review
time, candidate/report costs, induced voids and excluded competitor skill.
Replay the frozen allocation with and without the attack entry to measure
common-owner share changes against benign-ambiguity and infrastructure-failure
controls. These counterfactuals are diagnostics; the actual rule remains
whole-batch closure with strict certificates. Live-reward acceptance must
address any profitable induced voids observed in the study.

The proposed `reward-adjudication-v1` record uses the same deterministic CBOR
restrictions and encrypted publication path as `service-evidence-v1`. It binds:

- Protocol, chain, contract, epoch, task, comparison and submission identifiers.
- Source, candidate, target-reference, brief and rubric commitments; artifact,
  parameter, runtime, calibration and evidence hashes.
- Qualified public A panel manifests, canonical execution settings and bound
  input/output receipts when origin scoring is requested; B's complete submitted
  probability vectors and their comparison with canonical validator outputs.
- Numeric status, `q_x`, `q_z`, reduced `V` numerator/denominator and any required
  raw nonregression check.
- Frozen validator snapshot root, total `W`, signer UID/generation/key bindings,
  field IDs, ballot commitments/openings, reason codes and evidence byte spans.
- All stage deadlines, field certificates and agreeing weights; `PASS`, `FAIL`
  or `UNRESOLVED`, `G`, and the common comparison's void/incident reason.

Ballot signatures bind the record identity, snapshot, field, final stage,
evidence root and verdict. Commitments are salted; private openings and full
semantic evidence are encrypted separately to every authorized certifier.
Source text, reference writing and hidden review material require authorization
for that complete recipient set and must fit the funded witness limits. Customer
jobs never enter this process automatically.

Given the same certified inputs, validators compute the same result. Human
interpretations may disagree and fail to obtain a quorum. Agreement certifies
the scoped semantic judgment under the declared trust assumption, without
proving meaning preservation. Validators reproduce public A computation;
scoring B's submitted output does not require reproducing its private generator.
The one-ticket A-requester/B-provider rewrite mailbox pilot does not implement
artifact loading, canonical inference or the full common panel matrix; those
need separate capacity and replay checks.
