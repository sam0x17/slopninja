# Generator attribution and broader model coverage

Treat generating models as identifiable author classes, separately from human
authorship, prompt personas and Pangram scores. A detector score is an annotation
of the text; it does not determine which model produced it.

The current Train annotations cover original Qwen, Mistral and OLMo drafts,
human-text edits, and Qwen/Mistral revision chains. Expand coverage with GPT and
Claude through the user's authenticated CLIs on the Studio. Keep model variants
separate whenever the recorded identifiers distinguish them. The reserved
Pangram spending ceiling remains $150 cumulatively. Use the remaining Train
allocation for broader generators before collecting more of the original three.

## Labels and evidence

Every annotation can carry `slop_ninja_generator_attribution_v1` metadata:

- Requested model, reported model, declared revision, runtime and quantization.
- A generator label derived from the recorded model identifiers, independent of
  prompt style. Keep any later alias reconciliation in an explicit registry.
- Source family and full ancestry, ordered from source root to final text.
- Original draft generator, final model stage, and every intervening model step.
- Operation, prompt hash and profile, generation settings, timestamp, and hashes
  of the archived request and response.

Keep labels for original generator and final reviser independently. A GPT draft
revised by Claude remains a two-model workflow; it is not a clean Claude-only
authorship example. A model edit of a human source keeps its mixed-origin label.
Requested personas and style instructions are covariates, not new authors.

The `attribution` module derives this information from the existing validated
corpus. New annotation plans include it directly. `annotation_weights` also
derives it for archived observations without rewriting old plans or receipts.

Hosted aliases do not establish immutable model weights. Capture the requested
alias, any model identifier actually returned by the service, CLI version and
collection date. Leave unexposed temperatures, seeds, token limits and snapshot
identifiers unknown. In particular, a successful Codex CLI capture does not
justify inventing a provider snapshot or treating a requested temperature as
an observed setting. Raw CLI captures need a separate admission path before
joining the current open-weight corpus schema and commercial release lineage.

## Collection and evaluation

Use the same source/style combinations across providers. The next source set
contains 24 previously unannotated Train families: one per source collection and
prompt profile, selected with the existing deterministic sampler and exclusions
for all four Pangram cohorts. It contains 72 existing seed records, SHA256
`a956aad4715d76975ec8e884475d8c396d44af3e490cc257902c5571410c3ddf`.
Preserve each CLI's exact input, output, completion status and usage. Disable
unrequested tools and custom context; keep any unavoidable harness differences
in the runtime metadata. Preserve unsuccessful and rejected attempts.

Train a future attribution head on generator identities and report family-level
and variant-level results separately. Split by source family before fitting:
all provider versions of one source belong in the same partition. Evaluate on
held-out topics and prompt profiles as well as held-out source families, so a
model cannot earn attribution accuracy from a provider-specific subject or style
instruction. Keep cross-model revisions as their own evaluation task.

The current expansion remains adaptive Train collection. It does not change
the frozen v6 fit or its final Test procedure. A score-balanced weighting view
retains every observation, gives equal total weight to each observed origin,
then to each occupied score bin within that origin. Publish the cell counts and
effective sample size alongside it: weighting rare bins supplies no additional
independent examples. This view is prepared for a later declared experiment;
it is not evidence of an improved fitted model.

CLI interfaces were checked against [Codex non-interactive mode](https://learn.chatgpt.com/docs/non-interactive-mode)
and [Claude's CLI reference](https://code.claude.com/docs/en/cli-reference).

The matched capture run started on the Studio on 2026-09-12. It requests 24
drafts each from `gpt-6-astra`, `gpt-5.6-sol`, Claude `opus` and Claude `sonnet`
over the same frozen sources and profiles. The runner passed its parser tests,
format check and focused Clippy check before starting. The repository's full
Rust tests and all-targets Clippy check also passed. Capture
counts, successful completion and corpus admission remain separate milestones.
This job makes no Pangram calls. The earlier interface probes are retained
separately and do not replace any member of the matched cohort.
