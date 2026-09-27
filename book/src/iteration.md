# Iteration

[You can find all the code for this chapter here](https://github.com/ARMeeru/learn-rust-with-tests/tree/main/chapters/04-iteration).

Go does repetition with one keyword, `for`; there is no `while` and no `do`, and Learn Go with
Tests calls that a good thing. Rust has three loop keywords: `loop`, `while` and `for`. We will
only need `for` in this chapter, because the test we write asks for a fixed number of
repetitions, and we will say a word about the other two once the test is green.

The exercise is the same one the Go chapter uses: write a function that repeats a character five
times, then benchmark it, then see what the benchmark says about a refactor. Make the project
with `cargo new --lib iteration`, as you did for `adder` in the last chapter.

## Write the test first

There is nothing new in this test, so we can go straight to it:

```rust
{{#include ../../chapters/04-iteration/v1/src/lib.rs:test}}
```

Ignore the `should_panic` line for a moment; it is explained under the failure output below, and
you do not write it in your own file.

The function takes a `char`. A `char` is a single Unicode character and its own four-byte type,
not a string of length one, and it is the natural way to say "the thing to repeat".

## Try to run the test

Run `cargo test`:

```text
error[E0425]: cannot find function `repeat` in this scope
 --> src/lib.rs:7:24
  |
7 |         let repeated = repeat('a');
  |                        ^^^^^^ not found in this scope
  |
help: consider importing one of these functions
  |
3 +     use std::array::repeat;
  |
3 +     use std::io::repeat;
  |
3 +     use std::iter::repeat;
  |
3 +     use core::array::repeat;
  |
  = and 1 other candidate
```

Same story as the start of the last chapter: there is no function called `repeat`. The help text
is new, though: the compiler knows several standard library functions with that name and offers
to import each of them. One of them, `std::iter::repeat`, repeats a value any number of times,
which sounds exactly like what we are writing; we meet the iterator machinery behind it in
chapter 21. For now we write our own.

## Write the minimal amount of code for the test to run and check the failing test output

Keep the discipline. All we need is enough to compile, so the test can fail for the right reason:

```rust
{{#include ../../chapters/04-iteration/v1/src/lib.rs:code}}
```

The stub returns an empty `String` and ignores its argument. Run the tests:

```text
running 1 test
test tests::repeats_a_character_five_times ... FAILED

failures:

---- tests::repeats_a_character_five_times stdout ----

thread 'tests::repeats_a_character_five_times' panicked at src/lib.rs:13:9:
assertion `left == right` failed
  left: ""
 right: "aaaaa"
```

The test does its job: it wanted `"aaaaa"` and got an empty string. In the repository, this
step's test carries `#[should_panic(expected = "assertion `left == right` failed")]`, which turns
the failure above into a pass so that the whole book's test suite stays green while this crate
stays honestly broken. Your own test does not have that line, and the output above is what you
see.

## Write enough code to make it pass

Go's `for` follows the C tradition of three parts: a start, a condition and a step. Rust's `for`
instead walks through something that produces values, and the simplest such thing is a range:

```rust
{{#include ../../chapters/04-iteration/v2/src/lib.rs:code}}
```

Three things are new. `0..5` is a range, and it produces `0`, `1`, `2`, `3` and `4`, which is
five values; the loop runs once per value. The underscore in `for _ in` says we are not going to
use the value itself, we just want to go around five times. And `let mut` declares a variable
that is allowed to change, which Go never needs you to say. Rust makes mutability explicit:
without `mut`, the compiler rejects every attempt to change `repeated`, including the `push`
that appends the character to the end of the string.

Run the test and it passes.

## Refactor

The `5` now sits in the middle of the function. Nothing about it changes while the program runs,
which makes it a job for a named constant:

```rust
{{#include ../../chapters/04-iteration/v3/src/lib.rs:code}}
```

Constants are named in capitals, like `ENGLISH_HELLO_PREFIX` in the second chapter. The type,
`usize`, is Rust's type for counts and sizes.

While we are tidying: Rust's other two loop keywords are `loop` and `while`. `loop` runs forever
until you `break` out of it, and `while condition` runs as long as the condition holds. Neither
is needed here, since the count is fixed and no test drives a different shape of loop.

## How fast is it?

The Go chapter benchmarks `Repeat` with `go test -bench` and treats benchmarking as part of the
language's testing story. Rust's stable toolchain has no built-in benchmark harness, so the
community answer is [criterion](https://github.com/bheisler/criterion.rs), a crate that measures
and reports timings with some statistical care. It is the one dependency this chapter adds, a
dev-dependency because only the benchmark needs it.

Two declarations go in `Cargo.toml`:

```toml
{{#include ../../chapters/04-iteration/v4/Cargo.toml:bench}}
```

The repository copy says `criterion = { workspace = true }` because every crate in this book
takes its dependency versions from one shared table at the workspace root. In your project,
write `criterion = "0.8"` instead. The `[[bench]]` section declares a benchmark target: a file
under `benches/` that cargo compiles as its own program. `harness = false` says the file provides
its own `main`, which criterion supplies through the macros below.

Here is `benches/repeat.rs`:

```rust
{{#include ../../chapters/04-iteration/v4/benches/repeat.rs:all}}
```

The shape is close to a test: a function that receives a handle, criterion's `Criterion`, and
calls `bench_function` with a name and a closure. The closure is what gets timed, over and over.
Criterion already passes the closure's return value through `black_box` itself, so the work
cannot be thrown away as unused; the `black_box` around `'a'` stops the compiler from treating
the input as a known constant and computing with that constant instead of measuring. In your
project the first line says `use iteration::repeat;`; the repository names its crates after the
chapter and step, so the include above says `ch04_iteration_v4`. That is the only difference,
and it applies to every code block in this chapter.

Run `cargo bench`:

```text
Benchmarking repeat
Benchmarking repeat: Warming up for 3.0000 s
Benchmarking repeat: Collecting 100 samples in estimated 5.0000 s (493M iterations)
Benchmarking repeat: Analyzing
repeat                  time:   [9.9958 ns 10.079 ns 10.178 ns]
```

Criterion warms the code up, then collects one hundred samples and reports the time as a range:
a lower bound, a best estimate, and an upper bound for how long one call takes. The estimate
here is about ten nanoseconds. Yours will be different; timings depend on the machine and on
whatever else it is doing. The `go test -bench` output in the Go chapter reports a single
nanoseconds-per-operation number where criterion reports a range, and decides for itself how
many iterations to run, which is what those `493M iterations` are. You will also notice that
`cargo bench` compiles your unit tests and lists them as `ignored`, because benchmarks are the
only thing it runs.

Two kinds of line the quotes in this chapter leave out, because they vary from machine to
machine and from run to run: a note about which plotting backend criterion falls back to, and a
report of outlying measurements. Criterion also saves every run as a baseline. The next run in
the same project gets a `change` line comparing against it, and every benchmark output after
this one has one.

## Making room up front

The Go chapter's next refactor exists because Go strings are immutable: every `+=` in the
original `Repeat` built a brand new string and copied the old one into it, and `strings.Builder`
is the standard library's way to build a string without all that copying.

A Rust `String` is already a growable buffer, which is much closer to what `strings.Builder`
provides than to what a Go `string` is. Pushing onto it does not copy an ever-growing string;
the buffer grows in amortised steps. The part of the idea that still transfers is telling the
buffer how much room it will need before we start pushing:

```rust
{{#include ../../chapters/04-iteration/v5/src/lib.rs:code}}
```

`with_capacity` reserves memory up front, and it counts bytes. A `char` can be up to four bytes
long, and `len_utf8` is the method that says how many this particular one needs; for `'a'` that
is one.

Run `cargo bench` again, in the same project:

```text
Benchmarking repeat
Benchmarking repeat: Warming up for 3.0000 s
Benchmarking repeat: Collecting 100 samples in estimated 5.0000 s (690M iterations)
Benchmarking repeat: Analyzing
repeat                  time:   [7.1859 ns 7.2172 ns 7.2500 ns]
                        change: [−28.249% −27.575% −26.932%] (p = 0.00 < 0.05)
                        Performance has improved.
```

The `change` line is criterion comparing this run against the baseline it saved from the last
one, and its verdict is its own: just over a quarter less time, and repeated runs agreed. I did
not expect a gap that size, and I would not generalise the exact ratio from a microbenchmark
this small. One thing I checked before believing it: both versions make exactly one memory
allocation for five one-byte characters. `String::new()` starts with no room at all, so the
first `push` has to stop and grow the buffer mid-loop; `with_capacity` gets the growing over
with before the loop begins. The Go chapter's benchmark improved five-fold, because every
concatenation there really did copy the string so far. Rust starts from a better place, and the
same idea buys less. Your own numbers will differ, and on a quiet machine the two versions may
land closer together than mine did.

## Practice exercises

The Go chapter ends by leaving you three exercises. We do the first two here, as the last steps
of the chapter, and the third one is yours.

### Repeat a given number of times

Change the test so the caller says how many times to repeat the character:

```rust
{{#include ../../chapters/04-iteration/v6/src/lib.rs:test}}
```

Eight, not five, so the function cannot quietly keep its hard-coded count. Run the test:

```text
error[E0061]: this function takes 1 argument but 2 arguments were supplied
  --> src/lib.rs:17:24
   |
17 |         let repeated = repeat('a', 8);
   |                        ^^^^^^      - unexpected argument #2 of type `{integer}`
   |
note: function defined here
  --> src/lib.rs:3:8
   |
 3 | pub fn repeat(character: char) -> String {
   |        ^^^^^^
help: remove the extra argument
   |
17 -         let repeated = repeat('a', 8);
17 +         let repeated = repeat('a');
   |
```

The compiler takes the test's side in an odd way: it offers to remove the extra argument and
leave the hard-coded five in place. The test is the requirement, so the function is what
changes. The constant goes too; the count belongs to the caller now, and the capacity and the
range both come from it:

```rust
{{#include ../../chapters/04-iteration/v6/src/lib.rs:fix}}
```

Run the test and it passes. The benchmark needs the same update:

```rust
{{#include ../../chapters/04-iteration/v6/benches/repeat.rs:all}}
```

The character is hidden for the reason from the last section. The count gets the same treatment
because the benchmark should measure the function as it is, a general function over a count
that arrives at run time, rather than whatever the optimiser makes of one particular call site.

Run it:

```text
Benchmarking repeat
Benchmarking repeat: Warming up for 3.0000 s
Benchmarking repeat: Collecting 100 samples in estimated 5.0000 s (530M iterations)
Benchmarking repeat: Analyzing
repeat                  time:   [9.3859 ns 9.4746 ns 9.5752 ns]
                        change: [+29.524% +31.096% +32.390%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

Criterion is comparing against the last section's baseline, and it flags a regression of a
little over two nanoseconds. The cost is the count itself. It used to be a constant inside the
function, and the loop over it could be tied to that constant; now it arrives as a parameter
and stays a runtime value inside the body. I checked that the `black_box` at the call site is
not what costs it: passing a plain `5` instead measures the same. That is the price of letting
the caller choose, and for a function this small it is a small one.

### An example in the documentation

The Go book's second exercise is `ExampleRepeat`, a testable example in the documentation. The
Rust counterpart is the doc test from the last chapter, and it sits in the doc comment on the
function we just fixed:

```rust
{{#include ../../chapters/04-iteration/v6/src/lib.rs:code}}
```

The example imports the crate the same way the benchmark does: the repository copy says
`use ch04_iteration_v6::repeat;` where your file says `use iteration::repeat;`.

Run `cargo test` and the doc tests get their own section at the bottom, as in the last chapter:

```text
running 1 test
test tests::repeats_a_character_the_given_number_of_times ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests iteration

running 1 test
test src/lib.rs - repeat (line 3) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### The standard library's version

One more refactor, and it is the one the compiler was nudging us towards at the start of the
chapter. `std::iter::repeat_n` produces a value exactly `count` times; it is a close cousin of
the `std::iter::repeat` the compiler offered to import back then, and `collect()` gathers
whatever an iterator produces into a collection, a `String` here:

```rust
{{#include ../../chapters/04-iteration/v7/src/lib.rs:code}}
```

The doc comment comes along unchanged.

The benchmark does not change. Run it again:

```text
Benchmarking repeat
Benchmarking repeat: Warming up for 3.0000 s
Benchmarking repeat: Collecting 100 samples in estimated 5.0000 s (385M iterations)
Benchmarking repeat: Analyzing
repeat                  time:   [12.714 ns 12.828 ns 12.959 ns]
                        change: [+32.406% +33.801% +35.156%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

This time the comparison is fair in a way the last one was not: both versions take the count as
a runtime value, and `repeat_n` is still about a third slower than the loop. The Go chapter's
refactor made its benchmark five times faster; this one makes ours slower, by a few nanoseconds
on a function that was already tiny. I would still keep `repeat_n`: it says what it does in one
line, it is the form an experienced Rust programmer expects to read, and nothing in this book
has a hot path through `repeat`. If yours does, the benchmark is sitting right there in
`benches/` to tell you what the one-liner costs.

### Explore the standard library

The third exercise is yours, and it is the same one the Go chapter sets: spend some time
browsing the standard library. Open the documentation for `str` and `String`, with
`cargo doc --open` or on docs.rs, and look for the methods you would have reached for in other
languages: there are hundreds, from `repeat`, which answers this chapter's whole exercise in one
call, to `trim`, `split` and `replace`. Pick a few, guess what they do, and write a test each,
the way we have all chapter. Time spent in the standard library pays off for as long as you
write Rust.

## Wrapping up

What we have covered:

- More practice of the TDD workflow, under the fixed headings.
- `for` and ranges, `let mut`, `String::push`, and a first look at `loop` and `while`.
- `char`, and the difference between a character and a string.
- Benchmarks with criterion: `benches/`, `harness = false`, `black_box`, how to read the output,
  and what the `change` line compares. Rust's counterpart of `go test -bench`.
- `String::with_capacity`, and the finding that a Rust `String` already grows the way
  `strings.Builder` makes Go strings grow, so the win is smaller than the Go chapter's.
- `std::iter::repeat_n`, and a benchmark that shows the one-line form is slower than the loop
  for this input.
- A doc test as the counterpart of Go's `ExampleRepeat`.

The code for the final step is in `chapters/04-iteration/v7`.
