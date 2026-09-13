# V6 release attribution

The two v6 learning-rate candidates use the same frozen primary corpus. Source
credits can therefore be prepared before selection finishes. This step does not
choose a model, evaluate Test or qualify a release.

Run the Rust extractor on the Studio from `baseline/detector/`. The output's
parent directory must exist, and the output file must be new:

```sh
cargo run --example v6_source_credits -- \
  --run-root /Users/sam/slop_ninja_runs/revision_expansion_v6/detector_v6_v1 \
  --protocol-sha256 3a90c59834fb733b377a709da0c26887b40fbfe8fdf99d5542077e9ec777f79b \
  --output /path/to/new/source-credits.json
```

The expected protocol hash is recorded in the frozen
[evaluation command plan](manifests/v6-postfit-commands-v1.json). The extractor
checks the protocol, rights manifest and all three fitting shard hashes. It
includes Train, Development and Calibration records with complete ancestry;
the selected artifact's training metadata identifies the actual fitting and
selection views. It never opens the Test shard.

Each credit retains the record identity, source URLs, attribution, license,
change notices, generation metadata and parent ID. The extractor omits text,
raw prompts and local acquisition paths. It checks declared training and model
release permissions, ShareAlike notices, record uniqueness, family separation
and parent presence. These checks verify consistency with the admitted corpus;
they do not establish rights independently of its source evidence.

The output records the input, compiled helper source and executable hashes.
Preserve it with the extraction log and exact helper when assembling the release.
The [model distribution policy](SHARE_ALIKE_POLICY.md) also requires the weight
license and upstream checkpoint notices. Bind the completed attribution file to
the selected artifact and its final evaluation report before publishing the
research bundle. Until that review finishes, v5 remains the released reference.

The first extraction completed on the Studio on 2026-09-13. It contains 4,903
records from 685 source families and checks 3,618 ShareAlike notices. The saved
`source-credits.json` has SHA256
`fb1c81a6b655c696bc18da175cc9f1dd04ab90912409d8d849b1baad30e9a493`.
Formatting, the detector crate's tests, all-targets Clippy and the example build
passed before extraction. The full attribution file remains in release staging
until the model and its evaluation are ready.
