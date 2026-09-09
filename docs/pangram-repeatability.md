# Pangram repeatability probe

On 2026-09-09, three fresh Pangram requests for one identical synthetic text
returned equal document class fractions and different auxiliary scores. Two
later GETs of the first task returned its original result unchanged. Full
numeric repeatability across fresh requests failed in this sample.

## Observations

All three POSTs used one API account, selector `pangram-4`, and the same exact
input hash. All returned version `4.0` and distinct task IDs. Submission times
ran from `18:26:40.197185Z` to `18:27:11.370454Z`, about 31 seconds.

| Result field | Fresh request 1 | Fresh request 2 | Fresh request 3 |
| --- | ---: | ---: | ---: |
| `fraction_ai` | 1 | 1 | 1 |
| `fraction_ai_assisted` | 0 | 0 | 0 |
| `fraction_human` | 0 | 0 | 0 |
| `windows[0].ai_assistance_score` | 0.9999844431877136 | 0.9999838471412659 | 0.9999838471412659 |
| `windows[0].humanizer_score` | 0.0033263072837144136 | 0.003090519458055496 | 0.003090519458055496 |

The absolute differences were `5.960464477539062e-7` for the assistance score
and `0.00023578782565891743` for the humanizer score. Requests 2 and 3 had equal
complete results as parsed JSON. Each GET recheck matched request 1's complete
result as parsed JSON; the two saved GET bodies also matched byte for byte.
Polling an existing task therefore supplied no additional fresh inference.

Local records remain in ignored `data/pangram-proof-v1/`: `repeatability-01.json`
through `repeatability-03.json`, `recheck-01.json`, `recheck-02.json`, and
`repeatability.txt`. These are local observations without provider signatures
or a verified TLS transcript. Their hashes establish local identity only.

This probe does not establish cross-account agreement, immutable model weights
behind version `4.0`, or stability over longer periods. All document fractions
were at an endpoint, far from the strict `0.10` reward threshold. Their equality
here cannot establish threshold stability. No numerical audit tolerance is
derived from these three requests. The observed variation alone gives no basis
for accusing an evaluator of fabrication.

## Reproduce the fresh-request comparison

The assistant created this nonprivate input. Save the following single line as
`data/pangram-repeatability-local/input.txt`, with exactly one trailing LF byte
and no extra blank line. It has 126 whitespace-delimited words and 700 UTF-8
bytes. SHA-256:

```text
41d4b1a5815c017e78c0edd4d84d01a450e1c60dc22664a483cc9bfb142563a5
```

```text
The workshop opens at nine on Saturday. Please leave the side gate clear because the delivery driver will need room to unload two benches. We have enough paint for the chairs, but the cabinet needs another coat of primer before anyone starts on its doors. I put the spare brushes in a blue bucket under the sink. If the weather stays dry, we can move the sanding outside after lunch. Otherwise, we should finish the small repairs first and leave the dusty work until Sunday. Nobody needs to bring tools. Bring an old shirt, though, and tell me by Thursday if you want a sandwich. We will lock up at four, even if a few jobs remain unfinished. The building caretaker has another booking that evening.
```

With `PANGRAM_API_KEY` set in the environment, these existing CLI commands make
up to three paid requests. Use a new output directory; an existing successful
score file is reused locally and does not even issue a GET recheck.

```sh
shasum -a 256 data/pangram-repeatability-local/input.txt
for attempt in 01 02 03; do
  cargo run --release -- score data/pangram-repeatability-local/input.txt \
    --model pangram-4 --max-requests 1 \
    --out "data/pangram-repeatability-local/result-$attempt.json" || break
done
```

Keep uncertain submissions and resume known task IDs according to the
[evaluation protocol](evaluation.md); do not delete a record to force a retry.
The provider returned this text without its trailing LF. Compare the exact
submitted input separately from returned text and window offsets.

At the [oracle design's planning rate](pangram-oracle.md#who-pays), the three
fresh calls estimate to `3 * ceil(126 / 100) * $0.05 = $0.30`. Actual billing was
not verified. The [spot-check proposal](pangram-oracle.md#reproducibility-spot-checks)
keeps fresh-request agreement separate from authenticated evidence of the
original response.
