# Learn Rust with Tests

A Rust port of [Learn Go with Tests](https://quii.gitbook.io/learn-go-with-tests) by Chris James.
Same idea, same shape: learn the language by writing a test first, watching it fail, making it
pass, and refactoring, one small step at a time. This project is not affiliated with or endorsed
by the original author.

Work in progress. Chapters are published as they are written. The table of contents shows what is
done and what is still a stub.

Read it at https://armeeru.github.io/learn-rust-with-tests/ or, if you prefer GitBook,
https://armeeru.gitbook.io/learn-rust-with-tests/. Both are built from this repository on every
push to `main`.

## Who this is for

You have some programming experience and want to learn Rust properly, with tests as the way you
find out whether you understood something. You do not need to know Rust. You do not need to have
read the Go book, though if you have, the chapters line up so you can read them side by side.

## Why tests

Rust's compiler catches a lot, and this book leans on that: many chapters start with a compiler
error and explain the one line of it that matters. But the compiler cannot tell you whether the
code does what you meant. Tests can, and writing them first keeps each step small enough to
understand.

## Running the code

Every code sample in the book is a real crate in this repository, and the book pulls its snippets
from those crates, so what you read is what compiles. Each chapter has one crate per step under
`chapters/`, so you can open the exact state the prose describes:

```
cd chapters/02-hello-world/v1
cargo test
```

The repository pins a Rust toolchain in `rust-toolchain.toml`. If you have
[rustup](https://rustup.rs), the right version is used automatically.

To run everything the way CI does you need a few extra tools. See [CONTRIBUTING.md](CONTRIBUTING.md).

## How this differs from the Go book

Most chapters map one to one. A few could not, because Rust has no equivalent of the Go feature
being taught, so the chapter keeps its goal and changes its subject. Each of those says so in its
first paragraph. The chapter on cancellation, the chapter that replaces reflection with serde,
and the chapter on paused time in tokio are the main ones.

## Contributing

Yes please. Read [CONTRIBUTING.md](CONTRIBUTING.md) and [STYLE.md](STYLE.md) first. Every chapter
has an issue; pick one that is open.

## Licence

MIT, the same as the original. See [LICENSE.md](LICENSE.md), which carries the original
copyright notice as well as this port's.
