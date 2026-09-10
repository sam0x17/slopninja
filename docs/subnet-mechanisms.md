# Slop Ninja subnet mechanisms

Status: design proposal, 2026-09-09. We have enough evidence to define an
offline competition. Paid validation still needs a suitable private benchmark,
measured judge reliability and an authenticated challenge ledger.

The [whitepaper PDF](../whitepaper/slop_ninja.pdf)
([LaTeX source](../whitepaper/main.tex)) is the current overall design. The scoring
and audit access below apply to subnet-owned benchmarks. Customer inference
inputs are encrypted only for their assigned miner; a customer may separately
send evidence to a specific validator. Those disclosures do not authorize
general validator access or automatic benchmark reuse.

The initial subnet develops two capabilities: recognizing authors and production
histories, and transforming text toward a requested voice while preserving its
meaning and readability. Miners can improve the grammar and word representation,
the task models, or the editing process. The existing model supplies a baseline;
we do not need to perfect it before testing these incentives.

Miners keep their trained models private and serve authenticated task requests.
The [model and data plan](miner-models-and-data.md) defines starter architectures
and training records; the [paid inference proposal](paid-inference.md) adds
customer jobs with on-chain settlement. Emissions reward observed performance,
while customers pay separately for service. No weight publication is required.

Evasion targets Pangram and a common panel of qualified subnet origin
detectors, including the strongest from the previous completed A round.
Compare detectors against documented production histories; neither detector
agreement nor a public peer identity supplies ground truth. The
[API verification and funding design](pangram-oracle.md) specifies the
external dependency and its remaining trust assumptions.

## Two tasks, two chain mechanisms

Bittensor's current documentation caps a subnet at two on-chain mechanisms.
Each has independent weights and consensus; participants share UIDs. Validators
select a mechanism using `mechid`. The launch maps A detection to mechanism 0
and B transformation to mechanism 1. Author and origin detection remain
separately scored subtasks of A.
[Mechanism documentation](https://www.bittensor.com/docs/guides/subnets#mechanisms),
[weight submission](https://www.bittensor.com/docs/tx/set-weights).

| Logical task | Miner produces | Main evaluation | Proposed chain mechanism |
| --- | --- | --- | --- |
| A: Detection | Author probabilities and separately evaluated origin probabilities | Known author and documented production history | 0 |
| B: Transformation | A revision matching a requested author/style, optionally with detector evasion | Preservation, independent style assessment, Pangram and the common three-service origin panel | 1 |

Keep independent score tables and fixed task allocations before combining
weights. Otherwise cheap detection requests or a permissive quality judge could
consume the entire reward budget. Miners may specialize; they receive credit
only in the task pools they enter. The offline pilot gives A and B equal 50%
budget shares. These are provisional allocations to test, without an established
economic optimum.

Eligibility for a task's emissions also requires fulfilling its bounded
benchmark and training service obligations. Assigned requests are free at the
point of use; serving miners bear the costs from expected emissions. The
[reciprocal service policy](#how-the-tasks-train-one-another) caps this work
before admission. Customer jobs retain their separate quoted fees.

## 1. Author and origin detection

An author task supplies a query and reference samples labeled with opaque author
IDs. The initial benchmark uses a fixed gallery of 100 authors, matching our
existing evaluation. The miner returns a probability distribution over that
gallery. Unknown-author rejection needs separately allocated unknown examples
and calibration before joining the reward.

An origin task supplies text and asks for probabilities over explicitly defined
production histories. Human-only writing, model drafts and mixed revision
workflows require different provenance records. A detector verdict, publication
date or resemblance to a person cannot establish those labels. A transformed
model draft retains its recorded production history even if every detector
misclassifies it.

Use the multiclass Brier score for each task:

```text
B = 1 - 0.5 * sum_k((p_k - y_k)^2)
```

Here `p` is the submitted probability vector and `y` is the known one-hot label.
Score author and origin tasks separately, balancing authors/source groups before
aggregation. Report author retrieval accuracy and reciprocal rank, and origin
calibration, human false-positive rate and recall at a development-fixed
threshold. Define acceptable false-positive rates before reward evaluation.

For the offline payout simulation, use positive aggregate improvement over a
fixed public baseline, scaled by that baseline's remaining headroom. A uniform
100-author predictor already receives a high raw Brier score, so raw score
normalization would give substantial credit for guessing. The probability
metric and its later conversion into relative rewards are separate operations.

Keep reference dates earlier than query dates. Separate authors across fitting,
selection and final model evaluation; separate complete source families and all
their revisions across origin splits. Match domains, assignments and length
distributions across origins. Include same-author/different-topic and
different-author/similar-topic challenges, and hold out generation families.
Known hard human examples are essential to prevent an always-AI classifier
from winning against a pool dominated by generator submissions.

Miners serve a versioned author/origin prediction interface. They can retain
`encode(text)` and `profile(samples)` internally, including proprietary grammar
features and learned embeddings. Our public starter implementation exposes named
features, denominators and missingness for reproducibility; competitors need not
publish their implementation. Validators authenticate responses and score them
against held-out labels. A declared model version does not prove which private
weights ran. Reward measured performance rather than dimension count.

## 2. Author transformation and detector evasion

A challenge supplies source text, target reference samples, intended tone,
audience, allowed style changes and preservation requirements. It declares
`author_style`, `detector_evasion` or `both`; these modes retain separate results.
The output is exact revised text bound to the source and submitted model version.

First assess preservation of claims, participants, quantities, qualifications,
citations and argument relationships. Require readability and intended tone to
meet the brief. A failed requirement earns zero transformation utility, however
good the style or detector score. An unresolved review remains pending or
unscored under the round policy; it cannot pass by default.

Among revisions that pass, measure two independent improvements over leaving
the source unchanged:

- `V`: improvement in target-style fit, assessed by frozen independent models
  and blinded target-author or reader judgments. Use distractor authors and
  matched content to expose copying of subject matter as a substitute for voice.
- `E`: joint improvement against Pangram and the pinned subnet origin detector,
  retaining both raw score changes and both pass rates separately.

For source `x` and revision `z`, define the two improvements as:

```text
D_P = min_r F_r(x) - max_r F_r(z)
D_j = H_j(x) - H_j(z); D_S = median(D_1, D_2, D_3)
E_P = max(0, D_P)
E_S = max(0, D_S)
E = (E_P + E_S) / 2 if D_P >= 0 and D_S >= 0; otherwise 0
```

`F` is Pangram's reported AI-plus-assisted fraction, using three distinct
completed committed reports per text. Each panel service returns
`H_j = p_j(model-only) + p_j(mixed)`, a document-origin probability.
Take the median of the three paired improvements, retaining all raw responses.
A strict joint pass requires preservation, `max_r F_r(z) < 0.10`, a majority
of panel services with `H_j(z) < tau_j`, and a pass against the actual strongest
qualified prior-round service. Each threshold is calibrated on held-out
development data and frozen before scoring; it is not automatically 10%.
Report panel and strongest-service pass rates separately. The strongest has
one vote in the median and cannot alone veto incremental utility.

Miners may pay for extra Pangram attempts before commitment; only committed
reports enter scoring, without mandatory disclosure of other attempts.
Selection can bias observed passes. The [repeatability probe](pangram-repeatability.md)
found equal document fractions across fresh calls for three texts, with
auxiliary-score variation, but no near-10% observation or general determinism
guarantee. A strict pass describes the committed observations.

For a first simulation, use `G*V`, `G*E` or `G*(V+E)/2` for the declared mode,
where `G` is the preservation/quality pass and each improvement is bounded in
`[0,1]`. The combined mode also requires no regression on either requested axis.
These are proposed rewards to test against human rankings, not validated
measures of usefulness. Graded development improvement is distinct from meeting
the strict joint target. Already suitable text can remain unchanged
without a fabricated improvement bonus.

Our neural author score is an auxiliary measurement here. It cannot be the
sole reward: changing *may* to *must* improved every tested target score, and
adding unrelated text reversed some local phrasing preferences.
[Edit counterexamples](edit-utility-matrix.md),
[author-score comparison](preference-score-comparison.md).

Judge the complete revision, with the same source constraints in every mode.
Copying target samples, dropping difficult passages, adding unrelated prose or
changing uncertainty must not buy a detector advantage. Miner-authored comments
inside a revision are data, not instructions to the evaluator.

### Editorial requirements within transformation

The B transformation brief can specify audience accessibility, clarity,
organization, rhetorical force or concision alongside the requested voice.
It states which aspects of voice should remain. Stronger rhetoric
does not authorize turning a qualified claim into certainty.

After preservation assessment, compare revisions with the unchanged source and
fixed editor baselines. Randomize presentation order and hide miner identity,
detector scores and author-model scores from readers. Ask readers which version
better serves the brief, retaining ties and specific fidelity failures.

Include already-good text to measure needless editing. Shorter sentences, fewer
uncommon words and a lower reading level are diagnostic measurements; their
desirability depends on the audience. Preservation and readability are gates
on B rewards. A separate writing-improvement service is deferred beyond launch.

Begin with human judgments on a small private set. Evaluate automated judges
against those labels on separate sources before using them to allocate rewards.
Retain human audits and disputes, especially for fluent revisions that lose a
qualification. A detector miner's success at origin classification gives it no
automatic authority to judge writing quality.

## How the tasks train one another

On released authorized text, A supplies author/origin probabilities for
assigned B training requests; B supplies source- and brief-constrained rewrites
for assigned A requests. Preserve production records, failed edits and human
controls. Unknown assistance histories remain weak labels. Retire evaluation
material before releasing it, and keep complete source families separate from
private evaluation. Customer jobs never enter this exchange automatically.

Freeze the global epoch budget, qualified active roster, input/output limits,
provider capacities and canonical request schedule. Serving is mandatory
within this bounded emission-eligibility obligation and free to the requester;
expected emissions need not cover each provider's costs. New UIDs and request
volume cannot expand the global budget or create reward by themselves.

Each protocol-issued requester slot consumes its next lifetime interaction
index. Hash-rank opposite-interface peer UIDs using canonical, length-delimited
`SHA256(domain, chain, netuid, requester_UID, lifetime_index, peer_UID)`, with
UID tie-breaking and exact requester-UID exclusion. Pin the eligible roster
root in the ticket, but exclude it from the hash so membership changes do not
reshuffle the relative order of remaining peers. The first of the published
small `k` peer set is mandatory; the remaining order is a fixed fallback after
recorded failure. There are no nominations, menus, skipped indices or rerolls.

The counter persists for a UID slot through registration changes, while every
ticket pins the current hotkey, generation, endpoint key and service version.
New registrations qualify on their own performance. Retries keep the same
index and response record; they count once. An abandoned issued requester
obligation consumes its index and counts against the requester, without
penalizing a provider that never received a valid payload. Capacity is reserved
in canonical slot order; each fallback stage has its own fixed deadline and
minimum response window. Block height supplies deadlines, never a requester-
selected peer seed. Exhaustion retains failure rather than producing a redraw.

Public scheduling metadata includes requester UID, index, pinned roster and
assigned providers. Text, salted commitments' openings and private evidence
remain restricted. Tickets bind authorization, sizes, deadline and nonce;
issuance and consumption require one replay-protected ledger shared by
validators. Predictable assignments allow strategic abstention and may pair
commonly owned UIDs. Hash verification establishes allocation and accounting,
not owner independence, honest private execution or resistance to all Sybils.

For each closed measured epoch, let `N` be valid assigned task obligations and
`F` explicit declines or missing valid completion by their deadlines. Each
obligation binds a UID and role: requester payload submission or provider answer
to a valid delivered request. Eligibility
requires `N > 0` and `10*F <= N`: more than 10% failures zeros earned A and B
emission credit, while exactly 10% passes this gate. Apply it in aggregate
and separately to mandatory service on every committed interface and assigned
role. Only actually assigned roles require their own gate. Paid or cheap
successes cannot dilute failed free A/B obligations, and outgoing payloads
cannot dilute provider failures. Zero traffic gives
no automatic pass. Exclude unsolicited requests, duplicates, invalid payloads
and impossible deadlines; count requester abandonment on its own obligation.
A successful fallback does not erase the preceding provider's failure.

Each mandatory interface/role carries a recovery deficit, initially zero:
`D_e = max(0, D_previous + 10*F_e - N_e)`. Normal active status and emission
eligibility require zero deficit, the current epoch's gates and qualification.
Only actual protocol-assigned probation work in that same class retires the
deficit; it does not age out, and paid work or another class cannot dilute it.
Updating once per epoch prevents banking earlier excess successes. `N=0` leaves
the deficit unchanged. Key or service-version rotation within a registration
does not reset it; new registrations require their own probation, without
establishing that they have different owners.
Dropping a capability or role cannot cancel an accrued deficit. Every
outstanding class must clear before normal admission resumes.

Use authenticated delivery evidence and validator review for disputes; an
unsupported complaint is insufficient, and unresolved work cannot default to
fulfilled. Bind each request to a window containing its actual deadline and
retain late responses and corrections. The next epoch's frozen active roster
follows completed real request records, with a bounded deterministic allocation
of real probation requests for new or recovering registrations. There are no
heartbeats or mid-epoch peer-list recomputations. Measurement must close before
weight deadlines. Document the native payout cycle and demonstrate zero-credit
behavior under reveal delays; already distributed emissions cannot be reclaimed.

Peer training/search results never directly award B. A shared comparison
counter schedules a common three-A panel: include the strongest qualified
prior-round origin service, then hash-rank two other distinct qualified A UIDs
under a separate domain. Hash-rank B entries outside the panel into the fixed-size
batch. Filter the deterministic A reserve ranking against every B batch UID and
the initial panel, then freeze the residual order and capacity before issuing
tickets. Insufficient reserve capacity prevents issuance, without redrawing B.
Exact UID self-grading exclusion therefore persists through fallback.
Capacities, qualifications and tie-breaking are
committed; no participant chooses a batch. Only exact UID exclusion is claimed.

Reserve separate benchmark capacity for the complete common source/candidate
matrix, with identical sources, briefs, modes and service settings for all B
entries. Obtain signed required responses after candidate commitment. Opaque
grading mappings and mixed human/model controls conceal which B UID produced
each text and its source/candidate role, without guaranteeing that a service
cannot infer either. A failure invokes the fixed replacement for every affected
source and candidate cell, or voids the batch. Never mix per-miner panels.
Missing actual strongest-service evidence prevents a strongest-detector pass;
any degraded comparison identifies its replacement. Disagreement alone is
neither fraud nor grounds for replacement. A failure is charged to A, not B.

The median limits one arbitrary outlier only when fewer than half the three
panel UIDs collude. That bound and the provisional panel/batch sizes need
measurement. A earns quality on held-out labels and calibration, not harsher
B scores. Signed responses do not attest which private model ran.

Hash-rank reviewers by a separate domain, chain, subnet, shared comparison
index, submission UID and eligible reviewer UID. The launch uses three distinct
reviewers and fixed reserves, excluding exact miner UIDs in the comparison.
Review every scored submission and all required checks, including the whole
revision. Encrypted evidence goes only to assigned validators, who commit
before peer openings and resolve disagreements by fixed block deadlines.
Withhold detailed active-test feedback until retirement. This is predictable
full review, with no custom drand scheduling, hidden checker or audit sample;
native chain weight commit-reveal remains unchanged. Any later sampling policy
needs a separate security and cost argument.

## Combining rewards and handling failures


Average related samples within their source groups; average groups under a
fixed task mixture. Never let additional cheap submissions increase a miner's
share. Use the same assignments and resource budgets for competing baselines.
Publish completion coverage alongside conditional quality measurements.

Apply the aggregate and mandatory interface/role availability gates before
normalizing positive skill within each logical score pool. Combine A's separately
scored author and origin subtasks under the fixed task mixture for mechanism
0's weight vector. B's transformation scores supply mechanism 1's weight vector.
The pilot allocates 50% to each mechanism, so raw metric scale cannot decide
the allocation.

If nobody beats a task baseline, mark that pool unallocated in the offline
simulation. Do not invent an on-chain escrow or assume an all-zero weight vector
is valid. The live adapter needs an explicit chain-compatible failure policy.
Verified miner timeouts and malformed submissions receive zero on their own
assigned work; validator infrastructure failures void affected assignments
consistently.
Unresolved quality reviews remain visible and require resolution before final
weights, or a predeclared whole-assignment exclusion policy.

The existing [Rust contract](../src/subnet.rs) binds a revision to a challenge
and computes an offline detector/quality reward. It does not yet implement
these task pools, private serving, replay protection or
cumulative accounting. Its [documentation](subnet.md) remains the description
of current behavior; this proposal does not silently change that contract.
The separate [receipt prototype](receipt-envelope.md) implements envelope
signatures and encryption, with trusted assignments still supplied by its caller.

## Readiness and the next experiment

| Component | Existing evidence | What the pilot must establish |
| --- | --- | --- |
| Author representation | 56.30% top-1 across 600 evaluation authors in 100-author galleries; grammar adds 2.39 points within the frozen model | Useful performance on new genres and content-matched author comparisons |
| AI-origin detection | Small matched generation experiments and detector records | Reliable provenance labels, calibration and human false-positive control across generator families |
| Transformation | Exact edit proposals, conditional grammar preferences and structural checks | Target-author preference and fidelity of actual full revisions |
| Transformation quality review | An offline human-audit interface | Reader agreement, meaningful wins over unchanged text and judge failure rates |
| Network operations | Offline Rust challenge/submission bindings | Authenticated assignments, persistent budget/replay state and valid weight submission |

The author results come from the [scale study](training-author-scale.md) and
[family ablation](full-family-ablation.md). The latest
[structural comparison](evidence-contracts.md) increased accepted saved sentence
edits from 32 to 44 of 60; this supplies no writing-quality or detector result.

Build one small local tournament before another representation sweep. Start
with 24 fresh tasks spanning A detection and B transformation, including already-good controls.
Reuse existing scorers and revision contracts, and compare unchanged text, the
current deterministic editor and a general editor on the same brief. Three hash-assigned reviewers
should label fidelity and preference without seeing reward scores. Include
deliberate score exploits such as modal strengthening, omission and irrelevant
padding. The decisive result is whether the proposed rewards rank useful
revisions above those exploits at an affordable review cost.

Use rights-cleared or consenting writing for any live miner benchmark. The Blog
Authorship Corpus currently used locally permits noncommercial research, which
does not supply permission to distribute it in a commercial subnet. Private
correspondence and its derived profiles remain local.
[Corpus source](https://u.cs.biu.ac.il/~koppel/BlogCorpus.htm).

Launch thresholds, score-pool shares, review budget and the policy for failed
rounds remain decisions for the pilot.
