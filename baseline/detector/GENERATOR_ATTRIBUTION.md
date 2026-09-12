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

`audit_cli_captures` reviews these archives offline. It replays the recorded
stdout through the shared CLI parser, checks request/output/log hashes and
compares the resulting prose and generator metadata with each capture. It
also verifies the invoked model and exact source/profile prompt against the
original cohort, including source subsets used when resuming a run. The
expected source/model matrix rejects duplicate requests across run directories.
Keep the earlier completed captures when combining a resumed collection.

The text checks use the existing draft policy: 80-800 lexical words, between
half and twice the source length, no reasoning or code-fence wrapper, a changed
text hash, and the existing 20-word-span/12-word-sentence source-copy check.
Every completed capture remains in the review, including those that fail a
text check. Failed or unfinished attempts and missing matrix members are
reported separately. `--allow-partial` permits a clearly marked interim view;
the default requires the full requested matrix. These checks do not assess
meaning, tonal fidelity or readability, and do not admit hosted outputs into
the commercially released training corpus.

For the current Studio collection, run the auditor with all three capture roots,
the original 72-record source file, and the four expected requested-model
identifiers. Keep its output under ignored `data/` or the remote run directory.
The compiler optimization change after the first three GPT-6 captures does
not redraw those requests; the resumed source subset excludes those families
for GPT-6 alone. Other models still receive all 24 original source/profile
combinations.

The third capture directory stopped during Claude version discovery, before
issuing any Claude generation request. The fourth directory continues those
unissued requests and retains all 48 completed GPT captures from the earlier
directories. Future collectors save version stdout, stderr and exit status
before requiring a successful version check; a preflight failure remains
distinct from a failed model request.

```sh
audit_cli_captures --sources source-records.jsonl \
  --capture-root matched-cohort-v2 --capture-root matched-cohort-v3 \
  --capture-root matched-cohort-v4 \
  --expected-model gpt-6-astra --expected-model gpt-5.6-sol \
  --expected-model opus --expected-model sonnet --output-dir capture-review-v1
```

The current [OpenAI individual terms](https://openai.com/policies/row-terms-of-use/)
and [Anthropic consumer terms](https://www.anthropic.com/legal/consumer-terms)
assign their interests in outputs to the user, subject to their terms, and
include restrictions on developing competing models or services. Whether
those restrictions cover this released detector remains unresolved here.
Keep these hosted captures outside commercial model-release training until
that admission review is complete. Source-text licenses and ownership of
generated output do not by themselves resolve that separate use restriction.

## Completed matched collection

The [capture report](results/frontier-captures-v1.json) records all 96 outputs:
24 each from GPT-6, GPT-5.6, Claude Opus and Claude Sonnet. Every requested
source/model pair completed once, and all 96 outputs passed the archive and
mechanical text checks. No generation request was repeated during the resumed
collection. The raw text totals 129,060 bytes, with 83-326 lexical words per
passage. Each source collection contributes 24 captures; each prompt profile
contributes 16. All 24 source families remain Train.

Claude's prose messages identify `claude-opus-5` and `claude-sonnet-5`.
Helper models present only in usage accounting are retained as usage metadata,
not recorded as the prose author. The Codex streams did not expose a reported
model snapshot; their requested identifiers remain `gpt-6-astra` and
`gpt-5.6-sol`. The full review is bound by SHA256
`43ed39fa43ad7f1fa30c799656dac4a0515d26f26deb6c3444a1ea618aa09a95`.
This establishes collection and mechanical validation, not fidelity, commercial
training admission or a detector result.

## V5 diagnostic on these captures

The released v5 detector scored all 96 texts on the Studio using its bundled
CPU runner, original calibration and frozen thresholds. It detected 72/96 at
the stricter threshold and 76/96 at the looser one. All inputs were supported;
none was truncated, edited or dropped. The
[complete aggregate](results/frontier-v5-train-diagnostic-v1.json) includes
model, profile and collection counts and the exact input/output bindings.

| Requested generator | Stricter threshold | Looser threshold |
| --- | ---: | ---: |
| GPT-6 (`gpt-6-astra`) | 23/24 | 23/24 |
| GPT-5.6 (`gpt-5.6-sol`) | 21/24 | 21/24 |
| Claude Opus | 11/24 | 13/24 |
| Claude Sonnet | 17/24 | 19/24 |

The thresholds remain 0.9052925262799454 and 0.8884499969916239, applied with
strict greater-than comparisons to the sum of model-only and mixed class
probabilities. Their original nominal Calibration false-positive targets were
1% and 5%; this all-model-written cohort supplies no false-positive estimate.

All 24 source roots occur in v5 Train, although none of the 96 exact generated
texts does. This is a diagnostic of new generator outputs on familiar source
families, not a fresh benchmark. The generation harnesses also differ across
providers. Opus supplied the most misses in this matched sample, but these
counts do not establish a general ranking of generator detectability.

At the stricter threshold, v5 detected all 16 Anti-AI drafts but only 10/16
Fix Slop drafts and 10/16 plain drafts. The encyclopedia collection supplied
11 of the 24 misses. Different profiles use different source families, so these
observations do not isolate a causal effect of the style instructions. Retain
the full collection rather than selecting only explicit evasion prompts for
later work. There are no Pangram observations for these 96 captures yet, and
this diagnostic changed neither v6 fitting nor its selection or Test protocol.

The capture-audit revision in `c881faf` passed the complete Rust test suite,
format checks and all-targets Clippy on the Studio at 2026-09-12T22:51:59Z.
