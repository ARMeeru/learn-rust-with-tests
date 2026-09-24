# Contributing

Thanks for wanting to help. This file covers how a chapter gets written and what the checks
expect. [STYLE.md](https://github.com/ARMeeru/learn-rust-with-tests/blob/main/STYLE.md) covers how the prose should read.

## Licence

Contributions are accepted under the MIT licence in [LICENSE.md](https://github.com/ARMeeru/learn-rust-with-tests/blob/main/LICENSE.md). There is no
contributor licence agreement. By opening a pull request you agree your contribution is licensed
the same way as the rest of the book.

## How a chapter is built

1. Find the chapter's issue. Every chapter has one, linking the Go original. Comment that you are
   taking it so nobody duplicates the work.
2. Read the Go chapter end to end before writing anything.
3. Write the code first, test-driven, as step crates under `chapters/<nn>-<slug>/v1`, `v2` and
   so on. Each crate is the code exactly as the prose will show it after that step. Get
   `just check` green.
4. Write the prose around the crates. Every code block is an include from a step crate. Never
   type code into the Markdown.
5. Follow the chapter template in `book/src/template.md`. Every tutorial chapter ends with a
   "Wrapping up" section.
6. Open the pull request, link the issue, and go through the checklist below.

## Rules that are easy to miss

**Every committed crate compiles, and every committed test passes.** A red step where the test
calls something that does not exist yet cannot live in a workspace CI builds, so the
compile-error moment is told in prose, with the error quoted from the pinned toolchain. A red
step whose code compiles but fails its test keeps the real assertion behind
`#[should_panic(expected = "...")]`, so CI stays green while the crate stays honestly broken, and
the prose quotes the failure the reader will see. Say so in the prose at that point.

**When the compiler error is the lesson, CI verifies it.** Put the failing code in a trybuild
fixture under `tests/compile-fail/` in the crate version that follows it. The quoted error in the
prose then comes from the pinned toolchain and cannot rot silently.

**Package names carry the chapter and step.** `ch07-ownership-and-errors-v3`, so they are unique
in the workspace and greppable in CI output.

**Forward references are one sentence.** Ownership shows up long before its own chapter, and so
do generics. Say "the full story is in chapter 7" once and move on. Early chapters must not fill
up with caveats.

**Dependencies only where the subject demands them.** The fundamentals stay close to the standard
library. If a chapter needs a crate, the crate is already in `[workspace.dependencies]` or your
pull request explains why it should be.

**Snapshots for the application chapters.** From the acceptance-test chapters onward the code is
one evolving application under `app/`. At the end of each chapter it is copied to
`chapters/<nn>-<slug>/final/` with the package name suffixed by chapter, such as `poker-ch29`,
and the book includes from that copy.

## Tools

The toolchain is pinned in `rust-toolchain.toml` and rustup picks it up. The checks also need:

| Tool | What it does |
|---|---|
| [just](https://github.com/casey/just) | runs the recipes below |
| [cargo-nextest](https://nexte.st) | test runner |
| [cargo-deny](https://embarkstudios.github.io/cargo-deny/) | licence and advisory check |
| [mdbook](https://rust-lang.github.io/mdBook/) | builds the book |
| [typos](https://github.com/crate-ci/typos) | spell check |
| [lychee](https://lychee.cli.rs) | link check |

Then:

```
just check    # everything CI runs except the link check
just links    # the link check, needs the network
just test     # nextest across the workspace
just doctest  # doc tests, which nextest cannot run
just lint     # clippy with warnings denied
just fmt      # rustfmt
just book     # build the book
just prose    # dash and quote check on the prose
just serve    # local preview
```

## Pull request checklist

- [ ] Every step crate compiles and its tests pass under `cargo nextest run -p <crate>`
- [ ] Every code block in the chapter is an include from a step crate
- [ ] Compiler errors quoted in prose come from the pinned toolchain; where the error is the
      lesson, a compile-fail fixture covers it
- [ ] Chapter follows the template and ends with "Wrapping up"
- [ ] Forward references are one sentence each
- [ ] Chapter is in `book/src/SUMMARY.md` and `just book` builds with no link or spelling errors
- [ ] `just prose` passes and the prose has been read against the tells in STYLE.md
- [ ] Opinion passages say how sure they are and what would change the position
- [ ] The chapter's issue is linked

## Reporting mistakes

Open an issue with the `errata` label. Quote the sentence or the code, say what is wrong, and if
you know the fix, say that too. Small fixes are welcome as pull requests directly.
