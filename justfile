# Recipes mirror what CI runs. `just check` is the whole set.

default:
    @just --list

fmt:
    cargo fmt --all --check

lint:
    cargo clippy --workspace --all-targets -- -D warnings

test:
    cargo nextest run --workspace

# nextest cannot run doctests on stable, so they get their own step.
doctest:
    cargo test --doc --workspace

deny:
    cargo deny check

typos:
    typos

# Book prose plus the root pages the book includes or links to.
prose:
    tools/prose-check book/src README.md CONTRIBUTING.md STYLE.md

book:
    mdbook build

links: book
    lychee --config lychee.toml "book/book/html/**/*.html"

serve:
    mdbook serve --open

# Everything CI runs except the link check, which needs the network.
check: fmt lint test doctest deny typos prose book

# EPUB and PDF via pandoc in a container. Needs docker or podman.
books:
    tools/build-books.sh
