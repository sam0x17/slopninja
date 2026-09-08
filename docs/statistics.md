# What the coordinates mean

For word `w` in a corpus, store occurrence count `c(w)` and total lexical words `N`. The basic coordinate is `c(w)/N`; reports also show occurrences per 1,000 opportunities. Corpus size is therefore explicit, and adding documents adds both their word counts and their denominators.

The lexical tokenizer normalizes Unicode to NFC, normalizes apostrophes, lowercases, and retains internal contractions. Numbers, hyphens, and underscores separate lexical words. Raw text is retained unchanged. This convention differs from Pangram's whitespace word count; neither count is substituted for the other.

Word ngrams do not cross heuristic sentence boundaries. Grammar features use spaCy's sentence boundaries and parse. Every family has its own denominator: lexical tokens for words, ngram opportunities for ngrams, parsed tokens for POS/dependency counts, and parsed sentences for constructions. A sentence can contain several construction types; the sum of their counts may exceed the sentence denominator.

## Anomaly ranking

For a feature observed `a` times in `n` opportunities on the left and `b` times in `m` on the right, the implementation uses a half-count smoothing convention:

```text
smoothed rate ratio = ((a + 0.5)/(n + 1)) / ((b + 0.5)/(m + 1))
log odds = log((a + 0.5)/(n - a + 0.5))
         - log((b + 0.5)/(m - b + 0.5))
variance approximation = 1/(a + 0.5) + 1/(n - a + 0.5)
                       + 1/(b + 0.5) + 1/(m - b + 0.5)
z heuristic = log odds / sqrt(variance approximation)
```

Each coordinate is treated as a binary event per opportunity, so this also works for overlapping sentence constructions. These individually smoothed rates do not form a normalized multinomial vector. The exported sparse vector uses unsmoothed rates and keeps feature families separate.

The four-cell approximation ignores correlations among words, sentences, authors, and source groups. It is an exploratory ranking, **not a calibrated significance test**. The code does not implement a learned model or Pangram's embeddings. It does not present these ranks as probabilities of authorship or causal effects on detector scores.

By default, a feature needs five pooled occurrences and appearances in at least three documents and three source groups on one side to meet the displayed evidence floor. This floor is a screening rule, not statistical confidence. Rare features remain in the output with their evidence flag false; high ratios based on tiny counts should not drive editing rules. Repeated variants of one preface cannot satisfy the source-group floor.

Lexical selection with frequency uncertainty has a long history; the [Monroe, Colaresi, and Quinn paper](https://www.prio.org/publications/4012) is a useful starting point for richer corpus priors. The current formula above is deliberately documented directly rather than presented as an implementation of their informative-Dirichlet method.

## Extending this into a predictor

Build balanced model and human samples within topic/register strata. Fit scaling and feature selection only on training sources. Compare lexical, grammar, and combined predictors against a baseline that uses only length and topic. Validate across unseen prompt families and entire held-out domains. Bootstrap source groups for uncertainty; separate feature discovery from confirmation.

For editing, predict the *change* in repeated full-context detector scores from a candidate's feature changes. A classifier of original model prose need not identify edits that lower detector scores. Use paired candidates, unchanged controls, and a fixed candidate/query budget. A later ranking model should optimize the likelihood of passing all quality and detector gates, rather than the lowest detector observation alone.

Readable prose has no required destination in this feature space. Dense technical argument may need unusual nouns and complex clauses. Keep an unusual construction whenever changing it weakens the argument, removes information, or changes the intended voice.
