# Standard corpora and reusable Pangram annotations

Decision, 2026-09-11: build a versioned corpus with independent origin evidence
and separately stored detector observations. Start with a local benchmark, then
publish the portions for which both text and annotation reuse rights are clear.
A Pangram prediction must never overwrite an origin label or establish that an
unlabeled web page was written by a human.

## Dataset priorities

| Source | Contribution | Admission work |
| --- | --- | --- |
| [RAID](https://github.com/liamdugan/raid), initially book summaries | Paired sources, multiple generators, decoding settings and adversarial variants | Reconstruct `source_id` and `adv_source_id` families. Join human summaries to [CMU Book Summaries](https://www.cs.cmu.edu/~dbamman/booksummaries.html), whose Wikipedia-derived text is CC BY-SA. Preserve attribution and the applicable version of that license. RAID's MIT release does not settle rights to every underlying source. |
| [HC3](https://github.com/Hello-SimpleAI/chatgpt-comparison-detection#dataset-copyright), initially `wiki_csai` | Wikipedia answers paired with ChatGPT answers | Its copyright table identifies this subset as CC BY-SA. Recover article/revision attribution and group all answers to the same source together. Other subsets have different or unresolved rights. |
| [FineWeb](https://huggingface.co/datasets/HuggingFaceFW/fineweb/blob/main/README.md) | Broad web registers for false-positive stress testing | ODC-By covers the database; its card also points to Common Crawl terms. Recover page-level rights and composition evidence. Crawl time alone does not prove human authorship. Do not admit the whole collection as commercially cleared human training data. |
| Existing rights-admitted PLOS/Wikinews pairs | Immediate end-to-end annotation rehearsal, including model-edited historical text | Already tracked locally. Preserve the weak historical labels and existing splits. These narrow registers cannot substitute for the broader benchmark. |

RAID and HC3 are established detection datasets, while FineWeb supplies broad
web text. Pangram's [current model card](https://www.pangram.com/research/model-card/pangram-4)
reports FineWeb evaluations; downloading a similarly named collection would not
reproduce its exact sample or labeling policy. Public benchmark performance also
cannot establish that a provider never trained on the benchmark.

The initial import work should stage pinned RAID/HC3 subsets under ignored
`data/`, produce source joins and license evidence, and report exclusions before
admission. The existing detector loader deliberately does not yet admit CC BY-SA
records. Add a specific attribution/distribution policy before enabling those
records; changing an allowlist alone would leave the work unfinished. A book's
author is not the author of its Wikipedia plot summary.

## Corpus contract

Each document retains its upstream dataset revision and row ID, exact UTF-8
text hash, source family, derivation parents, origin label, evidence strength,
rights evidence and split. Keep upstream binary labels binary when production
records cannot support our three-class distinction. Attack metadata does not by
itself establish human involvement in a mixed-origin workflow.

Store Pangram observations in a sidecar keyed by exact text hash, requested
selector, returned version, timestamp and repeat index. Keep complete raw
responses, task/bulk IDs, request hashes and errors. Preserve document fractions
and window results separately. `fraction_ai + fraction_ai_assisted` measures
flagged content; it is not our document-origin probability. Missing and failed
requests remain visible. No score-based exclusions or replacement texts.

Freeze source-family splits before annotations. Use training observations for
error mining or an explicitly declared auxiliary teacher experiment. Development
observations support selection; confirmation annotations stay sealed until the
contender and analysis are frozen. Reuse each observation across all detector
comparisons. If the provider changes, append a new observation set. Never mix
versions silently or call agreement with Pangram accuracy against true origin.

For a standard public release, ship a corpus card, source manifest, reconstruction
code, documented labels and uncertainty, fixed splits, checksums, attribution and
the permitted annotation fields. Separately maintain fresh private confirmation
families. Customer inference text and private correspondence remain excluded.

## API and cost

Pangram documents bulk jobs with a 1,000-unit limit: each text consumes one unit
per started 100-word block for Pangram 4. Results expire 48 hours after terminal
completion, so archive them promptly. Save the initial bulk receipt before
polling, reconcile every accepted or failed item, and never automatically repeat
an uncertain submission. Pin the selector explicitly and record the returned
version. [Bulk API documentation](https://docs.pangram.com/api-reference/bulk-api).

On 2026-09-11 the posted rate is $0.05 per Pangram 4 unit, with 20% off for bulk
jobs. At exactly 300 words each, 1,000 documents cost an estimated $120 for one
bulk pass; 10,000 cost $1,200. Three complete repeats triple those figures.
Per-document rounding matters: 301 words require four units. Actual provider
word counting and account billing must be checked during a small rehearsal.
[Pricing](https://www.pangram.com/pricing).

Pangram's published terms grant internal business use of the service and retain
the user's rights in submitted content. I found no explicit grant for republishing
a corpus of API annotations or using them to train a competing detector. Those
uses remain unresolved pending applicable API/account terms or a written grant;
this is not a finding that either use is categorically prohibited. Local
evaluation and preparation can proceed separately.
[Terms, sections 5, 6 and 8](https://www.pangram.com/terms-of-service).

## Implemented preparation step

`annotation_plan` reads an already frozen, admitted origin corpus. It checks
external-evaluation permission, deduplicates exact inputs while retaining every
source label, and writes Pangram 4 bulk payloads plus a costed manifest. Each
planned repeat has a distinct request ID. It makes no network calls and reads no
API key. Its archived corpus and request files contain text and belong in ignored
`data/`. Submission, receipt reconciliation and annotation import remain to be
implemented for bulk jobs; the root CLI has an existing realtime corpus scorer.

```sh
cargo run --release --manifest-path baseline/detector/Cargo.toml --bin annotation_plan -- \
  --input data/baseline-detector/style-pilot-v2/studio-results/data/primary-shards/development.jsonl \
  --output-dir data/baseline-detector/pangram-corpus-v1/development-plan
```

The prepared 57-row batch contains 15,250 whitespace-counted words and an
estimated 176 billable units: $7.04 bulk or $8.80 realtime for one pass. Its exact
request payload hash is
`cd192f9652c9bcf630350a05f6844da7000cc9e536331d16eb7ef18581b89401`.
A read-only catalog request confirmed `pangram-4` access for the available key.

This prepares the exact development corpus as an API rehearsal. It is
already used for model selection and cannot provide fresh confirmation. The
separate word/grammar probe remains free of provider calls. No API spending or
annotation publication has occurred in this preparation step.

## Local standard-dataset staging

The Rust `stage_hc3` command fetched only `wiki_csai.jsonl` and its dataset card
at revision `4d0ff18143b5a7e1b1e79beb540c04549d1e59d3`. The local capture has 842
questions, 842 human-labeled answers and 842 ChatGPT-labeled answers. Of the 1,684
answers, 62 fall below Pangram's stated 50-word scope. The JSONL hash is
`4f78105c72e7edd3b619b3b44abac1e6f8bdb42211f22e43191d4dae889e1968`.
See the [capture manifest](manifests/hc3-wiki-staging-v1.json).

The rows contain only `question`, `human_answers` and `chatgpt_answers`: there
are no upstream row IDs or article/revision URLs. This makes reconstruction of
Wikipedia source attribution the next concrete admission task. Local hashes can
identify rows but cannot supply missing authorship or license evidence. The raw
subset remains in ignored `data/baseline-detector/standard-corpora/hc3-wiki-v1/`;
none of it has entered training or been sent to Pangram.

```sh
cargo run --release --manifest-path baseline/detector/Cargo.toml --bin stage_hc3 -- \
  --output-dir data/baseline-detector/standard-corpora/hc3-wiki-v1
```

The staging command requires a new directory, bounds downloads, archives exact
bytes and records hashes. It does not execute the upstream dataset loader.
