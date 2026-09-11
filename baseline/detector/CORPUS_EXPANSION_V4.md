# Corpus expansion v4

This is data preparation for the next detector experiment. No new detector has
been selected, trained or promoted. The existing v1 reference and v2 candidate
remain immutable. No further Pangram calls are planned in this stage.

## Source families and generation

Start with the 261 rights-admitted HC3 historical human excerpts, with corpus
SHA256 `178b2c9ab55a74107fb6792b72a5d69e369a9bd103b17fa3343caeaac989c23d`.
Keep the frozen source-family splits. Select one representative from each of the
243 families by ascending SHA256 of record ID, independent of text scores. Keep
all other admitted excerpts in the source archive and record this selection.
The selected corpus hash is
`4481783ee88a43df8369988f811a854b67b690a73212297588a2c8502fb9afb2`:
184 Train, 21 Development, 21 Calibration and 17 Test families.

Produce one independently composed, source-conditioned draft and one light to
moderate copyedit per representative. Keep the encyclopedia register for these
sources; the old news/scientific register choices remain unchanged. Preserve
source meaning, qualifications, intended tone and attribution in the prompts.
These instructions do not establish factual fidelity. Record model drafts and
edits of historical proxies under the existing weak-label policy.

Use the existing `style-mix-v1` assignment for six profiles: source-matched,
plain, informal, direct, anti-ai and fix-slop. Assign by frozen split, register and
source-family hash before generation. Use the same profile for siblings. Do not
select profiles, texts or retries by detector score. Retain every response and
failure; the existing whole-family admission rule requires both outputs to meet
the declared completion, length, formatting and source-copy checks. Report the
attempted denominator and exclusions. An incomplete run is not a training corpus.

OLMo 3 7B Instruct adds a third generator family to the existing Qwen and Mistral
work. The MLX conversion is pinned to
`mlx-community/Olmo-3-7B-Instruct-4bit@d732c91ae02e90cd5d86e810fbeb9741794b4dd9`.
The upstream [model card](https://huggingface.co/allenai/Olmo-3-7B-Instruct/blob/6e5971d9eba42665f5bd5a0fcf047f299ce1dccc/README.md)
declares Apache 2.0. Each runtime file was verified against the pinned Hub tree,
including the 4.11 GB weight file's SHA256. The local server uses the existing
MLX environment, disables prompt caching and serves one request at a time.
Generation uses temperature 0.7 and a 1,200-token completion cap, with complete
requests, model settings and raw responses retained locally.

The first run requests 486 outputs, two per selected family. Its immutable local
plan and response archive are under
`data/baseline-detector/corpus-expansion-v4/olmo-hc3/`. A finished summary is
required before training. The current 80-word output floor can exclude otherwise
usable edits of short sources; retain these failures when assessing corpus yield.

The compatibility probe completed one synthetic 100-word passage, 118 generated
tokens, in 1.38 seconds including prompt processing. This verifies that the
model works through our local HTTP path; it does not establish a workload-wide
throughput estimate or writing quality. See the
[probe report](results/olmo-generator-probe-v1.json).

## Next detector fit

Finish the generation and inspect Train admission failures and sample fidelity.
Add paired Qwen/Mistral examples on these new families, and OLMo examples in the
existing Train registers, before attributing improvements to generator diversity.
Otherwise generator family and register would be confounded. Review the staged
CMU book summaries to add narrative prose. Personal writing and documented human
editing sessions remain missing registers/workflows.

Freeze the combined corpus and candidate recipes before training. Select using
Development, calibrate on Calibration and open fresh Test predictions only after
selection. Source-history checks and generation are allowed before that point;
detector results on Test are not. Use existing Pangram observations for the old
Development comparison only. Keep provider predictions separate from origin
labels, and do not use them as training targets without a separate declared and
rights-reviewed experiment.

The new lane follows the [ShareAlike release policy](SHARE_ALIKE_POLICY.md).
Ship source attribution with any resulting public detector. The model's encoder
architecture and sparse word/grammar controls remain unchanged in this stage.
