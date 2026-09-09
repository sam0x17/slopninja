# Function-word and predicate-operator support

Two new Rust feature families have enough recurring observations for further
experiments. This audit used the existing three training panels: 3,889 posts
from 300 authors. It did not fit a model or read validation/test annotations.
The working 3,557-coordinate model still uses its original fourteen families.

The [function-word role family](function-word-roles.md) couples a normalized
lemma with its syntactic role, governing head's part of speech and relative
position. The [predicate-operator family](predicate-operators.md) records local
auxiliary/negation order and attachment, together with observed finite-carrier
evidence. The main predicate's lemma is excluded from the latter key, except
when an auxiliary is itself the clause head.

## Coverage

A supported type occurs in at least five distinct posts, three distinct authors
and three distinct author/date groups. Within-post repetitions contribute to
occurrence mass but count once toward post support. These thresholds were fixed
before extraction. The table pools all three training panels.

| Family | Observations | Distinct types | Supported types | Occurrences in supported types |
| --- | ---: | ---: | ---: | ---: |
| Function-word role | 689,053 | 6,345 | 1,932 | 98.9835% |
| Predicate/operator sequence | 248,567 | 3,417 | 1,008 | 98.5223% |

Neither family was unavailable in any post. No eligible function-word or
operator event had a missing lemma. Both extractors nevertheless preserve an
explicit missing-lemma event and a zero-opportunity unavailable state, with
synthetic tests for those cases.

Rare types remain common: 46.75% of function-role types and 49.55% of operator
types appeared in only one post. Their occurrence mass is small. Using only the
original panel's vocabulary, 0.84% and 0.86% of function-role occurrences in the
other two panels were unseen. The corresponding operator fractions were 1.21%
and 1.16%. All three panels are known training data; these are recurrence
descriptions, not a fresh generalization result.

Of 248,567 predicate-head observations, 78,540 had no finite carrier, 169,328
had one and 699 had multiple carriers. The extractor includes fragments and
nonfinite clauses, so absence is not a parser-error label. Positive morphology
and tag evidence differed for 4,505 carriers. This includes absent morphology
evidence and does not establish contradictory annotations or semantic scope.

## Validation and reproduction

The audit binds each training manifest, query table and annotation/cache file
by SHA256. It checks source text, parser identity, split membership and
disjoint post/text/author identities across panels. Re-extracting every post
produced exactly the same canonical bytes for the complete original fourteen
families. Every new predicate denominator equaled the old clause-head count.
No parser, LLM or detector calls were needed for the corpus audit.

An independent Rust audit reproduced every support and unseen-mass statistic,
all 7,778 event-trace count reconstructions and all 7,778 cache/annotation file
hashes. It also re-extracted the original fourteen families from one post per
author, covering all 300 authors. The workspace passed formatting, Clippy and
243 test invocations, including 11 installed-parser invocations. Shared module
tests compiled into different binaries are included in that count.

The protocol and full outputs are local, ignored artifacts beneath
`data/author-corpora/blog-authorship-2004/grammar-choices-v1/`. Source-aligned
events, post identifiers and cache provenance remain there. Public examples
are synthetic.

From the repository root, after building the new binary in `grammar/`:

```sh
grammar/target/release/slopninja-grammar-choices \
  --protocol data/author-corpora/blog-authorship-2004/grammar-choices-v1/protocol.json \
  --expected-protocol-sha256 b73a0db45387bd1155be4a66e4bef649c073995eb56e0d8b38a3eff55d0f253d \
  --repo . \
  --corpus data/author-corpora/blog-authorship-2004 \
  --out data/author-corpora/blog-authorship-2004/grammar-choices-v1/audit-replay-v1
```

Use a new output directory. The recorded report SHA256 is
`8a3c5360a29e60e708c2b4216ae11cd22d5bd492a1c97b865593c96b0c23286f`.
The receipt also binds the executable, source files and per-post output.

## What to test next

The earlier [clause-family study](clause-family-study.md) showed that better
recurrence alone did not improve author recognition. Before spending another
fresh author cohort, compare these joint features with their simpler marginals
on training data. For function roles, preserve lemma/POS and role frequencies
while varying their association. For operators, compare ordered, attached
sequences with operator inventories. Whole query dates must remain outside
their own author reference profiles. Any such reused-training comparison is
exploratory and needs a separate frozen protocol.

The [edit utility matrix](edit-utility-matrix.md) asks a separate question:
whether the existing rules produce measurable choices while preserving the
intended claims. Recurrence, author recognition and distance improvement each
require their own interpretation. None certifies readability, tonal fidelity
or detector performance.
