# Chapter template

Every tutorial chapter in this book follows the shape below, copied from the Go book. The
headings are fixed so a reader always knows where they are in the loop. Copy this file to start a
new chapter and replace the notes under each heading.

A short introduction. What we are going to build, and which Rust idea it will teach. If the
chapter changes subject from its Go counterpart, say so here, in the first paragraph.

## Write the test first

The test, and nothing else. Explain any test syntax the reader has not seen before.

## Try to run the test

In Rust this is usually a compiler error. Quote it, then explain the one line that matters and
what the compiler is asking for. If the error is the lesson of the chapter, a compile-fail fixture
in the step crate verifies the quoted text.

## Write the minimal amount of code for the test to run and check the failing test output

Enough code to compile, no more. Often a function that returns a placeholder or calls `todo!()`.
Show the failing assertion so the reader sees the test doing its job.

## Write enough code to make it pass

The smallest change that turns the test green. Resist writing the general solution here.

## Refactor

Tidy with the test as a safety net. This is where idioms get introduced: the reader has working
code and can now see why the idiomatic version is better.

## Repeat for new requirements

Add the next requirement and go around the loop again. Use a heading that names the requirement
rather than repeating "Repeat for new requirements". Most chapters go around three to six times.

## Wrapping up

What the reader has learned, in a short list. Which Rust ideas appeared for the first time. What
the TDD process gave us in this chapter. Where the code for the final step lives.
