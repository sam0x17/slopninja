# Project conventions

Use Rust for the CLI, corpus collection, SQLite storage, statistics, experiments,
and detector orchestration. Use Python only where an NLP/ML dependency requires
it. The current exception is the spaCy bridge in `python/`.

Keep exact corpus text, provenance, input hashes, source-group splits, and every
detector observation. Treat source-conditioned rewrites separately from writing
generated from scratch. A low detector score does not establish fidelity,
readability, human authorship, or performance on unseen sources.

Keep generated corpora, credentials, local databases, and live invocation logs in
ignored `data/`. Check in pinned source manifests, reproducible Rust code, and
reports. `legacy/` and `experiments/pangram-preface/` preserve earlier work; do not
extend their Python orchestration for new experiments.

Run `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets -- -D warnings`
for Rust changes. Run the installed spaCy bridge integration when changing NLP.
Do not issue live model or detector calls as part of automated tests.
