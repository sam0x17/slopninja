# Hosted generator corpus, snapshot 1

This snapshot contains **96 model-written passages from 24 source families**:
24 each from GPT-6 Astra, GPT-5.6 Sol, Claude Opus and Claude Sonnet. The data
files total **1,994,501 bytes (1.99 MB)**, including complete prompts, generation
metadata and 24 supporting human source records. The generated prose itself is
129,060 bytes and 19,562 whitespace-counted words. Slop Ninja Research published
this research snapshot on 2026-09-12.

| File | Contents |
| --- | --- |
| [reviews.jsonl](reviews.jsonl) | All 96 exact output texts, model provenance, source attribution and mechanical review results |
| [requests.jsonl](requests.jsonl) | Complete prompts and requested settings, joined by output and request SHA256 |
| [source-records.jsonl](source-records.jsonl) | The 24 referenced human source records with their original rights and origin evidence |
| [capture-audit.json](capture-audit.json) | The original capture audit, unchanged |
| [manifest.json](manifest.json) | File sizes, hashes, coverage and publication checks |

The supporting human roots are already in the v5 Train corpus. They are not
24 new annotations. All generated passages use those Train families, and none
shares an exact text hash or source family with the 375 previously annotated
Train texts. The combined local annotation collection therefore contains
**471 distinct Train texts from 133 families**. Another 57 Development texts
have earlier annotations. Repeated scans do not count as additional texts.

Each generator received the same 24 sources, covering PLOS abstracts, Wikinews,
historically matched HC3 Wikipedia answers and CMU Wikipedia book summaries.
Six prompt profiles cover source-matched, plain, informal, direct, anti-AI and
Fix Slop writing. There are 16 captures per profile across the four generators.
These are source-conditioned drafts composed by a model. A Pangram result of
human or AI-assisted does not change that recorded workflow.
The supporting human labels rely on historical source evidence and do not
certify a documented human-only writing process.

The original audit's `provider_output_rights_reviewed: false` and
`commercial_training_admitted: false` fields describe its state when captured.
They remain unchanged. Publication does not add these rows to an admitted
commercial training corpus or alter the frozen v6 fitting experiment.

The requested GPT selectors are `gpt-6-astra` and `gpt-5.6-sol`; the CLI did not
expose immutable model snapshots. Claude requests used `opus` and `sonnet`, with
prose responses reporting `claude-opus-5` and `claude-sonnet-5`. Auxiliary usage
metadata can mention another model; that does not identify the prose generator.
Unexposed temperature, seed and output-limit settings remain null. Keep the
requested writing profile separate from the generator label.

`requests.jsonl` retains both parsed request fields and `archived_request_utf8`.
Hash the UTF-8 bytes of the latter to verify the original `request_sha256`, then
parse it to check equality with `request`. `reviews.jsonl` is a byte-for-byte
copy of the reviewed archive. The source file selects the 24 referenced human
roots from the original 72-row seed file; the original seed hash is retained.
Historical local paths identify acquisition and generation archives, which are
not required to read the snapshot.

Preserve the source attribution, URLs, license and ShareAlike obligations in
every record. These include CC BY 2.5, CC BY 4.0, CC BY-SA 3.0 and CC BY-SA 4.0,
plus the CMU collection's CC BY-SA 3.0 US notice where recorded. The embedded
Fix Slop prompt adaptation retains its upstream attribution and
[MIT notice](LICENSE.fix-slop). To the extent Slop Ninja Research holds rights
in an output's additions, those additions are offered under the corresponding
source license. This does not replace third-party rights or account terms.

Both [OpenAI's output-ownership provision](https://openai.com/policies/row-terms-of-use/#content)
and [Anthropic's output assignment](https://www.anthropic.com/legal/consumer-terms)
support publishing user outputs, subject to their terms and third-party rights.
This is the basis for this publication, checked on 2026-09-12. It does not
establish unrestricted rights to train competing models; commercial training
admission remains unresolved.

All 96 passages received Pangram 4 annotations. Individual provider scores,
reports and invocation logs remain in ignored storage under the existing
[annotation publication policy](../../baseline/detector/STANDARD_CORPUS.md).
See the [Pangram and v5 aggregate comparison](../../baseline/detector/results/pangram-frontier-v1.json).
The numeric fractions are detector observations, not ground-truth origin labels.

The Mac Studio publication check verified all output and original request
hashes, source joins, source redistribution flags, the complete model/profile
matrix and disjointness from the earlier Train annotation collection. These
checks do not certify factual fidelity, readability or fresh-test performance.
Keep each source family together when designing future evaluations.

To verify the snapshot files from this directory:

```sh
shasum -a 256 -c SHA256SUMS
```
