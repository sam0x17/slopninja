# Private miner models and training data

Design decision, 2026-09-09. Miners retain their trained weights, adapters,
feature definitions and training recipes. They serve three versioned task
interfaces and earn emissions for measured performance. Customers can separately
buy [asynchronous inference](paid-inference.md). Publishing a winning model is
optional. This replaces the earlier proposal to execute submitted miner
artifacts on validators.

We should provide a reproducible starter implementation and permitted training
data, then let miners change either. The subnet evaluates service outputs;
it cannot establish private training effort, model ownership or architecture
from those outputs. A miner can improve a grammar, train a network, combine
models or buy another service, subject to its advertised customer terms.

## Three task interfaces

| Task | Private starter model | Inputs | Outputs |
| --- | --- | --- | --- |
| Author and origin detection | Word/grammar metric baseline; compare a small projection network and a 149M text encoder | Query, optional reference gallery, versioned origin-label definition | Author probabilities, origin probabilities, calibrated abstention |
| Author transformation and detector evasion | 8B decoder with a style adapter, deterministic edit proposals and candidate selection | Source, target writing samples, style changes, preservation brief, evasion mode | Exact revision or abstention; private evidence and required detector reports |
| Writing improvement | The same decoder base with a separate editorial adapter and preference scorer | Source, audience, purpose, tone, allowed edits and preservation brief | Exact revision or unchanged text, with private change notes |

These are three functional models; miners need not maintain three independent
foundation models. Sharing a base between editing adapters reduces the starting
cost. Independent validator judges remain outside all three miner interfaces.
The current [two-mechanism mapping](subnet-mechanisms.md) remains: detection in
mechanism 0, separately scored transformation and quality pools in mechanism 1.

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

Keep the existing 3,557-coordinate word/grammar representation as the first
ablation. Refit vocabulary, normalization and weights using the new training
split. Compare its diagonal metric with a `3557 -> 256 -> 128` projection,
and then a text encoder with separate author and origin heads. All are proposed
fits; our existing Blog Corpus result is a local research baseline and is not
evidence for these new models or commercial data.

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
Use independent preservation checks before ranking style and Pangram outcomes.
Rhetorical strength and accessibility are requested controls, with examples
and ratings in training, rather than assumed directions in the author space.

### Writing improvement

Train the editorial adapter on brief-specific edits and blinded preferences,
including fluent but unnecessary edits and already-good originals. Keep
accessibility, argument force and tone as separate labels; shortening alone is
not a quality target. Retain uncertain and tied reader judgments.

The miner's preference scorer can rank its own candidates. Validators use
different frozen judges and reader audits. A miner must not score itself for
payment. The quality task does not require a Pangram improvement unless the
customer explicitly adds detector measurement as a second task.

## Training material to provide

Use four distinct pools: a common training release, a public development set,
private evaluation sources, and per-customer reference writing. Miners may add
their own permitted data. Customer text and profiles are private inference
inputs by default, with a separate opt-in for training or benchmark reuse.

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

For adversarial training, preserve failed edits, unchanged controls and good
human writing alongside successful revisions. Retire evaluation material before
releasing it for training, and only release samples with permission. Miners
retain proprietary customer outputs; sharing those cannot be an entry condition.
Commissioned benchmark output terms should explicitly permit later use by
detector miners, while the public leaderboard discloses only aggregates.

Pangram's [model card](https://www.pangram.com/research/model-card/pangram-4)
says it does not train on customer API data. Its reports still reveal submitted
outputs to the provider and authorized auditors. Keeping weights private blocks
direct downloads, but cannot prevent black-box study through purchased jobs or
leaked examples. A supported agreement must cover subnet report retrieval and
any proposed reuse of detector labels for training; an accessible endpoint
alone supplies no such license.
[Service terms](https://www.pangram.com/terms-of-service).

## What to build next

First make a small licensed source manifest and 24 reviewed tasks across the
three interfaces. Compare the unchanged source, current deterministic edits
and one general editor under the same brief. Establish whether reviewers can
reliably detect losses of meaning before fine-tuning a reward-driven editor.
Use the writer pilot to price the larger collection.

Then refit the Rust word/grammar baseline on permitted authors and compare the
small projection. Train one 8B editing adapter only after approved pairs exist;
separate the two adapters when their requested behavior conflicts. Run paid-job
settlement on local/test chain with synthetic jobs before connecting customer
funds. The resulting release should include schemas, source/license manifests,
split rules and aggregate measurements. Raw private writing, live reports,
customer profiles and miner weights remain outside Git.
