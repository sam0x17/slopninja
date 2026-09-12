# Project conventions

The project is named **Slop Ninja**, with **slop.ninja** as the planned domain.
Use **Slop Ninja** in project-facing prose and `slop_ninja` in filenames and
command names. The local repository directory is `slop_ninja/`; the GitHub
repository slug remains `slopninja`. Existing versioned `slopninja` schemas and
cryptographic domain identifiers retain their spelling for compatibility.
Existing `unslop` command names, package names and versioned
artifact schemas remain compatibility identifiers. Historical experiment
records retain their original paths and hashes.

Use Rust for the CLI, corpus collection, SQLite storage, statistics, experiments,
and detector orchestration. Use Python only where an NLP/ML dependency requires
it. Current exceptions are the spaCy annotation bridges and PyTorch author
scoring experiments in `grammar/crates/grammar-eval/python/`. Keep
corpus handling, feature extraction and vector geometry in Rust.

Run heavy compute, including model generation, training and evaluation, on
`sam@matthews-mac-studio.local`. Keep local work to editing, inspection and
lightweight coordination. Use dedicated project processes on the Studio. Do not
reboot it, restart LM Studio or interrupt unrelated workloads. Preserve caches
and record changes of execution host when resuming an experiment.

Keep exact corpus text, provenance, input hashes, source-group splits, and every
detector observation. Treat source-conditioned rewrites separately from writing
generated from scratch. A low detector score does not establish fidelity,
readability, human authorship, or performance on unseen sources.

Keep generated corpora, credentials, local databases, and live invocation logs in
ignored `data/`. Check in pinned source manifests, reproducible Rust code, and
reports. `legacy/` and `experiments/pangram-preface/` preserve earlier work; do not
extend their Python orchestration for new experiments.

Private correspondence and coursework, extracted passages, identifying source
manifests, fitted profiles, and derived candidate texts stay in ignored
`data/grammar-private/`. Do not commit them or use their sentences in public
fixtures or documentation. Public examples must be synthetic or separately
authorized. Preserve provenance and group excerpts from one work together.

Reference writing from the blogging era is acceptable; there is no hard 1999
cutoff. Preserve original composition dates when available and keep source groups
separate across fitting and evaluation. Date-based exports still need usable
dates for their declared chronological split. Keep corpus discovery notes and
downloaded samples in ignored `data/author-corpora/`.

The fresh composable base lives in the independent `grammar/` Cargo workspace.
Keep parsing adapters separate from Rust syntax, word counts, feature extraction,
geometry and rewrite rules. Version changes to feature meanings, denominators,
rules and geometry. Compare all candidate points in the same frozen space; a
distance improvement never certifies meaning preservation or detector success.

Run `cargo fmt --check`, `cargo test`, and `cargo clippy --all-targets -- -D warnings`
for Rust changes. Run the installed spaCy bridge integration when changing NLP.
Do not issue live model or detector calls as part of automated tests.
Run these checks in `grammar/` for changes to that workspace.
