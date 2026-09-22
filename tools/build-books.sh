#!/usr/bin/env bash
# Build the EPUB and PDF from mdBook's expanded Markdown.
#
# Ported from build.books.sh in Learn Go with Tests by Chris James (MIT).
# Differences: the chapter list is read from book/src/SUMMARY.md instead of
# being typed twice, drafts and the two repo-facing Meta pages are skipped,
# and one pandoc image serves both formats.
#
# usage: tools/build-books.sh [version-label]
# Needs docker or podman. Writes dist/learn-rust-with-tests.{epub,pdf}.

set -euo pipefail

version="${1:-${GITHUB_REF_NAME:-development build}}"
image="${PANDOC_IMAGE:-pandoc/extra:latest}"
runner="${CONTAINER_RUNNER:-$(command -v docker || command -v podman)}"
markdown="book/book/markdown"
out="dist"

mdbook build
mkdir -p "$out"

# Chapter order from SUMMARY.md: linked entries only, in order, README.md is
# index.md in the rendered output, and pages meant for the repo rather than
# the reader are left out of the ebooks.
mapfile -t chapters < <(
  grep -oE '\]\([^)]+\.md\)' book/src/SUMMARY.md \
    | sed -E 's/^\]\(//; s/\)$//; s/^README\.md$/index.md/' \
    | grep -vE '^(contributing|template)\.md$'
)
printf 'chapters in the ebook: %s\n' "${#chapters[@]}"

sed "s|%%FOOTER_VERSION%%|${version}|" tools/ebook/meta.tmpl.tex > "$out/meta.tex"
sed "s|%%VERSION%%|${version}|" tools/ebook/cover.tmpl.tex > "$out/cover.tex"

# The container sees the repo at /data. Paths below are relative to that.
run_pandoc() {
  "$runner" run --rm -v "$PWD:/data" -w /data "$image" "$@"
}

run_pandoc --from=gfm+rebase_relative_paths --to=pdf --file-scope \
  --pdf-engine=xelatex -H "$out/meta.tex" -B "$out/cover.tex" \
  --variable urlcolor=blue --variable geometry:margin=1in \
  --toc --toc-depth=1 --lua-filter=tools/fix-internal-links.lua \
  --metadata title="Learn Rust with Tests" \
  -o "$out/learn-rust-with-tests.pdf" \
  "${chapters[@]/#/$markdown/}"

run_pandoc --from=gfm+rebase_relative_paths --to=epub --file-scope \
  --toc --toc-depth=1 --lua-filter=tools/fix-internal-links.lua \
  --metadata-file=tools/ebook/metadata.yaml \
  --metadata date="${version}" \
  -o "$out/learn-rust-with-tests.epub" \
  "${chapters[@]/#/$markdown/}"

ls -l "$out"/learn-rust-with-tests.*
