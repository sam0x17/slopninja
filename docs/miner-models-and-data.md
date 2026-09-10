# Public A models, private B models and training data

Design decision, 2026-09-10. A miners publish complete immutable inference
artifacts; B miners keep their generation models private. Validators qualify
and freeze A artifacts before B generation, then execute the common A panel
directly for authoritative benchmark probabilities. Miners retain raw training
data and recipes unless they contain components required for A inference.
Customers can separately buy [asynchronous inference](paid-inference.md).
The [public-model contract](public-model-evaluation.md) replaces authoritative
private A endpoint responses.

B jointly targets author-style matching and evasion against Pangram and the
subnet's qualified AI detectors. Both are main launch objectives, measured for
every B benchmark candidate. Cleanup, full meaning preservation, readability
and the frozen target tonal intent are hard requirements. We test whether
removing generic model habits improves both author fit and detector evasion;
neither result guarantees the other. Earlier single-objective modes remain
research ablations and do not define the launch interface.

The [whitepaper PDF](../whitepaper/slop_ninja.pdf)
([LaTeX source](../whitepaper/main.tex)) is the canonical design draft. Customer
sources, references, profiles and briefs are encrypted only for the assigned
miner, besides the customer's own access. Results return encrypted to the
customer. The protocol grants validators, auditors, customer-service operators
and external providers no automatic access to either. A new miner requires a
new customer-signed assignment and customer-created envelope.

A customer may separately disclose selected evidence to one explicitly chosen
validator using a customer-signed packet encrypted to that validator's
authenticated key. The packet binds the job, review context and scope and
contains the relevant evidence and commitment openings. It grants no standing
access or permission to broadcast, and the miner does not rewrap it. Partial
or redacted evidence may not open the original commitment or establish fidelity
of the whole job. See [optional inspection](paid-inference.md#optional-customer-directed-inspection).

We should provide a reproducible starter implementation and permitted training
data, then let miners change either. An A artifact must include weights,
adapters, feature definitions, tokenizer, preprocessing, pooling, calibration,
inference code, dependencies and every other component needed for its declared
reference execution. Publication rights and availability, resource limits,
test vectors and numerical disagreement rules are admission requirements.
Reserve validator storage and the complete replay budget before issue.
Publication does not establish training effort, ownership independence,
accuracy or absence of targeted behavior. A miner can improve a grammar, train
a network or combine local models. Confidential customer jobs require execution
on the assigned
miner's hardware; forwarding content to another miner or hosted API is outside
that policy. Encryption and private weights cannot prevent the assigned miner
from leaking plaintext it receives.

## Two task interfaces

| Task | Starter model and publication | Inputs | Outputs |
| --- | --- | --- | --- |
| A: Author and origin detection | Public full sparse word/grammar metric baseline; compare a public 149M text encoder with author and origin heads | Query, optional reference gallery, versioned origin-label definition | Validator-computed author probabilities, origin probabilities, calibrated abstention |
| B: Author-style matching and detector evasion | Private 8B decoder with a style adapter, deterministic edit proposals and candidate selection | Source, target writing samples, audience, purpose, target tone, allowed edits and preservation brief; frozen style evaluator and detector panel for benchmarks | Exact revision, unchanged text or abstention; private change notes; author-fit and Pangram/subnet detector evidence for authorized benchmark tasks |

Miners choose the architecture within each interface's published execution and
resource limits; only A requires public artifacts. Validators replay A and
independent validator judges evaluate B outputs on benchmark-owned
tasks under their authorized access rules. Customer evidence reaches a chosen
validator only through a separate customer-directed disclosure.
The [two-mechanism mapping](subnet-mechanisms.md) assigns author/origin detection
to mechanism 0 and joint author-style matching and detector evasion to mechanism 1, each with 50%
of pilot emissions. Fidelity, readability and the requested audience and style
controls are Task B gates and brief requirements. A separate writing-improvement
service is deferred beyond the initial launch.

The mandatory training-service flow is A requester to B provider for authorized
rewrites. A publishes artifacts and has no mandatory inference endpoint;
endpoint availability or replies do not qualify an A artifact. B executes
public A locally, then validators verify committed scores independently.
Task emission eligibility also requires valid fulfillment of actually assigned
training-service roles. Requests within that obligation carry no per-call
payment; the serving miner bears their costs in exchange for emission
eligibility. Freeze global budgets, capacities and the schedule before admission.
New UIDs and traffic create no extra total capacity or request-count rewards.

The whitepaper's [reward rules](../whitepaper/sections/07-rewards.tex) define
deterministic peer assignment using SHA256 over a canonical, length-delimited
encoding of the versioned domain, chain, netuid, requester UID, lifetime
interaction index and peer UID. For each A requester, rank eligible B providers
excluding the exact requester UID. A ticket
pins the frozen eligible roster root without including it in the peer hash.
The requester UID slot counter persists across hotkey changes and UID reuse.
The first ranked peer is mandatory; recorded failures invoke the fixed reserve
order. Requesters cannot nominate peers, skip indices or obtain another draw.
An authoritative replay-protected ledger records issuance and consumption.
Duplicates and retries keep the original index and assignment. Abandoning an
issued requester obligation consumes its index and counts as requester failure;
an undelivered payload does not count against the provider. Public scheduling
metadata leaves text and commitment openings restricted to authorized recipients.
Predictable assignments and distinct UIDs do not prove separate ownership or
prevent model extraction.

The [service-evidence rules](service-evidence.md) track activated obligations
by UID and actual mandatory role, A requester or B provider: certified successes `S`,
attributable failures `F`, unresolved work `U` and platform voids. Settled work
is `N=S+F`; availability requires `U=0`, `N>0` and `10F<=N`. More than 10%
failures gives zero earned A
and B emission credit for that measured epoch; exactly 10% passes the availability
gate. Each actually assigned A-requester or B-provider class must also pass its
mandatory-service gate with a positive assigned workload. A-provider and
B-requester detector-call classes do not exist and incur no zero-work penalty.
Outgoing payloads cannot dilute provider failures. Paid traffic and optional
hosting replies cannot dilute failed rewrite obligations. An actually assigned
class with no settled work receives no automatic service pass.
Authenticated publication and validity certificates support failure decisions;
retries count once. An invalid or unresolved request does not activate its
provider. Unused fallback reservations earn no credit. Publish unresolved and
void counts alongside settled work. Qualification uses the prior completed
request window, with bounded real
probation work for new or recovering registrations. Each registration generation
must establish its own performance even when its UID slot counter persists.
Measurement deadlines close before weight submission and reveal; the published
payout lag settles earned credit without clawing back distributed emissions.
Each actual A-requester or B-provider class also carries `D_e = max(0, D_previous + 10*F_e - N_e)`
from an initial zero. Normal eligibility requires zero deficit and current gates;
only same-class protocol probation work retires it, with no age-out or reset
through key/service-version rotation. New registrations need their own probation.
Unresolved or platform-void work cannot retire a deficit. The training-service
transport pilot does not supply validator execution of the full common detector
matrix; that capacity and schedule must be frozen before emission-bearing work.

Paid inference has its own market in subnet alpha. Miners publish signed total
quotes for bounded jobs, with capacity, expiry and delivery terms; customers
choose on quality, deadline and price. Reservation locks the quote and terms,
while future offers can change with demand and competition. There is no
owner-set base price or global utilization parameter. Quote discovery exposes
no input plaintext to bidders. The [alpha settlement adapter](paid-inference.md#offer-and-job-lifecycle)
must still be implemented and tested: allowances alone do not fund a job,
service price and network/transfer fees remain distinct, and no alpha ERC-20
interface is assumed. Customer revenue stays separate from benchmark emissions.
A miners may offer paid hosting separately, without making that endpoint an
artifact-qualification or benchmark-scoring dependency. The same encrypted
assigned-provider and optional chosen-validator privacy rules apply.

For concrete starting checkpoints, compare
[ModernBERT-base](https://huggingface.co/answerdotai/ModernBERT-base), a 149M
encoder whose authors release weights under Apache 2.0, and
[Qwen3-8B](https://huggingface.co/Qwen/Qwen3-8B), an Apache-2.0 decoder.
These are reproducible experiment candidates, not claims about the best current
models. Pin weight revisions, tokenizer and license in each experiment.
Compare the [4B](https://huggingface.co/Qwen/Qwen3-4B) and
[32B](https://huggingface.co/Qwen/Qwen3-32B) decoder sizes only after the data and
evaluation work; select using fidelity, reader preference and cost per accepted
revision. No GPU allocation or checkpoint download is part of this proposal.

### Detection and author profiles

Keep the existing full word/grammar metric as the first reference. The first
[document projection experiment](author-projection.md) tested `3557 -> 128` and
`3557 -> 256 -> 128` models. Neither replaced that reference, and both transferred
poorly to Global Voices. The reference learns 3,557 coordinate corrections while
still scoring complete sparse word and grammar distributions. Removing only
unselected contributions from its frozen scorer reduced Blog accuracy from
56.30% to 40.83% and Global Voices accuracy from 73.81% to 29.17%. Preserve that
wider feature coverage in the next learned model; the projection comparison
also changed pooling, normalization and scoring, so it cannot isolate network
architecture.
Refit vocabulary, normalization and weights using permitted training data before
a commercial starter release. The text encoder with separate author and origin
heads remains a proposed fit; the Blog-trained models remain local research.

Compare word-only, grammar-only and combined scoring in each held-out register.
In the [first Global Voices transfer test](global-voices-transfer.md), retaining
the frozen grammar contributions reduced accuracy, reversing their benefit on
Blog data. Select a starter model across sources rather than from pooled
accuracy on a single corpus.

Train the author branch on same-author and different-author pairs, with hard
negatives sharing topic and genre. Use independent works for reference and
query. At inference, form each gallery author's prototype from their references
and calibrate query-to-prototype probabilities on development authors. This
supports a new customer without retraining a classifier for their identity.
Train unknown-author rejection on authors absent from each gallery; leave it
out of rewarded production decisions until calibrated.

Train the origin head from recorded writing workflows. Start with human-only,
model-only, and mixed human/model production; keep generation, editing order
and revision logs alongside that coarse label. Model-to-model rewriting remains
model-only. Historical publication alone receives weaker provenance status.
Pangram is the bootstrap quality anchor for A's origin detection. A validated
superior subnet detector takes over; an improved Pangram becomes the target again
when it demonstrates superiority. Documented production histories
supply the labels; matching Pangram's predictions alone earns no reward. A future
span head needs separately validated alignments; workflow labels do not justify
invented token-level truth.
Neither public A probabilities nor private B endpoint responses establish an
output's assistance history. Retain unknown provenance when the production
record is insufficient.

At launch, compare Pangram and every A origin artifact being scored on a fresh
shared held-out reference set each A scoring epoch, including the incumbent and
challengers. The operator funds shared Pangram observations throughout. Freeze
selection and superiority criteria before evaluation, with common populations
and human false-positive constraints. Certified changes apply to future epochs;
ties and inconclusive results retain the incumbent, and missing Pangram evidence
cannot establish a switch. Follow the
[anchor and parity policy](../whitepaper/sections/08-pangram-audits.tex).
The quality anchor is a benchmark target. Keep the existing A Brier utility and
fixed public reward baseline when it changes; Pangram's text fraction cannot
directly substitute for an A document-origin probability.
Author evaluation still needs independent author labels. A's ongoing external
comparison remains required if a future policy reduces B candidate API frequency.

### Transformation

Use a single generator to pursue both objectives, conditioned on the source,
the target's independent samples, the editorial brief and a style profile.
Evaluate author fit and Pangram/subnet detector outcomes on every authorized
B benchmark task. The first version serializes
named word/grammar deviations and their support counts into the prompt. Compare
that with a learned profile prefix only if it improves evaluation on new authors.
Do not ask a model to interpret thousands of unexplained coordinates.

Train on approved `(source, target references, brief, accepted revision)` pairs.
Include the unchanged source and meaning-breaking candidates as rejected
alternatives. Begin with supervised fine-tuning, then fit preferences among
revisions that pass preservation review. The deterministic Rust editor supplies
small candidates and explicit checks; an LLM can propose broader changes.

The author distance helps choose candidates but cannot approve them. Our modal
experiments already showed that changing a claim can improve author proximity.
Use independent preservation checks before ranking style and detector outcomes
on authorized benchmark tasks. Each comparison batch uses a common three-A
panel: the strongest qualified public origin artifact from the previous completed A
round plus two qualified A artifacts selected by hash rank. A separate comparison domain and
shared counter fix the panel and then the B batch, excluding the exact panel
UIDs. Verify publication, cache all required artifacts and reserve validator
replay capacity before task disclosure or B generation. Require distinct panel
UIDs and inference-content hashes excluding owner/signature metadata. Identical
content receives one credit entry and panel seat, by earliest finalized
accepted commitment and canonical UID tie-break. Near copies remain an
evaluation problem. Reserves apply only before issue. After issue, neither
models nor thresholds change: a validator-node outage uses another approved
runner with the same artifact. Actual reference failure leaves the common
comparison unresolved or void, with no automatic pass or B nonresponse.
Different UIDs can share an owner.
Select the strongest artifact
by held-out origin Brier performance subject to calibration and human
false-positive limits. Every B entrant faces the same sources, briefs, panel
and settings. Freeze the qualified artifacts before B generation. B's candidate
envelope commits the exact final text, assigned panel artifact hashes, reference
settings and seed schedule, full candidate origin-probability vectors, required
source baselines and scalar score projections. B cannot substitute an easier
model, omit a panel cell or submit scores from different text or execution
settings. Validators independently execute the complete source/candidate matrix
and compare canonical outputs with the committed claims. A hash or signature
binds a claim but does not verify its numerical value. Checking still requires
the A forward passes and their reserved compute budget; private B generation
does not need replay. Preservation `G`, public style `V` and Pangram evidence
remain separate checks.
Retain input, artifact, execution and output bindings under the declared
reference execution. Validator replay errors or missing evidence follow batch
closure and cannot become B nonresponse. Report reference-artifact bootstrap
rounds separately.

Keep the standard A/B native pools; experimental shared-pool and funded-match
settlements are not adopted. Pangram's AI-plus-assisted fraction stays separate
from each panel member's
`H = p(model-only) + p(mixed)`. Subnet improvement is the median of the three
paired source-minus-candidate improvements. Joint evasion utility averages
Pangram and median panel improvement only when neither aggregate axis regresses.
The median's outlier protection assumes fewer than half the panel UIDs collude.
This detector component accompanies mandatory author-fit measurement; a lower
detector score alone cannot establish the full B result.
Strict detector success requires the semantic gate, Pangram below 10% under the committed
repeat policy, a majority of panel artifacts below their own frozen thresholds,
and a pass against the strongest qualified prior-round origin artifact. Report strongest-A
and panel success separately. The [reward rules](../whitepaper/sections/07-rewards.tex)
define the exact aggregation and treatment of missing strongest-artifact evidence.
The [settlement hypothesis](settlement-gameability.md) shows that complementary
credits do not guarantee conserved alpha payouts after native normalization.
Its credit proposal is not adopted; the [funded-match option](alpha-match-reserve.md)
is retained as an unadopted alternative for the earlier private-score design.
Validator replay establishes the declared A computation;
it cannot establish detector accuracy or rule out a committed trigger that
favors an allied B model. Hidden labels, source separation and targeted
collusion tests remain necessary. Pangram remains an external origin check and
cannot replace author-specific A evaluation or preservation review.

The [adjudication specification](reward-adjudication.md) separately fixes `V`
from a public frozen author-distance artifact and development calibration.
Source and candidate use the same empirical distance rank; `V` is exact positive
improvement divided by the source's remaining rank headroom, with zero for a
saturated source. Certified preservation, readability and tone fields determine
`G`. Source meaning must be preserved. Target tonal intent is fixed before
generation from the brief and any requested authorial style and reference samples;
it may differ from source tone. Tone review is mandatory even without a
separate tone instruction. A certified failure sets `G=0`; absent a failure, every required field must
pass for `G=1`. Unresolved judgments void the matched comparison for the B batch
without erasing independently attributable service failures.
Launch B scoring uses the combined author-fit and evasion objectives, with
the semantic gate applying to the whole revision. Single-objective studies
remain diagnostic ablations. The [reward rules](../whitepaper/sections/07-rewards.tex)
define combined utility and the distinction between incremental improvement
and strict detector success.

The validator runner withholds submitting B UIDs, source/candidate roles and
hidden labels from A's declared inference inputs, mixes controls and commits
the complete replay evidence by a
fixed deadline. Under the [audit rules](../whitepaper/sections/08-pangram-audits.tex),
three hash-assigned preparers organize each dossier. Final certificates require
distinct signers with agreeing frozen effective weight `3w>2W`. Every certifying
validator receives encrypted benchmark evidence and verifies each required
semantic field; checking the preparers' tally is insufficient. Missing,
abstaining and recused validators remain in `W`. Security assumes dishonest
eligible weight below `W/3` in that frozen set and sufficient honest participation
for closure. Validators commit final ballots before private openings; conflicting
certificates halt settlement.
Native chain weight commit-reveal remains unchanged. These checks do not attest
private B generation or establish honest majorities.

Assigned A-requester to B-provider rewrite calls use the bounded training-service obligation;
authoritative A benchmark execution uses the separate validator budget. Pangram
requires three
distinct completed reports for both source and candidate; miners fund selected
precommitment reports, and validators retrieve them without mandatory fresh
inference. Report selection can bias observed passes; the
[three-text repeatability probe](pangram-repeatability.md) does not establish
near-threshold stability or general determinism.

Confidential customer jobs use local checks and the customer's review, with no
automatic submission to Pangram, the subnet opponent or external judges. A
customer can separately provide evidence to a chosen validator.
B still pursues author fit and detector evasion for those jobs. A verified
Pangram result requires a separate customer-authorized disclosure flow outside
the confidential job protocol.
Rhetorical strength and accessibility are requested controls, with examples
and ratings in training, rather than assumed directions in the author space.

Train the transformation adapter on brief-specific edits and blinded preferences,
including fluent but unnecessary edits and already-good originals. Keep
accessibility, argument force and tone as separate labels; shortening alone is
not a quality target. Retain uncertain and tied reader judgments.

The miner's preference scorer can rank its own candidates. Benchmark validators
use the frozen public evaluator and certified semantic fields; a miner cannot
award itself emissions. Confidential customer payment follows acknowledgment and the agreed
timeout policy. A chosen validator's inspection can be advisory; its signed
opinion affects escrow only if both parties accepted that validator's authority
and dispute terms before accepting the job. Choosing a reviewer cannot change
deadlines or redirect funds. External detector measurement requires a separately
customer-initiated disclosure flow outside the default private job.

## Training material to provide

Use four distinct pools: a common training release, a public development set,
private evaluation sources, and per-customer reference writing. Miners may add
their own permitted data. Buying an inference job authorizes no training or
benchmark reuse of its text, references, profiles, brief or result. Any expanded
access or reuse requires a separate customer-initiated authorization and delivery.

The local [Blog Authorship Corpus](https://u.cs.biu.ac.il/~koppel/BlogCorpus.htm)
permits noncommercial research. Keep it and its fitted models in local research;
do not put them in the commercial miner starter package. Our private book and
correspondence also remain excluded. A software repository's license does not
automatically license the text its loader downloads.

The initial source choices and their limits are in
[the data acquisition plan](miner-training-sources.md). The principal missing
asset is a consented collection of multiple works by each writer, paired with
editorial decisions. It would serve both tasks better than collecting
unattributed web text solely for volume.

For a pilot, commission 50 writers to supply or create 12 distinct pieces each,
roughly 300-1,000 words, plus two edits of supplied drafts. Include common briefs
across writers and different subjects within each writer. Obtain explicit rights
for commercial model training, private benchmark evaluation, and any intended
redistribution separately. Preserve the agreed attribution and release scope.
These are proposed acquisition targets, not data already collected or a spend
authorization. A 50-writer pilot measures collection and judging costs; it
cannot reproduce our 100-author gallery benchmark.

If that pilot works, target 500 writers: 300 training, 100 development and 100
private evaluation authors, with at least 12 independent works each. Split
authors before feature selection. Within each development/evaluation author,
use earlier works as references and later works as queries. Reserve separate
authors for unknown-author tests. Add another cohort before claiming broad
generalization; one 100-author test gallery is narrow evidence.

Produce the three training tables from permitted source families:

| Table | How to obtain examples | Supervision |
| --- | --- | --- |
| `authorship` | Writer collection; separately qualified attributed public prose | Writer identity, provenance strength, genre, composition date, independent work |
| `production` | Human sources plus our own licensed model generations and recorded human/model edits | Workflow history, exact parent links, generator revision, edit order |
| `revision_preferences` | Writers/editors revise permitted drafts for a fixed brief; reviewers compare original and candidates | Preservation verdict, target voice, accessibility, rhetoric, tone, preference or tie |

Generate matched content across origins using both common writing briefs and
source-conditioned rewrites, and record which method produced each example.
Hold out entire prompt/source families and generator families. Keep every
revision of a work in the same split. Public source text may already appear in
foundation-model pretraining, so reserve newly commissioned writing for the
strongest contamination check.

Reverse editing supplies useful weak supervision: make a generic draft from
a writer's permitted original, then train restoration toward that original
using references from their other works. Have a reviewer check that the generic
draft retained the information needed to reconstruct it. Otherwise the target
teaches the model to invent missing details. Synthetic pairs and real writer
edits remain distinct in sampling and reporting.

On released authorized training text, B can execute public A locally for
author/origin feedback at its own compute cost without a Pangram call or an
A endpoint request. Downloading a published artifact is not a mandatory
inference ticket. Assigned B providers supply rewrites that follow the source
and brief to A requesters. Authorized retired B outputs also feed later A
training. The frozen roster and lifetime interaction indices determine
mandatory counterparts under the hash schedule. Preserve failed edits, unchanged
controls and human writing alongside successful revisions, with their actual
production labels regardless of detector scores.
Retire evaluation material before releasing it for training, release only with
permission, and keep complete source families disjoint from future private
evaluation. Keep hidden labels, unreleased sources, other miners' candidates
and preservation adjudication restricted until retirement. B can calculate
public A scores for its own text at any time. Sharing customer outputs cannot be an
entry condition or required training contribution.
Commissioned benchmark output terms should explicitly permit later use by
detector miners, while the public leaderboard discloses only aggregates.

Pangram's [model card](https://www.pangram.com/research/model-card/pangram-4)
says it does not train on customer API data. That does not make submissions
private from Pangram: the provider sees submitted text, and report readers may
see it too. Use it for authorized benchmark-owned text. Confidential customer
sources and outputs cannot be submitted or exposed through public reports
within the confidential job protocol; a customer may initiate a separate
disclosure outside that protocol.
Keeping B weights private blocks direct downloads but cannot prevent black-box
study through purchased jobs or leaked examples. A supported agreement must
cover benchmark report retrieval and any proposed reuse of detector labels;
an accessible endpoint alone supplies no such license.
[Service terms](https://www.pangram.com/terms-of-service).

## What to build next

First make a small licensed source manifest and 24 reviewed tasks across the
two interfaces. Compare the unchanged source, current deterministic edits
and one general editor under the same brief. Establish whether reviewers can
reliably detect losses of meaning before fine-tuning a reward-driven editor.
Use the writer pilot to price the larger collection.

Then refit the Rust word/grammar baseline on permitted authors and compare the
next learned model while retaining unselected feature contributions. Train one
8B transformation adapter only after approved pairs exist. Run paid-job
settlement on local/test chain with synthetic jobs before connecting customer
funds. The resulting release should include schemas, source/license manifests,
split rules and aggregate measurements. Raw private writing, live reports,
customer profiles and private B weights remain outside Git. Published A
artifacts need immutable public distribution and signed content bindings;
large weight files need not live in the source repository.
