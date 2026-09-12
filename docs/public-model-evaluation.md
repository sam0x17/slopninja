# Public A models and private B generation

Design update, September 11, 2026, whitepaper draft 0.21.
A artifacts are public and validators execute them independently. B weights
remain private; every credited B candidate, mandatory rewrite and paid B job
requires qualified attested execution. Validators verify its version-bound
receipt and evaluate the output independently, without downloading B weights.
The [whitepaper](../whitepaper/slop_ninja.pdf), including its
[model policy](../whitepaper/sections/03-models.tex) and
[customer protocol](../whitepaper/sections/05-confidentiality.tex),
defines the canonical design. The implementation remains unqualified.

Status: **adopted design**, September 10, 2026. A detector artifacts are public;
B generation remains private. The runner and revised benchmark protocol still
need implementation and evaluation before launch. Scope remains A detection
and B transformation.

B jointly targets cleanup toward an authorized writer's profile and evasion
against Pangram and the subnet's qualified AI detectors. Every launch B
benchmark evaluates both goals. Remove unwanted generic or mechanical patterns
while retaining every source argument, detail and qualification and satisfying
the target tonal intent. Match the writer's characteristic
word choices, grammatical constructions and rhetorical habits from authorized
reference writing. Preserve deliberate informality and quirks; the brief can
request a change of register, audience or rhetorical strength without imposing
a generic polished style on every writer.

The empirical target is consistency with the writer's own work, eventually to
the point that readers cannot reliably distinguish a revision from that work in
the same register. Assess this through blinded comparisons and independent
author-profile tests on held-out writing, with declared acceptance margins,
enough samples to test them, uncertainty and failure cases. No such result is
claimed today, and profile agreement would not establish human authorship.
The joint thesis is that removing generic model habits can improve author fit
and resistance to detection together. Measure both: author fit does not establish
detector evasion, and low detector scores do not establish successful cleanup.
Meaning preservation, readability, required cleanup and target tonal intent
remain hard acceptance requirements. Earlier single-objective modes are research
ablations rather than launch options.

Validators execute qualified, immutable A artifacts directly on frozen private
benchmarks. An A miner's endpoint response cannot supply an authoritative
benchmark probability. B miners submit text for independent output evaluation;
they keep their generation models private and supply the required attested
execution receipt. Receipt verification cannot replace output evaluation.
Pangram is A's bootstrap quality anchor for AI-origin detection.
A validated superior subnet detector takes over; an improved Pangram becomes the
target again when it demonstrates superiority. The anchor cannot replace
author-attribution labels or B's preservation review.

At launch, evaluate Pangram and every A origin artifact being scored on the same
fresh held-out reference set each A scoring epoch, including the incumbent and
challengers. The operator funds shared Pangram observations throughout. Freeze
selection and superiority criteria before evaluation, with common populations
and human false-positive constraints. Certified evidence can establish an anchor
change for future epochs; no change applies during an active round. Ties and
inconclusive results retain the incumbent, and missing Pangram observations
cannot establish a switch. Independently documented production histories supply
the labels; agreement with Pangram earns
no reward by itself. The quality anchor is a benchmark target; retain A's existing
Brier utility and fixed public reward baseline when it changes.
Pangram's AI-plus-assisted fraction and A's document-origin probabilities have
different meanings, so do not compare their raw Brier values without a separately
validated mapping to the same event. Any later reduction in B candidate API
frequency leaves A's continuing external comparison in place. The
[cadence and parity policy](../whitepaper/sections/08-pangram-audits.tex) defines
that obligation and the evidence required for a B cadence change.

The intended product split is free access to capable detection models and
competition in private transformation services. Pangram-level origin detection
is a benchmark target, not a measured result of this project. A submissions
must permit free artifact access, local inference and reuse for training private
B models. External providers, including Pangram, may obtain A models. Paid A
hosting remains possible because execution costs compute; B miners can retain
their transformation weights and sell inference. A training data and recipes
need not be published.
Customers can buy either A or B inference at market prices denominated in
alpha. Self-hosting gives A customers an alternative to a provider's quote;
the protocol fixes no discount or price ratio between the interfaces.

## What publication changes

For each round, validate and cache qualified A artifacts before revealing B's
task and starting candidate generation. Freeze the complete common panel then.
Validators execute the same common panel on the complete source/candidate
matrix, using identical settings. Record the artifact, input and output hashes
with the execution evidence. B commits its self-scores with the final text,
the assigned panel's artifact hashes and reference settings, full candidate
probability vectors, required source baselines and scalar projections.
These numbers record the miner's claimed result under the assigned configuration,
so validators can identify and investigate discrepancies tied to that submission.
They earn no separate credit and do not reduce required validator execution.
Validators independently recompute the canonical results; those verified
results determine rewards. Record mismatches without trusting a claimed high
score or using it to skip other candidates' verification. B cannot nominate an
easier model, omit panel cells or submit a score for different text. An A
endpoint cannot change a reported score or
withhold its reply to change the benchmark result once the artifact is available
to validators. Serving availability and customer inference remain separate
responsibilities. No A inference endpoint is required for training or scoring.
The remaining mandatory training tickets assign A as requester and B as rewrite
provider. Benchmark execution consumes reserved validator capacity. If a validator runner
fails, an approved replacement runs the same artifact. Do not change the model
or threshold after task issue. A candidate-specific reference failure withholds
only that candidate's assigned
credit. A certified shared failure affects every entry requiring that evidence.
Retain assigned aggregation weights in either case; a runner failure cannot
become B nonresponse or an automatic pass. Deliberate error-triggering inputs need
testing before deployment.

B miners can query published A models freely during training at their own
compute cost. Official evaluation requires one bounded invocation under the
frozen qualified serving profile. Generation, internal search, scoring and final
selection run inside the measured application. The task fixes the seed; repeated
execution, including after restart, must reproduce the same output under the
qualified profile. External best-of-many selection cannot supply an eligible
final candidate. Public A scores for texts B knows cannot be hidden from B.
Later A versions and fresh source families measure transfer beyond the
training detector ensemble; Pangram checks an independent external target.

Replay establishes the result of the specified computation. It does not
establish that the computation is an accurate detector. For example, A could
publish a model that returns a low model-origin probability whenever a chosen
phrase appears, and a colluding B could insert that phrase. Validators would
reproduce that behavior correctly. Publication therefore removes a private
reporting dependency while leaving targeted model behavior to independent
evaluation and the reward design.

Keep hidden, independently sourced author and provenance labels, separate
source families, and human/model controls. Freeze benchmark commitments before
evaluation, restrict active evidence to authorized validators, and retire tasks
before releasing hidden labels, other miners' candidates and detailed review
material. Evaluate artifact updates on fresh hidden material. Public weights
do not establish common ownership, original training,
truthful provenance or absence of targeted backdoors. Every B task requires
independent adjudication of meaning, readability and target tonal intent.
Fix that intent before generation from the brief and any requested authorial
style and reference samples; a style shift may call for different tone from the
source. Detector or author-fit gains cannot offset a failed
semantic gate.

## The complete artifact

A weight file alone is insufficient. The signed manifest must bind:

- Weights, architecture, adapters, tokenizer and vocabulary, preprocessing,
  reference pooling, prompts, output schema and calibration.
- Executable source/build, dependencies, runtime, numerical precision, kernels,
  supported hardware, batching, deterministic settings and resource limits.
- All random generators, the seed schedule, decoding and stopping rules, and
  any search procedure used within A inference. These bindings govern A's
  reference computation. B's qualified model manifest separately binds its
  deterministic task seed schedule and bounded internal search profile.
- Artifact availability, publication rights, test vectors, update deadlines and
  the exact reference execution used to settle numerical disagreements.

Do not expose requester/provider UIDs, A/B assignments, source/candidate roles
or hidden labels to the executable. Supply only the task's declared text,
references and brief. Use an isolated runner without network access, undeclared
files, persistent state or scheduling metadata. Text itself can still reveal
task properties; interface restrictions do not prove blindness.

For A, prefer a reference execution that produces canonical integer probability
outputs. If other hardware is allowed, freeze output tolerances and boundary
resolution before issue; validators must not choose whichever nearby score
favors their result. For private B, qualify repeatable execution within the
declared hardware/runtime
profile, including repeated instances and restarts. A seed alone does not
guarantee matching execution across devices or releases, as documented by
[PyTorch](https://docs.pytorch.org/docs/2.14/notes/randomness.html).

## Alternatives considered

| Option | Authoritative benchmark execution | Main tradeoff |
| --- | --- | --- |
| Public A, attested private B (selected) | Validators run A, verify B execution receipts and independently evaluate B text | Retains private B weights; B requires a qualified confidential runtime |
| Public A+B | Validators run both immutable artifacts on the benchmark | Funds reusable models, but requires validator generation capacity and a defined generation policy |
| Artifacts disclosed only to validators | Authorized validators replay the models | Retains restricted distribution, but trusts recipients to keep weights confidential |

Publishing B is useful when the reward is for a transformation model's behavior
across tasks. For a reward on submitted revisions, validators already have the
candidate needed for evaluation; replaying generation adds a separate execution
requirement. Publishing both models still leaves independently labeled detection
tests and semantic review necessary.

An encrypted artifact commitment alone does not prove execution. The selected
B design adds independently verified confidential execution and version-bound
receipts, with hardware and audited-runtime assumptions. Validators replay the
public scoring models; they do not reproduce private B generation.

## Costs and implementation requirements

Measure artifact storage, distribution, loading and validator inference costs.
Reserve the full common panel and required review workload before admission.
B miners fund qualified generation and mandatory rewrite capacity. Validators
fund public scoring, attestation checks and independent review. Measure these
costs separately from any future customer revenue.

Hash the inference-affecting content, excluding owner signatures and wrapper
metadata. One identical content hash receives at most one A model-credit entry
and one panel seat per round. Assign that entry to the earliest finalized,
accepted artifact commitment; break same-position ties by canonical UID.
Registration uses a domain-separated hash commitment to the miner's
registration identity, round, canonical content digest and a fresh random
256-bit nonce. Registration fields and nonce are excluded from the
inference-content identity; keep the nonce secret until opening.
For previously unreleased content, finalize the
commitment before disclosing its digest, manifest or retrievable artifact.
The round fixes a publication deadline before qualification and panel selection.
By that deadline, open the commitment and provide the complete artifact.
Validators verify the opening and content digest and complete qualification;
only accepted entries retain their original commitment's finalization priority.
Missing or invalid publication reserves no credit, panel seat or priority in
later rounds. Already-public artifacts and foundation-model components remain
admissible. This assigns duplicate credit without establishing model authorship
or training priority.

New hotkeys do not create another credit entry for that artifact. Small weight
changes and equivalent implementations remain a copying and attribution
problem that hash deduplication does not solve. Separate payment for measured
serving work from model-performance rewards. New artifact versions qualify for
later rounds and cannot replace a frozen version during evaluation.

Public artifacts do not require public customer inputs. Paid B customers encrypt
to a verified job key inside the qualified runtime, withholding input keys from
the miner and compute operator. An ordinary paid A endpoint can read its input
with the customer's explicit acceptance; customers may run public A locally.
B pursues author fit and detector evasion in
private jobs without automatically uploading text to Pangram. External
measurement requires a separate customer-authorized disclosure flow outside
the confidential job protocol. Customer plaintext, keys and outputs must not
enter the benchmark replay process automatically.

Before launch, test the public A/private B design on hidden tasks, including
synthetic colluding-trigger controls. Report reference-execution agreement,
runtime cost, copying incentives and remaining monetary failure cases.
Publication permits anyone, including Pangram, to inspect A. B weights remain
private; purchased outputs can still be studied. Customer text and training
recipes do not become public merely because A's inference artifact is public.
