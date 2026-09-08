# Fixed editing process: first development results

The fixed clarity-and-preservation prompt produced **no new detector passes** on
four development inputs. Two inputs already scored 0% and stayed there. The two
flagged inputs worsened. Every baseline and candidate was measured three times,
and each exact input returned the same combined fraction on all three repeats.

| Development source | Input model corpus | Baseline AI + assisted | Revision AI + assisted |
| --- | --- | ---: | ---: |
| Forest study, DOI ending 0186226 | Codex | 17.30% | 65.05% |
| Forest study, DOI ending 0186226 | Claude Opus | 70.84% | 79.57% |
| Cardiac study, DOI ending 0188551 | Codex | 0% | 0% |
| Cardiac study, DOI ending 0188551 | Claude Opus | 0% | 0% |

This is four model inputs from **two source families**, not four independent
studies. Candidate detector success was 2/4, baseline success was also 2/4, and
improvement from failing to passing was 0/2. The three confirmation scans for one
input are repeated observations of that input, not extra source samples.

## What was frozen

The [protocol](../experiments/edit-process-v1/PROTOCOL.md) and
[editing instruction](../experiments/edit-process-v1/instructions.txt) were fixed
before development generation. The instruction asks for an internal claim
inventory, simpler clause structure when useful, preserved qualifiers and tone,
and a final comparison against the source. It specifies no target length and
no list of detector-favoured synonyms.

Each input received one Codex CLI invocation, requested `gpt-6-astra` at low
reasoning effort. The CLI does not expose the resolved model in its event stream;
the saved attribution remains `requested:gpt-6-astra`. The collector captured all
four successful invocations and their usage. The generator received only the
immediate model text, with no human original or detector feedback. No manual
changes were made to these outputs.

The Rust `edit-prompts` command strips earlier prompt metadata from the generation
context, uses separate IDs for different model versions of the same source, and
freezes source/instruction hashes. The development split is now used; the test
split remains untouched.

## Preservation and readability

An independent assistant compared all four revisions with their immediate model
inputs without viewing detector results or original human abstracts. The
[claim-level review](../experiments/edit-process-v1/quality-review.json) found no
material loss of entities, numbers, methods, results, qualifications, logical
scope, or tone. Readability changes were judged modest; not every changed phrase
was clearly better.

These are assistant judgments. No human audit has been performed, so the joint
human-quality-and-detector gate remains false for all four. Preservation here is
relative to the supplied model input; the process cannot establish the truth of
an inherited claim or its fidelity to an unavailable earlier source.

## What we learned

The successful manual training revision did not transfer through this fixed
prompt to the flagged development inputs. Clearer, faithful prose can score
worse. We should not deploy this prompt as an adversarial miner or describe its
2/4 candidate pass rate as an improvement.

The [grammar measurements](../experiments/edit-process-v1/grammar-deltas.json)
also show that the fixed prompt did not consistently produce the changes seen
in the manual training edits. Relative-clause counts decreased in one pair,
increased in two, and stayed unchanged in one. Pronoun-token rates fell in only
one pair. This failure does not isolate whether different grammar changes would
have worked; it shows that the tested instruction did not deliver the target.

The controlled training screen still provides two useful observations: a complete
revision repeatedly moved 100% to 0%, and one small grammatical change repeatedly
moved 72.31% to 36.61% through changed labels on later, untouched sentences. The
[training report](controlled-edits.md) preserves every other candidate, including
failures and regressions.

A next process should compare multiple preservation-checked candidates, record
query cost, and retain the source as a control when rewriting worsens the score.
That is a search procedure to test, not a demonstrated solution. Freeze its
candidate-generation and selection policy before using fresh test families;
revisiting these development sources would be further tuning.

## Reproduce the process

```sh
target/release/unslop edit-prompts data/matched-pilot-v1/codex.jsonl \
  data/matched-pilot-v1/claude-opus.jsonl \
  --instructions experiments/edit-process-v1/instructions.txt --split dev \
  --out data/edit-process-v1/prompts.jsonl
target/release/unslop collect data/edit-process-v1/prompts.jsonl \
  --provider codex-cli --model gpt-6-astra --corpus fixed-edit-process-v1 \
  --out data/edit-process-v1/revisions.jsonl --max-requests 4
```

Model collection consumes model usage. Exact local outputs are cached and cannot
be overwritten by a different prompt. New uncached generation need not reproduce
the same text. Each frozen source/candidate pair then uses the Rust `study`
command with `--repeats 3 --max-requests 6 --full-document`.

The [evaluation plan](../experiments/edit-process-v1/evaluation-plan.json) binds all
four source/candidate hashes. The [detector report](../experiments/edit-process-v1/detector-report.json)
retains all 24 task IDs and all repeated fractions. Exact source files,
generation streams, and complete detector responses remain under
`data/edit-process-v1/`. The four generated revisions and their grammar features
are stored in SQLite with their development split and generator attribution;
all 24 observations are attached to the correct original or revised documents.
These additions leave the training word profiles unchanged.
