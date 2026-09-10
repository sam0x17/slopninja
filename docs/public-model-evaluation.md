# Public A models and private B generation

Status: **adopted design**, September 10, 2026. A detector artifacts are public;
B generation remains private. The runner and revised benchmark protocol still
need implementation and evaluation before launch. Scope remains A detection
and B transformation.

Validators execute qualified, immutable A artifacts directly on frozen private
benchmarks. An A miner's endpoint response cannot supply an authoritative
benchmark probability. B miners submit text for independent output evaluation;
they do not have to publish their generation models or prove the generation
procedure. Pangram supplies an independent external origin-detection benchmark.
It cannot replace author-attribution labels or B's preservation review.

## What publication changes

For each round, validate and cache qualified A artifacts before revealing B's
task and starting candidate generation. Freeze the complete common panel then.
Validators execute the same common panel on the complete source/candidate
matrix, using identical settings. Record the artifact, input and output hashes
with the execution evidence. B commits its self-scores with the final text,
the assigned panel's artifact hashes and reference settings, full candidate
probability vectors, required source baselines and scalar projections.
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
or threshold after task issue. A reference execution failure leaves the common
comparison unresolved or void under the fixed closure rule; it cannot become
B nonresponse or an automatic pass. Deliberate error-triggering inputs need
testing before deployment.

B miners can query published A models locally during training and candidate
search at their own compute cost. The protocol places no per-query cap on that
local work. Public A scores for texts B already knows cannot be hidden from B.
Official evaluation still accepts one committed final candidate per assigned
task. Later A versions and fresh source families measure transfer beyond the
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
truthful provenance or absence of targeted backdoors. Preservation and requested
readability/tone constraints still need their own adjudication.

## The complete artifact

A weight file alone is insufficient. The signed manifest must bind:

- Weights, architecture, adapters, tokenizer and vocabulary, preprocessing,
  reference pooling, prompts, output schema and calibration.
- Executable source/build, dependencies, runtime, numerical precision, kernels,
  supported hardware, batching, deterministic settings and resource limits.
- All random generators, the seed schedule, decoding and stopping rules, and
  any search procedure used within A inference. These bindings govern A's
  reference computation. B's private generation seeds and internal search
  budget remain its own choice; its official final-candidate count is fixed.
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
favors their result. For public B, require an exact reference token sequence or
define how a fixed set of generated samples is evaluated. A seed alone does not
guarantee matching execution across devices or releases, as documented by
[PyTorch](https://docs.pytorch.org/docs/2.14/notes/randomness.html).

## Alternatives considered

| Option | Authoritative benchmark execution | Main tradeoff |
| --- | --- | --- |
| Public A, private B (selected) | Validators run A; B submits candidate text for independent evaluation | Removes private A score reporting while retaining B's private models |
| Public A+B | Validators run both immutable artifacts on the benchmark | Funds reusable models, but requires validator generation capacity and a defined generation policy |
| Artifacts disclosed only to validators | Authorized validators replay the models | Retains restricted distribution, but trusts recipients to keep weights confidential |

Publishing B is useful when the reward is for a transformation model's behavior
across tasks. For a reward on submitted revisions, validators already have the
candidate needed for evaluation; replaying generation adds a separate execution
requirement. Publishing both models still leaves independently labeled detection
tests and semantic review necessary.

An encrypted artifact commitment without validator access does not permit
replay. Keeping weights secret from the executing validators would require an
additional confidential-computation or proof design, with its own assumptions.

## Costs and implementation requirements

Measure artifact storage, distribution, loading and validator inference costs.
Reserve the full common panel and required review workload before admission;
public B also requires generating every scheduled candidate. Publication moves
benchmark computation to validators and does not make it free.

Hash the inference-affecting content, excluding owner signatures and wrapper
metadata. One identical content hash receives at most one A model-credit entry
and one panel seat per round. Assign that entry to the earliest finalized,
accepted artifact commitment; break same-position ties by canonical UID.
New hotkeys do not create another credit entry for that artifact. Small weight
changes and equivalent implementations remain a copying and attribution
problem that hash deduplication does not solve. Separate payment for measured
serving work from model-performance rewards. New artifact versions qualify for
later rounds and cannot replace a frozen version during evaluation.

Public artifacts do not require public customer inputs. Paid customer jobs can
retain encrypted requests to the selected provider, or use local execution if
the customer chooses. Their plaintext, keys and outputs must not enter the
benchmark replay process automatically.

Before launch, test the public A/private B design on hidden tasks, including
synthetic colluding-trigger controls. Report reference-execution agreement,
runtime cost, copying incentives and remaining monetary failure cases.
Publication permits anyone, including Pangram, to inspect A. B weights remain
private; purchased outputs can still be studied. Customer text and training
recipes do not become public merely because A's inference artifact is public.
