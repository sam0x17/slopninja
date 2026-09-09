# Private miner models and training data

Design decision, 2026-09-09. Miners retain their trained weights, adapters,
feature definitions and training recipes. They serve three versioned task
interfaces and earn emissions for measured performance. Customers can separately
buy [asynchronous inference](paid-inference.md). Publishing a winning model is
optional. This replaces the earlier proposal to execute submitted miner
artifacts on validators.

The [whitepaper PDF](../whitepaper/slopninja.pdf)
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
data, then let miners change either. The subnet evaluates service outputs;
it cannot establish private training effort, model ownership or architecture
from those outputs. A miner can improve a grammar, train a network or combine
local models. Confidential customer jobs require execution on the assigned
miner's hardware; forwarding content to another miner or hosted API is outside
that policy. Encryption and private weights cannot prevent the assigned miner
from leaking plaintext it receives.

## Three task interfaces

| Task | Private starter model | Inputs | Outputs |
| --- | --- | --- | --- |
| Author and origin detection | Full sparse word/grammar metric baseline; compare a 149M text encoder with author and origin heads | Query, optional reference gallery, versioned origin-label definition | Author probabilities, origin probabilities, calibrated abstention |
| Author transformation and detector evasion | 8B decoder with a style adapter, deterministic edit proposals and candidate selection | Source, target writing samples, style changes, preservation brief, evasion mode | Exact revision or abstention; detector reports only for authorized benchmark tasks |
| Writing improvement | The same decoder base with a separate editorial adapter and preference scorer | Source, audience, purpose, tone, allowed edits and preservation brief | Exact revision or unchanged text, with private change notes |

These are three functional models; miners need not maintain three independent
foundation models. Sharing a base between editing adapters reduces the starting
cost. Independent validator judges evaluate benchmark-owned tasks under their
authorized access rules. Customer evidence reaches a chosen validator only
through a separate customer-directed disclosure.
The current [two-mechanism mapping](subnet-mechanisms.md) remains: detection in
mechanism 0, separately scored transformation and quality pools in mechanism 1.

Paid inference has its own market in subnet alpha. Miners publish signed total
quotes for bounded jobs, with capacity, expiry and delivery terms; customers
choose on quality, deadline and price. Reservation locks the quote and terms,
while future offers can change with demand and competition. There is no
owner-set base price or global utilization parameter. Quote discovery exposes
no input plaintext to bidders. The [alpha settlement adapter](paid-inference.md#offer-and-job-lifecycle)
must still be implemented and tested: allowances alone do not fund a job,
service price and network/transfer fees remain distinct, and no alpha ERC-20
interface is assumed. Customer revenue stays separate from benchmark emissions.

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
Pangram supplies a comparison, not the label. A future span head needs separately
validated alignments; workflow labels do not justify invented token-level truth.

### Transformation

Use a single generator conditioned on the source, the target's independent
samples, the editorial brief and a style profile. The first version serializes
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
on authorized benchmark tasks. Evasion targets both Pangram and the strongest
eligible independent subnet origin service from the previous completed round,
pinned before the transformation round. Select it by held-out origin Brier
performance subject to a human false-positive limit, with self-scoring excluded
and a predeclared fallback; author-identification rank is not a substitute.
Controls and probing monitor drift, without attesting the service's private
weights. Until a competitive incumbent qualifies, use a named reference detector
for bootstrap rounds and report those results separately.

Keep Pangram's AI-plus-assisted fraction and the subnet detector's
`H = p(model-only) + p(mixed)` separate. Joint evasion utility averages their
positive improvements only when neither raw axis regresses. Strict joint success
requires Pangram below 10% under the committed repeat policy and `H` below its
own development-calibrated, frozen threshold, plus preservation. The
[mechanism specification](subnet-mechanisms.md#2-author-transformation-and-detector-evasion)
defines the exact aggregation. Required subnet benchmark calls have an explicit
round budget; private search is miner-funded. The coordinator obtains the required
signed subnet candidate response after candidate commitment and commits it before
the audit beacon; private search responses cannot replace it. Selected Pangram
reports may come from miner-paid precommitment attempts and are checked through
GETs without new Pangram inference charges.

Confidential customer jobs use local checks and the customer's review, with no
automatic submission to Pangram, the subnet opponent or external judges. A
customer can separately provide evidence to a chosen validator.
Rhetorical strength and accessibility are requested controls, with examples
and ratings in training, rather than assumed directions in the author space.

### Writing improvement

Train the editorial adapter on brief-specific edits and blinded preferences,
including fluent but unnecessary edits and already-good originals. Keep
accessibility, argument force and tone as separate labels; shortening alone is
not a quality target. Retain uncertain and tied reader judgments.

The miner's preference scorer can rank its own candidates. Benchmark validators
use different frozen judges and reader audits; a miner cannot award itself
emissions. Confidential customer payment follows acknowledgment and the agreed
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
editorial decisions. It would serve all three tasks better than collecting
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

For adversarial training on authorized benchmark material, preserve failed
edits, unchanged controls and good human writing alongside successful revisions.
Retire evaluation material before releasing it for training, and only release
samples with permission. Route authorized retired revisions into later detector
training and disjoint private evaluation source families, retaining their actual
production labels regardless of detector scores. This feedback connects the
detection and transformation competitions. Sharing customer outputs cannot be an entry condition
or required training contribution.
Commissioned benchmark output terms should explicitly permit later use by
detector miners, while the public leaderboard discloses only aggregates.

Pangram's [model card](https://www.pangram.com/research/model-card/pangram-4)
says it does not train on customer API data. That does not make submissions
private from Pangram: the provider sees submitted text, and report readers may
see it too. Use it for authorized benchmark-owned text. Confidential customer
sources and outputs cannot be submitted or exposed through public reports.
Keeping weights private blocks direct downloads but cannot prevent black-box
study through purchased jobs or leaked examples. A supported agreement must
cover benchmark report retrieval and any proposed reuse of detector labels;
an accessible endpoint alone supplies no such license.
[Service terms](https://www.pangram.com/terms-of-service).

## What to build next

First make a small licensed source manifest and 24 reviewed tasks across the
three interfaces. Compare the unchanged source, current deterministic edits
and one general editor under the same brief. Establish whether reviewers can
reliably detect losses of meaning before fine-tuning a reward-driven editor.
Use the writer pilot to price the larger collection.

Then refit the Rust word/grammar baseline on permitted authors and compare the
next learned model while retaining unselected feature contributions. Train one
8B editing adapter only after approved pairs exist;
separate the two adapters when their requested behavior conflicts. Run paid-job
settlement on local/test chain with synthetic jobs before connecting customer
funds. The resulting release should include schemas, source/license manifests,
split rules and aggregate measurements. Raw private writing, live reports,
customer profiles and miner weights remain outside Git.
