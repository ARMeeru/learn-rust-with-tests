# Learn Rust with Tests

Learn Rust by writing a test first, watching it fail, making it pass, and then tidying up. One
small step at a time, one idea per step.

This is a port of [Learn Go with Tests](https://quii.gitbook.io/learn-go-with-tests) by Chris
James, published under the same MIT licence. The structure and most of the exercises are his. The
Rust, and the chapters where Rust needed a different subject, are mine. This project is not
affiliated with or endorsed by the original author.

The book is a work in progress. Chapters appear in the sidebar as they are written; greyed-out
entries are planned but not yet done.

## Why tests

Rust's compiler catches a great deal, and this book leans on that. Many steps begin with a compiler
error, and the job is to read the one line of it that matters. But the compiler cannot tell you
whether the code does what you meant. Tests can. Writing the test first also forces each step to be
small enough to hold in your head, which is how most of the learning happens.

If you have not done test-driven development before, the cycle is:

1. Write a test for the behaviour you want. It fails, because the code does not exist yet.
2. Write the smallest amount of code that makes it pass.
3. Tidy the code, with the test proving you did not break it.
4. Repeat for the next requirement.

Every chapter walks that loop several times and ends with a section called "Wrapping up" that says
what you have learned.

## Who this is for

- People who want to learn Rust and already know how to program in something.
- People who know some Rust but want to get better at testing it.
- People who have read the Go book and want to see the same ideas in a language with a very
  different type system. The chapters line up, so you can read them side by side.

You do not need to know Rust.

## What you need

- A computer with a terminal you are comfortable in.
- [Rust installed through rustup](https://rustup.rs). The first chapter walks through this.
- A text editor. Anything with rust-analyzer support will make the compiler's messages easier to
  act on.
- Some programming experience: variables, functions, `if`, and the idea of a loop.

## How the code is organised

Every code sample in this book is a real crate in the
[repository](https://github.com/ARMeeru/learn-rust-with-tests), and the book pulls its snippets
from those crates. What you read is what compiles and passes its tests in CI.

Each chapter has one crate per step. When the text says "the code so far is in
`chapters/07-ownership-and-errors/v3`", you can open that directory and run `cargo test` to see
exactly the state the chapter describes.

## How this differs from the Go book

Most chapters map one to one. A few could not, because Rust has no equivalent of the Go feature
being taught, so the chapter keeps its goal and changes its subject. Each of those says so in its
first paragraph. The main ones:

- Go's chapter on reflection becomes a chapter on walking values generically with serde.
- Go's chapter on context becomes a chapter on cancellation with tokio.
- Go's chapter on the synctest fake clock becomes a chapter on tokio's paused time.
- Go's chapter on pointers and errors grows into a chapter on ownership, borrowing and errors,
  since ownership is the thing most people find hardest about Rust.

## Feedback

Mistakes, unclear passages and better ways of doing things: open an issue or a pull request on
[GitHub](https://github.com/ARMeeru/learn-rust-with-tests). The
[contributing guide](contributing.md) explains how chapters are built.

[MIT licence](https://github.com/ARMeeru/learn-rust-with-tests/blob/main/LICENSE.md)
