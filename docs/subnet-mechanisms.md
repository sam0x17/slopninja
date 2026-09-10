# Slop Ninja subnet mechanisms

Status: public A/private B design approved, 2026-09-10. We have enough evidence to define an
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

A miners publish complete immutable inference artifacts. B miners keep their
generation models private and serve authenticated task requests.
The [model and data plan](miner-models-and-data.md) defines starter architectures
and training records; the [paid inference proposal](paid-inference.md) adds
customer jobs with on-chain settlement. Emissions reward observed performance,
while customers pay separately for service. A publication includes every
component needed for the frozen reference execution; raw training data and
recipes can remain private. The [public-model contract](public-model-evaluation.md)
requires qualification, artifact availability and validator replay capacity
before B generation.

Evasion targets Pangram and a common panel of qualified subnet origin
detectors, including the strongest from the previous completed A round.
Compare detectors against documented production histories; neither detector
agreement nor a public peer identity supplies ground truth. The
[API verification and funding design](pangram-oracle.md) specifies the
external dependency and its remaining trust assumptions.

The standard A/B native pools remain the proposed launch settlement. The
[settlement hypothesis](settlement-gameability.md) conserves complementary policy
credits but finds a counterexample after simplified native normalization; that
credit proposal and experimental shared-pool settlement are not adopted. The
[funded-alpha option](alpha-match-reserve.md) also remains unadopted.

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
| A: Detection | Public immutable author/origin inference artifact | Validators execute it against known authors and documented production histories | 0 |
| B: Transformation | A revision from a private model matching a requested author/style, optionally with detector evasion | Preservation, independent style assessment, Pangram and validator execution of the common three-artifact origin panel | 1 |

Keep independent score tables and fixed task allocations before combining
weights. Otherwise cheap detection requests or a permissive quality judge could
consume the entire reward budget. Miners may specialize; they receive credit
only in the task pools they enter. The offline pilot gives A and B equal 50%
budget shares. These are provisional allocations to test, without an established
economic optimum.

Mandatory training-service tickets run only from A requesters to B providers
for authorized rewrites. A publishes artifacts and has no mandatory inference
endpoint; provider endpoint service does not qualify an A artifact. Eligibility
also requires fulfilling actually assigned training-service roles. Requests are free at the
point of use; serving miners bear the costs from expected emissions. The
[training-service policy](#how-the-tasks-train-one-another) caps this work
before admission. A miners may offer paid hosting separately; customer jobs
retain their quoted fees, assigned-provider encryption and optional disclosure
to one chosen validator.

## 1. Author and origin detection

An author task supplies a query and reference samples labeled with opaque author
IDs. The initial benchmark uses a fixed gallery of 100 authors, matching our
existing evaluation. Validators execute the frozen A artifact to obtain a probability distribution over that
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

Here `p` is the validator-computed probability vector and `y` is the known one-hot label.
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

Miners publish a versioned author/origin inference artifact, including
`encode(text)` and `profile(samples)` when used, grammar
features and learned embeddings. Our public starter implementation exposes named
features, denominators and missingness for reproducibility. Every competing A
artifact must supply its complete inference implementation, weights, tokenizer,
preprocessing, calibration, dependencies and reference-execution rules.
Validators execute those artifacts against held-out labels and retain exact
input/output bindings. Reward measured performance rather than dimension count.

## 2. Author transformation and detector evasion

A challenge supplies source text, target reference samples, intended tone,
audience, allowed style changes and preservation requirements. It declares
`author_style`, `detector_evasion` or `both`; these modes retain separate results.
The output is exact revised text bound to the source and submitted model version.
Its candidate envelope also commits the protocol-assigned A panel artifact
hashes, reference settings and seed schedule, every candidate origin-probability
vector, source baselines where required, and scalar projections. The panel is
pinned for the full batch before B task disclosure. B cannot choose an easier
model, omit cells or use a score for another text or seed. Validators rerun A
and compare canonical outputs; they do not replay B's private generator.

The [adjudication specification](reward-adjudication.md) defines the whole-revision
preservation, readability and tone fields. A valid global failure certificate for
any required field sets `G=0`; otherwise every required field needs a pass
certificate for `G=1`. Empty tone constraints are satisfied deterministically.
If no field has a failure certificate and a required pass certificate is missing,
`G` remains unset and the matched source/brief/mode comparison is void for the
entire B batch. Conflicting certificates halt settlement.
None of these outcomes erases independently attributable service failures.

Among revisions that pass, measure two independent improvements over leaving
the source unchanged:

- `V`: exact positive improvement in a public frozen author-distance rank,
  divided by the source's remaining rank headroom. The development calibration
  multiset, target references and evaluator are shared by source and candidate.
- `E`: joint improvement against Pangram and the pinned subnet origin detector,
  retaining both raw score changes and both pass rates separately.

For style tasks, [the exact rank rule](reward-adjudication.md#frozen-numerical-inputs)
uses `Q=1,000,000`, half credit for calibration ties before flooring, and
`V=max(0,q_z-q_x)/(Q-q_x)` when `q_x<Q`. A saturated source earns zero.
Retain exact reduced rationals through aggregation. Evasion-only tasks record
style as `not_requested`. Private miner scores never define this evaluator.

For source `x` and revision `z`, the research evasion formula is:

```text
D_P = min_r F_r(x) - max_r F_r(z)
D_j = H_j(x) - H_j(z); D_S = median(D_1, D_2, D_3)
E_P = max(0, D_P)
E_S = max(0, D_S)
E = (E_P + E_S) / 2 if D_P >= 0 and D_S >= 0; otherwise 0
```

`F` is Pangram's reported AI-plus-assisted fraction, using three distinct
completed committed reports per text. Validator execution of each A artifact produces
`H_j = p_j(model-only) + p_j(mixed)`, a document-origin probability.
Take the median of the three paired improvements, retaining all raw responses.
A strict joint pass requires preservation, `max_r F_r(z) < 0.10`, a majority
of panel artifacts with `H_j(z) < tau_j`, and a pass against the actual strongest
qualified prior-round artifact. Each threshold is calibrated on held-out
development data and frozen before scoring; it is not automatically 10%.
Report panel and strongest-artifact pass rates separately. The strongest has
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
The standard A/B native pools remain separate; shared-pool experiments do not
change that settlement. A public model can contain behavior that favors an
allied B miner, and validators will reproduce it. Publication removes private
A score reporting but supplies no accuracy or ownership-independence guarantee.
Hidden-label evaluation and targeted collusion tests remain required.
Graded development improvement is distinct from meeting
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

On released authorized text, B can execute public A artifacts locally for
author/origin feedback at its own compute cost, without a Pangram call. B can
also calculate scores for its own candidate text before commitment; those scores
are not secret. B supplies source- and brief-constrained rewrites
for assigned A requests. Preserve production records, failed edits and human
controls. Unknown assistance histories remain weak labels. Retire evaluation
material before releasing it, and keep complete source families separate from
private evaluation. Authorized retired B outputs feed later A training.
B weights, raw training data and recipes remain private. Customer jobs never
enter this exchange automatically. A has no mandatory inference service;
artifact downloads and B's local A execution create no detector-call tickets.
Validators perform authoritative benchmark A execution separately.

Freeze the global epoch budget, qualified active roster, input/output limits,
qualified B-provider capacities and canonical A-requester schedule. B serving
is mandatory
within this bounded emission-eligibility obligation and free to the requester;
expected emissions need not cover each provider's costs. New UIDs and request
volume cannot expand the global budget or create reward by themselves.

Each protocol-issued A-requester slot consumes its next lifetime interaction
index. Hash-rank qualified B-provider UIDs using canonical, length-delimited
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

The [service-evidence protocol](service-evidence.md) records each activated
obligation as certified success `S`, attributable failure `F`, unresolved `U`
or platform void `V` (this accounting field is separate from style utility).
Settled obligations are `N=S+F`. Each obligation binds a UID and one actual
mandatory role: A requester publication or B provider answer after certified
valid input. Invalid
or unresolved input does not activate a provider; unused fallback reservations
earn no credit. Eligibility requires `U=0`, `N>0` and `10*F<=N`:
more than 10% failures zeros earned A and B
emission credit, while exactly 10% passes this gate. Apply it in aggregate
and separately to each actually assigned A-requester or B-provider class.
A-provider and B-requester detector-call classes do not exist and receive no
zero-work penalty. Paid or optional hosting successes cannot dilute failed
training-service obligations, and outgoing payloads cannot dilute provider
failures. An actually assigned class with no settled work receives no automatic
service pass. Exclude unsolicited requests, duplicates and impossible
deadlines. Invalid input is a requester failure, never a provider failure;
count requester abandonment on its own obligation. Publish `U` and platform
void counts even though they are outside settled `N`.
A successful fallback does not erase the preceding provider's failure.

Each actual A-requester or B-provider class carries a recovery deficit, initially zero:
`D_e = max(0, D_previous + 10*F_e - N_e)`. Normal active status and emission
eligibility require zero deficit, the current epoch's gates and qualification.
Only actual protocol-assigned probation work in that same class retires the
deficit; it does not age out, and paid work or another class cannot dilute it.
Updating once per epoch prevents banking earlier excess successes. `N=0` leaves
the deficit unchanged. Unresolved and platform-void work cannot retire it.
Key or service-version rotation within a registration
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
prior-round public origin artifact, then hash-rank two other distinct qualified
A UIDs under a separate domain. Hash-rank B entries outside the panel into the
fixed-size batch. Freeze and cache complete qualified artifacts, thresholds,
panel and resource budget before B task disclosure or generation. Require
distinct UIDs and inference-content hashes excluding owner/signature metadata.
Identical content receives one credit entry and panel seat, by earliest finalized
accepted commitment and canonical UID tie-break. Near copies remain an
evaluation problem. Reserves apply only before issue. Publication and artifact
availability are admission requirements. Insufficient validator capacity prevents issue;
no participant chooses a batch. Only exact UID exclusion is claimed.

Reserve validator execution for the complete common source/candidate matrix,
with identical sources, briefs, modes and inference settings for all B entries.
Validators execute the fixed artifacts after candidate commitment, retaining
artifact hashes, exact input bindings and reference-execution outputs. They
compare the full canonical outputs and derived projections with B's committed
self-scores. Claimed scores, hashes and signatures do not establish correct
inference. Verification still requires the full A forward-pass workload;
preservation `G`, public style `V` and Pangram reports remain separate. The
isolated runner exposes no undeclared UID, hidden-label, source/candidate-role
or scheduling inputs. Text may still reveal task properties. An A endpoint
reply or timeout cannot determine a score or trigger an artifact fallback.
No model or threshold changes after issue. A validator-node outage uses another
approved runner with the same artifact; actual reference failure leaves the
common comparison unresolved or void under fixed batch closure. It cannot
become B nonresponse or an automatic pass. Missing strongest-artifact evidence prevents
the corresponding strict pass. Never mix per-miner panels.

The median limits one arbitrary outlier only when fewer than half the three
panel UIDs collude. That bound and the provisional panel/batch sizes need
measurement. A earns quality on held-out labels and calibration, not harsher
B scores. Validator replay establishes the declared A computation; it cannot
rule out committed triggers. Pangram supplies an external origin check and
cannot replace author-specific A evaluation or preservation review.

Hash-rank three evidence preparers using the separate review domain and frozen
assignment fields. They organize dossiers and disagreements but cannot finalize
verdicts. Every frozen eligible validator receives authorized encrypted benchmark
evidence. Each signer verifies the required evidence and semantic field itself.
The proof-verified snapshot fixes integer effective weights and total `W` before
task disclosure; agreeing signer weight must satisfy strict `3w>2W`. Missing,
recused, abstaining and nonresponsive validators remain in `W`.

The conditional assumption is dishonest weight strictly below `W/3` within that
frozen eligible set, with sufficient honest participation to close. Honest
signers do not authorize conflicting final verdicts. Certificates attest scoped
judgments, without proving meaning preservation or private B generation.
The [adjudication deadlines](reward-adjudication.md#fixed-pilot-deadlines) govern
preparation, final ballots, private openings and closure. Keep hidden labels,
unreleased sources, other miners' candidates and preservation adjudication
restricted until retirement. B can compute A scores on text it possesses.
Native chain weight commit-reveal remains unchanged.
The [training-service pilot](service-evidence.md) tests transport and accounting;
a full common-panel benchmark needs its own frozen matrix schedule and capacity.

## Combining rewards and handling failures


Average related samples within their source groups; average groups under a
fixed task mixture. Never let additional cheap submissions increase a miner's
share. Use the same assignments and resource budgets for competing baselines.
Publish completion coverage alongside conditional quality measurements.

In the research reward simulation, apply the aggregate and actually assigned
A-requester/B-provider availability and zero-deficit gates before
normalizing positive skill within each logical score pool. Combine A's separately
scored author and origin subtasks under the fixed task mixture for mechanism
0's weight vector. B's transformation scores supply mechanism 1's weight vector.
The pilot allocates 50% to each mechanism, so raw metric scale cannot decide
the allocation.

If nobody beats a task baseline, mark that pool unallocated in the offline
simulation. The [proposed live fallback](../whitepaper/sections/07-rewards.tex)
directs its weight to a verified owner-associated hotkey whose miner incentive
is withheld under the native burn/recycle rule. Pin its UID, registration
generation, owner association and burn setting before submission; revalidate
them for the payout cycle. The configuration must allow one nonzero weight
and a full-weight destination. The latter cap is root-controlled, so owner
configuration alone cannot establish it. Unmet conditions block this adapter's
launch; neither zero weights nor a stale earned vector substitutes for the
fallback. Burning can reduce future subnet emissions. Quantization, UID changes,
native reveal timing and actual payout behavior require pinned-runtime tests.
Application certificates specify intended weights; native consensus determines
payouts. Experimental shared-pool settlement remains unadopted.

Verified miner failures receive zero on their assigned work. Platform incidents
and unresolved quality comparisons follow the [service-evidence](service-evidence.md)
and [adjudication](reward-adjudication.md) closure rules, retaining all missingness
and independently attributable failures.

The existing [Rust contract](../src/subnet.rs) binds a revision to a challenge
and computes an offline detector/quality reward. It does not yet implement
these task pools, public A artifact execution, private B serving, replay protection or
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
current deterministic editor and a general editor on the same brief. Three
hash-assigned preparers should organize the evidence for the global semantic
certificates, with review blinded to reward scores. Include
deliberate score exploits such as modal strengthening, omission and irrelevant
padding. The decisive result is whether the proposed rewards rank useful
revisions above those exploits at an affordable review cost.

Use rights-cleared or consenting writing for any live miner benchmark. The Blog
Authorship Corpus currently used locally permits noncommercial research, which
does not supply permission to distribute it in a commercial subnet. Private
correspondence and its derived profiles remain local.
[Corpus source](https://u.cs.biu.ac.il/~koppel/BlogCorpus.htm).

Concrete evaluator artifacts, full benchmark capacity, native adapter behavior
and adversarial funding remain launch requirements.
