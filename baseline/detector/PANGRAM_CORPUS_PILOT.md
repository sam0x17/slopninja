# Pangram corpus rehearsal

Declared before the first bulk submission, 2026-09-11. The user authorized bulk
experimentation with a cumulative spending cap. The local budget ledger records
that cap and reserves headroom before any paid request. Uncertain submissions
retain their reservation and cannot be automatically repeated.

Submit one pass over all 57 original Development documents from style pilot v2:
19 source families with a historical human proxy, a recorded model draft and a
recorded model edit of that proxy in each family. Preserve all text, existing
splits, source labels and evidence qualifications. This is already-opened
development material, not a new confirmation sample.

Use the prepared Pangram 4 bulk request with SHA256
`cd192f9652c9bcf630350a05f6844da7000cc9e536331d16eb7ef18581b89401`.
The exact source-records file has SHA256
`2e3ef17f7c1be26477458e0a33c3b8a291703123bbbcdbc1fad472e74768d949`.
Pin selector `pangram-4`, retain the returned version, initial receipt, every
status/result response and all failures. Require exact coverage, task identity
and echoed-text equality before importing annotations. Never publish raw corpus
text or provider responses through this rehearsal.

Report Pangram's observed content fractions and native label distribution.
For the prespecified binary diagnostic, mark an input as flagged when
`fraction_ai + fraction_ai_assisted >= 0.10`. Report flagged counts separately
for the three source-origin labels, teacher families and prompt profiles, with
missing observations visible. These weak source labels qualify all apparent
errors. Content fractions are not calibrated document-origin probabilities, so
do not calculate a Pangram origin log loss or Brier score from them.

Compare descriptively with the already selected word/grammar candidate
`combined-0.001` and v2 encoder
`52f0054fc5b3c3f22d04a34fb1dcd9855e7296a0ff3ad943fc270741ccc3224e`.
Use each local artifact's frozen calibration operating points, without refitting
or selecting models using Pangram. Different operating points and 19 human
families do not establish matched-FPR superiority or a quality-anchor switch.
Three-class argmax confusion may be reported for our own models separately.

Preserve the annotations for reuse across later comparisons on these exact
texts. Do not use these Development annotations as new training labels. Any
repeats or further source batches need their own predeclared sample and must
charge the same cumulative ledger. Actual account billing remains unverified
until provider usage evidence is available; reported costs are estimates.
