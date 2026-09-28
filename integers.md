# Integers

[You can find all the code for this chapter here](https://github.com/ARMeeru/learn-rust-with-tests/tree/main/chapters/03-integers).

Integers work as you would expect. Let us write an `add` function to try things out, and use it
to practise the loop from the last chapter under the headings this book will use from here on.

This chapter is a library rather than a program, because `add` is something other code will call,
not something a person runs. Make one with `cargo new --lib adder`, which gives you `src/lib.rs`
instead of `src/main.rs`. Your projects so far might look like this:

```text
learn-rust-with-tests
    |
    |-> hello
    |    |- Cargo.toml
    |    |- src/main.rs
    |
    |-> adder
         |- Cargo.toml
         |- src/lib.rs
```

Each project has its own `Cargo.toml`, which names the crate and lists its dependencies. We have
none yet.

## Write the test first

Replace the contents of `src/lib.rs` with just the test:

```rust
    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn adds_two_numbers() {
        let sum = add(2, 2);
        let expected = 4;
        assert_eq!(sum, expected);
    }
```

Ignore the `should_panic` line for a moment; it is explained under the failure output below, and
you do not write it in your own file.

Two small things to notice. The values are integers now, so `assert_eq!` will print them as
numbers rather than quoted strings. And the test module is the whole file so far; `use super::*`
will pull in `add` as soon as it exists.

## Try to run the test

Run `cargo test` and inspect the compile error:

```text
error[E0425]: cannot find function `add` in this scope
 --> src/lib.rs:7:19
  |
7 |         let sum = add(2, 2);
  |                   ^^^ not found in this scope
  |
help: use the `.` operator to call the method `Add::add` on `{integer}`
  |
7 -         let sum = add(2, 2);
7 +         let sum = 2.add(2);
  |
```

The first line is the whole story: there is no function called `add`. The help text is the
compiler noticing that integers have a method by that name, from the `Add` trait that gives `+`
its meaning, and guessing you meant it. You did not, but it is a good example of how the
compiler's suggestions are worth reading even when you reject them.

## Write the minimal amount of code for the test to run and check the failing test output

Write enough code to satisfy the compiler and that is all. We want to check that the test fails
for the right reason.

```rust
pub fn add(x: i32, y: i32) -> i32 {
    0
}
```

`pub` makes the function public, so code outside this crate can call it. In the last chapter
everything was private, because nothing outside `main.rs` needed it. A library exists to be
called, so its interface is `pub`.

Rust has no shorthand for two parameters of the same type; each gets its own type annotation.

Now run the tests, and we should be happy that the test correctly reports what is wrong:

```text
running 1 test
test tests::adds_two_numbers ... FAILED

failures:

---- tests::adds_two_numbers stdout ----

thread 'tests::adds_two_numbers' panicked at src/lib.rs:13:9:
assertion `left == right` failed
  left: 0
 right: 4
```

In the repository, this step's test carries `#[should_panic(expected = "assertion `left == right`
failed")]`, which turns the failure above into a pass so that the whole book's test suite stays
green while this crate stays honestly broken. Your own test does not have that line, and the
output above is what you see.

## Write enough code to make it pass

In the strictest sense of TDD we should now write the minimal amount of code to make the test
pass. A pedantic programmer might change the `0` to a `4` and stop there.

Ah hah! Foiled again, TDD is a sham, right?

We could write another test with different numbers to force that to fail, but that becomes a game
of cat and mouse. Once we know more of the language I will introduce property-based testing, in chapter 16,
which stops this argument for good by letting the computer pick the numbers.

For now, fix it properly:

```rust
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

Rerun the tests and they pass.

## Refactor

There is not much in the code itself to improve. What we can improve is how a user of this
function finds out what it does.

In Rust, a comment starting with three slashes is a documentation comment. It attaches to the
item below it, and `cargo doc` turns it into a web page:

```rust
/// Takes two integers and returns the sum of them.
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

Run `cargo doc --open` and your browser shows the same kind of page you get for the standard
library, with `add` listed and that sentence under it. Editors show the same text when you hover
the function. It is worth a sentence on every public function, written for the person who will
call it without reading the body.

### Examples that are tested

If you want to go the extra mile, put an example in the documentation. You will find one on
almost every function in the standard library.

Code examples that live outside the codebase, in a readme or a wiki, drift out of date, because
nothing checks them. Rust's answer is that a code block inside a documentation comment is a test.
`cargo test` compiles it and runs it, and if the example stops being true, the build fails.

```rust
/// Takes two integers and returns the sum of them.
///
/// ```
/// use ch03_integers_v4::add;
///
/// let sum = add(1, 5);
/// assert_eq!(sum, 6);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

The example is ordinary Rust between two lines of three backticks. Inside it, the crate is
referred to by name, as a user would. In your project that line is `use adder::add;`. In the
repository the crates are named after the chapter and step, so the include above says
`ch03_integers_v4` instead; that is the only difference.

Run the tests and a second section appears in the output:

```text
running 1 test
test tests::adds_two_numbers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 1 test
test src/lib.rs - add (line 3) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The example ran, with no arrangement from us. To see it fail, change the `6` to a `7` and run
`cargo test --doc`, which runs only the documentation tests:

```text
---- src/lib.rs - add (line 3) stdout ----
Test executable failed (exit status: 101).

stderr:

thread 'main' panicked at /tmp/rustdoctestV3Vj5g/doctest_bundle_2024.rs:9:1:
assertion `left == right` failed
  left: 6
 right: 7
```

Put the `6` back. The point stands: documentation that lies now breaks the build.

Examples also appear on the `cargo doc` page, rendered as code, which is often the fastest way for
a reader to understand a function. If you publish a crate to crates.io, the same documentation
appears on docs.rs, built from the same comments.

## What about the edges?

Our function has a bug, or at least a surprise, that the Go original of this chapter does not have
to talk about. An `i32` holds whole numbers from about minus two billion to plus two billion. What
happens when we add past the top?

Write the test first, as a question. The first time through, leave off the `should_panic` line
you see below, so that the test is just a call and we can watch what happens:

```rust
    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn adding_past_the_maximum_panics() {
        add(i32::MAX, 1);
    }
```

`i32::MAX` is the largest value the type can hold. Run it without the attribute:

```text
running 1 test
test tests::adding_past_the_maximum_panics ... FAILED

failures:

---- tests::adding_past_the_maximum_panics stdout ----

thread 'tests::adding_past_the_maximum_panics' panicked at src/lib.rs:10:5:
attempt to add with overflow
```

In a debug build, which is what `cargo test` produces by default, Rust checks every arithmetic
operation and panics on overflow. That is a deliberate choice: silently wrapping around to a
negative number is how a great many bugs in other languages begin. Now run the same test with
`cargo test --release`:

```text
test tests::adding_past_the_maximum_panics ... ok
```

It passes, because in a release build the checks are off for speed and the addition wraps around.
The same code, two behaviours. That is worth knowing about before it finds you in production.

Since the debug behaviour is what we want to pin down, turn the observation into a real test by
saying what we expect. That is what the `should_panic` line above does: put it back.

`should_panic` with an `expected` message is how you test that something fails on purpose. The
test now passes in debug, because the panic happened and said the right thing. In release it
would fail, which is correct: the release build does not do what this test says.

## Adding without surprises

A caller who might hit the edge needs a way to find out instead of crashing. Rust's integers have
a method for exactly that, and we can wrap it in a function of our own. Test first:

```text
error[E0425]: cannot find function `try_add` in this scope
  --> src/lib.rs:32:20
   |
32 |         assert_eq!(try_add(2, 2), Some(4));
   |                    ^^^^^^^ not found in this scope
```

The second test fails the same way, one error per call. Same error as the start of the chapter,
same fix: write the function. Here are the two tests I wrote and the function that makes them
pass:

```rust
    #[test]
    fn try_add_adds_two_numbers() {
        assert_eq!(try_add(2, 2), Some(4));
    }

    #[test]
    fn try_add_reports_overflow() {
        assert_eq!(try_add(i32::MAX, 1), None);
    }
```

```rust
/// Adds two integers, or returns `None` if the sum does not fit in an `i32`.
///
/// ```
/// use ch03_integers_v6::try_add;
///
/// assert_eq!(try_add(1, 5), Some(6));
/// assert_eq!(try_add(i32::MAX, 1), None);
/// ```
pub fn try_add(x: i32, y: i32) -> Option<i32> {
    x.checked_add(y)
}
```

`Option<i32>` is a type that is either `Some(value)` or `None`. It is how Rust says "there might
not be an answer" without a special sentinel value and without a crash. `checked_add` on any
integer type returns exactly that: `Some` of the sum when it fits, `None` when it does not. Our
`try_add` is a thin wrapper, which is fine; the point is the signature. A caller can see from the
type alone that this addition might not produce a number, and the compiler will make them deal
with the `None` case before they can use the value. We will use `Option` a great deal from here
on, and chapter 8, on maps, looks at it properly.

The doc comment carries two examples this time, one for each outcome. Both run under
`cargo test`.

## Wrapping up

What we have covered:

- More practice of the TDD workflow, under the headings the rest of the book uses.
- `cargo new --lib`, `pub`, and `i32`.
- Documentation comments, and examples inside them that are compiled and run as tests.
- Integer overflow: a panic in debug builds, a silent wrap in release builds.
- `should_panic` for testing that something fails on purpose.
- `Option` and `checked_add`, the first sight of a type that says "maybe no answer".

The code for the final step is in `chapters/03-integers/v6`.
