# Whitepaper build

The authoritative source is [main.tex](main.tex), with editable LaTeX files
in [sections/](sections/), a bibliography in [references.bib](references.bib),
and a vector figure in [figures/](figures/).
Edit those LaTeX sources directly, then run from the repository root:

```sh
bash whitepaper/build.sh
```

The build uses `latexmk`, XeLaTeX, BibTeX, and `rsvg-convert`, with the TeX Gyre
Pagella/Heros and Latin Modern Mono fonts. It renders the editable SVG to a
PDF figure and compiles the native LaTeX project. No Markdown conversion is
part of the build. Temporary files and logs stay in ignored `build/`.

[slop_ninja.pdf](slop_ninja.pdf) and the PDF figure are checked in. With the
existing figure, run `latexmk -xelatex -outdir=build main.tex` from this
directory, or configure a TeX editor for XeLaTeX with BibTeX.
The source and rendered paper are design proposals, not a deployed protocol.

Draft 0.17 requires qualified attestation for B emissions, mandatory rewrites
and paid B jobs, protecting private weights and customer text through
[attested inference](sections/05-confidentiality.tex). Model owners and customers
independently verify the protected runtime before provisioning their secrets.
Ordinary hosting remains available for A with customer acceptance; ordinary B
execution is limited to research without emission credit.
[Private model version receipts](sections/03-models.tex) associate attested
benchmark outputs with that version and bounded serving profile; independent
quality review remains required. An unresolved candidate receives zero assigned
credit while competitors retain completed results and all assigned weights.
The [commercial section](sections/06-paid-inference.tex) treats B as the primary
paid product and A as an open resource with optional hosting.
[B settlement](sections/06-paid-inference.tex) now requires a TEE receipt,
complete encrypted result publication and a strict weighted delivery certificate,
with fixed timeout refunds and no customer acknowledgment. This has not yet
been implemented or qualified for paid B customer use.
