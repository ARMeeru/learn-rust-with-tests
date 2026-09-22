# Install Rust

The official installer is [rustup](https://rustup.rs). It puts three things on your machine: the
compiler, `cargo`, and a way to switch between compiler versions. Follow the one-line instruction
on that page for your operating system, then open a new terminal and check:

```text
$ cargo --version
cargo 1.98.1 (797e8a9bc 2026-08-05)
```

Your version may be newer. That is fine.

## What a toolchain is, and why this book pins one

A toolchain is one version of the compiler together with its standard library and tools. rustup
can hold several side by side. A new stable Rust ships every six weeks, and most of the time
nothing you rely on changes. But compiler messages do change wording between releases, and this
book quotes a lot of compiler messages.

So the repository behind this book pins a toolchain in a file called `rust-toolchain.toml` at its
root. When you run `cargo` inside the repository, rustup reads that file and uses the version it
names, downloading it the first time. Every compiler error quoted in these pages was produced by
that version, so what you see on your screen matches what you read.

You do not need to do anything for this to work. It is worth knowing about because when you
start your own projects you will want the same trick: one line in that file, and everyone on the
project builds with the same compiler.

## The commands you will use constantly

`cargo` is the build tool, the test runner and the package manager, all in one. Four commands
carry most of this book.

```text
cargo new hello          # a new binary project in a folder called hello
cargo new --lib greet    # a new library project
cargo test               # build and run the tests
cargo run                # build and run the program
```

Two more keep code tidy, and we run them constantly:

```text
cargo fmt                # format every file the way the Rust community expects
cargo clippy             # the linter; it catches mistakes the compiler allows
```

`cargo fmt` ends arguments about style. `cargo clippy` is opinionated and often right; when it
suggests a change, read the suggestion before dismissing it, because it is usually teaching you
an idiom.

## Your editor

Install rust-analyzer support for whichever editor you use. VS Code, Zed, Neovim, Helix and the
JetBrains tools all have it, either built in or as an extension. It gives you three things this
book leans on:

- Compiler errors shown inline as you type, so the "try to run the test" step often happens
  before you run anything.
- Rename, extract function, and inline variable as safe refactorings. Refactoring is a third of
  the loop this book teaches, so these need to be one keystroke, not a manual search.
- Go to definition and hover for types. When a compiler message mentions a type you have not
  seen, hovering it is faster than searching.

Set the editor to run `cargo fmt` on save. Then you never think about formatting again.

## How the code in this book is laid out

Every code sample in this book lives in the
[repository](https://github.com/ARMeeru/learn-rust-with-tests) as a real project that compiles
and passes its tests. The book pulls its code blocks from those projects, so what you read is
what runs.

Each chapter has one folder under `chapters/`, and inside it one project per step, named `v1`,
`v2` and so on. When the text says "the code so far is in `chapters/02-hello-world/v3`", you can
go there and run it:

```text
git clone https://github.com/ARMeeru/learn-rust-with-tests
cd learn-rust-with-tests/chapters/02-hello-world/v3
cargo test
```

The first time you do this rustup will fetch the pinned toolchain, which takes a minute.

You do not have to use the repository. Following along by typing the code into your own project
is a better way to learn, and every chapter is written so that you can. The repository is there
for when you get stuck and want to compare against a known-good state.

## Wrapping up

You should now have `cargo` on your path, an editor that shows compiler errors as you type and
formats on save, and an idea of what the toolchain pin in the repository is for. The next chapter
writes the first program and the first test.
