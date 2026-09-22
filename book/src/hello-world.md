# Hello, World

[You can find all the code for this chapter here](https://github.com/ARMeeru/learn-rust-with-tests/tree/main/chapters/02-hello-world).

It is traditional for your first program in a new language to be
[Hello, World](https://en.m.wikipedia.org/wiki/%22Hello,_World!%22_program).

Make a new project and step into it:

```text
cargo new hello
cd hello
```

`cargo` has already written the program for us in `src/main.rs`:

```rust
{{#include ../../chapters/02-hello-world/v1/src/main.rs:all}}
```

Run it with `cargo run`.

## How it works

A Rust program starts at a function called `main`. The `fn` keyword defines a function with a
name and a body.

`println!` prints a line to the terminal. The exclamation mark means it is a macro rather than a
plain function. For now the only difference that matters is that macros can take a variable
number of arguments, which is why `println!` can format things for you later on.

## How to test

How do you test this? It is good to separate your "domain" code from the outside world, meaning
the side effects. Printing to the terminal is a side effect. The string we print is our domain.

So we separate the two, which makes the domain easy to test:

```rust
{{#include ../../chapters/02-hello-world/v2/src/main.rs:code}}
```

We have a new function, `hello`, and this time the definition says what it returns: `-> &'static
str`. Read that as "a string slice that lives for the whole program". A string literal like
`"Hello, world"` is baked into the binary, so it lives forever, and this is the type of it. We
will meet the other kind of string in a moment.

`main` now prints whatever `hello` returns. The `{}` inside the `println!` string is a
placeholder, filled in by the argument that follows.

Now we write a test for `hello`, in the same file:

```rust
{{#include ../../chapters/02-hello-world/v2/src/main.rs:test}}
```

Run `cargo test`:

```text
running 1 test
test tests::hello_world ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

It passed. Just to check, break it on purpose by changing the `want` string and run the tests
again. Then put it back.

Notice that you have not had to choose a testing framework or install anything. Testing is built
into the language and the tool. The test is ordinary Rust.

### Writing tests

Writing a test is writing a function, with a few rules.

- The function is marked with `#[test]`. That attribute is what makes `cargo test` find it.
- Tests usually live in a module marked `#[cfg(test)]`, conventionally called `tests`. That
  attribute means the module is compiled only when testing, so none of it ends up in the real
  program.
- `use super::*;` brings everything from the enclosing file into the test module, so the test
  can see `hello`.
- A test fails by panicking. `assert_eq!` panics if its two arguments are not equal, and prints
  both of them when it does. We will see what that looks like shortly.

Some new syntax appeared as well.

#### Declaring variables

`let got = hello();` declares a variable and gives it a value. The compiler works out the type
from the right-hand side, so we did not have to write it. Variables are immutable unless you say
otherwise, which we will get to when we need it.

#### `assert_eq!`

Another macro. It takes the value you got and the value you wanted, and when they differ it
fails the test with a message showing both. It is the assertion you will use most, and its
failure message is good enough that we rarely need to write our own.

### Rust's documentation

Rust ships its documentation with the toolchain. `rustup doc` opens the standard library
reference, the book, and everything else in your browser, offline. `rustup doc std::string`
jumps straight to a module.

Almost every function in the standard library has an example in its documentation, and those
examples are compiled and run as tests when the standard library is built, so they are never
stale. That is a habit worth copying, and we will, later in the book.

### Hello, YOU

Now that we have a test, we can change the software safely.

In the last example we wrote the test after the code, so that you could see how to write a test
and declare a function. From here on we write the test first.

The next requirement is to let us say who we are greeting. Start by capturing that requirement in
a test. This is basic test-driven development, and it makes sure the test is actually testing
what we want. When you write tests after the fact, there is a risk that the test keeps passing
even when the code does not work as intended.

```rust
{{#include ../../chapters/02-hello-world/v4/src/main.rs:test}}
```

Run `cargo test` and the compiler stops you:

```text
error[E0061]: this function takes 0 arguments but 1 argument was supplied
  --> src/main.rs:15:19
   |
15 |         let got = hello("Chris");
   |                   ^^^^^ ------- unexpected argument of type `&'static str`
   |
note: function defined here
  --> src/main.rs:1:4
   |
 1 | fn hello() -> &'static str {
   |    ^^^^^
help: remove the extra argument
   |
15 -         let got = hello("Chris");
15 +         let got = hello();
   |
```

Rust's compiler messages are long, and the habit to build is to read the first line and the
underlined code, then only go further if those two did not explain it. Here the first line says
it all: we called `hello` with one argument, and it takes none. The compiler even offers a fix,
though it is the wrong one for us, because the test is right and the function is behind. The
line numbers in these quotes are from my file and will not match yours; ignore them.

When you use a statically typed language it is important to listen to the compiler. It
understands how your code should snap together so you do not have to. In this case it is
telling you exactly what to do next: `hello` has to accept an argument.

Change `hello` to take a name:

```rust
{{#include ../../chapters/02-hello-world/v3/src/main.rs:code}}
```

and make `main` pass one too, `hello("world")`, so it compiles.

Two things changed in the signature. The argument is `name: &str`, a borrowed string slice, the
type you use when a function only needs to read a string. And the return type is `String`, not
`&'static str`. A `String` is an owned, growable string that lives on the heap. We need it
because we are about to build a greeting out of pieces at run time, and the result cannot be a
literal baked into the binary. The distinction between `&str` and `String` is the first sight of
ownership; the full story is in chapter 7.

`.to_string()` turns the literal into a `String` so the types line up. The compiler is happy.
The test is not:

```text
running 1 test
test tests::hello_to_people ... FAILED

failures:

---- tests::hello_to_people stdout ----

thread 'tests::hello_to_people' panicked at src/main.rs:17:9:
assertion `left == right` failed
  left: "Hello, world"
 right: "Hello, Chris"
```

This is `assert_eq!` doing its job: both values, labelled, with the line number. We finally have
a compiling program that does not meet its requirement, which is exactly the state we want to be
in before we write the real code. In the repository, this step's test is marked `should_panic`
so that the checks stay green while the code stays honestly broken; the output above is what
your own test prints.

Make it pass by using the name:

```rust
{{#include ../../chapters/02-hello-world/v4/src/main.rs:code}}
```

`format!` is `println!`'s sibling: same placeholders, but it returns a `String` instead of
printing. Writing a variable's name inside the braces, `{name}`, fills the placeholder with that
variable.

Run the tests and they pass. Normally, as part of the TDD cycle, we should now refactor.

### A note on source control

At this point, if you are using source control, and you should be, I would commit the code as it
stands. We have working software backed by a test.

I would not push to main though, because I plan to refactor next. It is nice to have the commit
in case the refactoring gets into a mess; you can always go back to the working version.

### Constants

There is not much to refactor, but we can introduce a language feature, constants:

```rust
{{#include ../../chapters/02-hello-world/v5/src/main.rs:code}}
```

A constant is declared with `const`, a name in capitals by convention, and its type, which the
compiler does not infer for constants. Placeholders in `format!` can name a constant just as they
name a variable.

After refactoring, run the tests again to make sure nothing broke.

Constants capture the meaning of a value. `ENGLISH_HELLO_PREFIX` tells a reader what the string
is for; `"Hello, "` on its own does not.

## Hello, world... again

The next requirement is that calling the function with an empty string should give us "Hello,
World" rather than "Hello, ".

Start with a new failing test:

```rust
{{#include ../../chapters/02-hello-world/v6/src/main.rs:test}}
```

Two tests now, each a function with `#[test]`. The name of a test is the only description it
gets, so spend a moment on it. `empty_string_defaults_to_world` reads like a requirement, which
is what a test is.

Run them and the new one fails:

```text
---- tests::empty_string_defaults_to_world stdout ----

thread 'tests::empty_string_defaults_to_world' panicked at src/main.rs:19:9:
assertion `left == right` failed
  left: "Hello, "
 right: "Hello, World"
```

While we have a failing test, fix the code with an `if`:

```rust
{{#include ../../chapters/02-hello-world/v6/src/main.rs:code}}
```

In Rust, `if` is an expression: it produces a value. So instead of assigning `name` inside the
branches, we write the whole `if` on the right of a `let` and it evaluates to whichever branch
ran. This declares a new `name` that shadows the parameter, which is allowed and common when the
new value is the old one cleaned up.

`name.is_empty()` is a method call. `&str` has many; this is the one that asks whether the string
has no characters.

Run the tests. Both pass: the new requirement is met and the old one is not broken.

It is important that your tests are clear specifications of what the code needs to do. In the
Go original of this chapter, this is where the tests get refactored to share an assertion helper,
because Go's failure messages need help to show both values. Rust's `assert_eq!` already prints
both, with labels, so there is nothing to extract here. If a test ever needs to explain itself
further, `assert_eq!` takes an optional message after the two values, formatted like `println!`.

### Back to source control

Now that we are happy with the code, I would amend the previous commit so that we only check in
the version with the constant and the default, together with its tests.

### Discipline

Let us go over the cycle again.

- Write a test.
- Make the compiler pass.
- Run the test, see that it fails, and check the error message is meaningful.
- Write enough code to make the test pass.
- Refactor.

This may seem tedious, but sticking to the feedback loop is important.

Relevant tests, and the chance to refactor under their protection, are the point.

Seeing the test fail matters because it shows you the failure message. As a developer it is very
hard to work in a codebase where a failing test does not tell you what is wrong. Rust's compiler
gives you a second kind of failure message, earlier and often more precise, and reading those
well is a skill this book practises on purpose.

By keeping your tests fast and making them one command to run, you can get into a state of flow
while writing code. Without tests you are committing to checking your code by hand, running the
program, which breaks the flow. You will not save time, especially in the long run.

## Keep going! More requirements

We have more requirements. We now need a second parameter, the language of the greeting. If the
language is one we do not recognise, default to English.

We should be confident we can use TDD to flesh this out.

Write a test for a user passing in Spanish, and add it to the others:

```rust
{{#include ../../chapters/02-hello-world/v7/src/main.rs:spanish_test}}
```

Remember not to cheat. Test first. When you run the tests the compiler should complain that you
are calling `hello` with two arguments instead of one:

```text
error[E0061]: this function takes 1 argument but 2 arguments were supplied
  --> src/main.rs:18:19
   |
18 |         let got = hello("Elodie", "Spanish");
   |                   ^^^^^           --------- unexpected argument #2 of type `&'static str`
```

Fix the compile error by adding a second `&str` argument to `hello`. The compiler will then
complain about every other call, in the other tests and in `main`, because they pass one
argument. Fix those by passing an empty string as the language. Now everything compiles, and
every test passes apart from the new one:

```text
---- tests::in_spanish stdout ----

thread 'tests::in_spanish' panicked at src/main.rs:20:9:
assertion `left == right` failed
  left: "Hello, Elodie"
 right: "Hola, Elodie"
```

We can use `if` to check whether the language is Spanish and change the greeting. Do that, see
the tests pass, then refactor. There are magic strings, and one of them is repeated. Here is
where I ended up:

```rust
{{#include ../../chapters/02-hello-world/v7/src/main.rs:code}}
```

Two new things. `return` leaves the function early with a value; without it, a function returns
the value of its last expression, which is why there is no `return` on the final line and no
semicolon after it. And constants can be compared with `==` just like anything else.

### French

- Write a test asserting that passing `"French"` gives you `"Bonjour, "`.
- See it fail, and check the message is easy to read.
- Make the smallest reasonable change to the code.

Here is the test I wrote:

```rust
{{#include ../../chapters/02-hello-world/v8/src/main.rs:french_test}}
```

You probably added another `if`. That works. But two `if`s checking the same variable are a smell,
and there is a better tool.

## `match`

When you have several checks against one value, Rust's `match` is the tool. It is more than a
`switch`: the compiler checks that the arms cover every possible value, and the whole `match` is
an expression that produces the value of whichever arm ran.

```rust
{{#include ../../chapters/02-hello-world/v8/src/main.rs:code}}
```

Each arm is a pattern, an arrow, and a value. `_` is the pattern that matches anything, so it
plays the part of the default. Because we are matching on a `&str`, the compiler cannot know we
have covered every string, so the `_` arm is required. That will change in a moment.

Add a test for a greeting in a language of your choice and see how little it takes to extend
the function.

### one...last...refactor?

Passing the language around as a string works, but it lets a caller write `"spanish"` or
`"Spansh"` and silently get English. The compiler cannot help, because every string is a valid
string.

Rust has a better fit for "one of a fixed set of options": an `enum`. Write the test for it first,
changing one call to use a value that does not exist yet:

```text
error[E0433]: cannot find type `Language` in this scope
  --> src/main.rs:44:35
   |
44 |         let got = hello("Elodie", Language::Spanish);
   |                                   ^^^^^^^^ use of undeclared type `Language`
```

Then declare the type, change the signature, and update every call:

```rust
{{#include ../../chapters/02-hello-world/v9/src/main.rs:code}}
```

```rust
{{#include ../../chapters/02-hello-world/v9/src/main.rs:test}}
```

A few new concepts:

- `enum Language { English, Spanish, French }` declares a type with exactly three possible
  values, written `Language::Spanish` and so on. There is no fourth. Misspelling one is a compile
  error, not a silent default.
- The `match` no longer has a `_` arm. Because the compiler knows all three variants, it can
  check that we handled every one. Add a fourth variant later and the `match` will refuse to
  compile until you add its arm, which is exactly the help you want when the list grows.
- `greeting_prefix` is a separate function, and it is private. In Rust everything is private
  unless you write `pub` in front of it, so hiding the internals of the algorithm took no effort.
  It returns `&'static str` because every prefix is a literal.
- `main` now loops over the three languages with `for`, which we will look at properly in the
  iteration chapter.

The "unrecognised language defaults to English" requirement has quietly disappeared, because an
unrecognised language is no longer possible. That is the enum doing the work the `_` arm used to
do, at compile time instead of run time. When a requirement can be made unrepresentable rather
than handled, that is usually the better design, and Rust gives you the tools to do it often.

## Wrapping up

Who knew you could get so much out of Hello, world?

By now you should have some understanding of:

### Some of Rust's syntax

- Writing tests with `#[test]`, a `#[cfg(test)]` module, and `assert_eq!`
- Declaring functions with arguments and return types
- `let`, `const`, `if` as an expression, `match` and `enum`
- `&str` and `String`, and that the difference is about ownership, which chapter 7 covers
- `format!` and `println!`

### The TDD process and why the steps matter

- Write a failing test and see it fail, so we know the test is relevant and its failure message
  is easy to understand. In Rust the first failure is often the compiler's, and that message is
  worth reading as carefully as a test's.
- Write the smallest amount of code to make it pass, so we know we have working software.
- Then refactor, backed by the safety of the tests, to end up with code that is easy to work with.

We went from `hello()` to `hello("name")` to `hello("name", Language::French)` in small,
easy-to-understand steps.

This is trivial compared to real software, but the principles stand. TDD is a skill that needs
practice, and by breaking problems into pieces small enough to test, you will have a much easier
time writing software. The code for the final step is in `chapters/02-hello-world/v9`.
