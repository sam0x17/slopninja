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
