# 02 — Lifetimes

## What this code does

Defines an `Excerpt` struct that borrows two string slices instead of
owning `String`s, plus two helper functions — one whose lifetime is elided
by the compiler, one whose lifetime must be written explicitly because it
takes two reference inputs.

## Features to spot

- [ ] A struct with a lifetime parameter (`struct Excerpt<'a>`)
- [ ] An `impl` block carrying the same lifetime parameter
- [ ] A function with an elided lifetime that still compiles
- [ ] A function that requires an explicit lifetime annotation to compile
- [ ] A commented-out block demonstrating a "does not live long enough" error

## Guiding questions

1. Why does `Excerpt` need `<'a>` at all — what would happen if you tried to
   write `struct Excerpt { source_title: &str, text: &str }` with no
   lifetime?
2. `first_sentence` has no explicit lifetime annotation. Why does the
   compiler not need one here, when `pick_shorter` does?
3. In `pick_shorter<'a>(first: &'a str, second: &'a str) -> &'a str`, both
   inputs share the *same* lifetime `'a`. What does that force the caller
   to guarantee about `a` and `b`?
4. Uncomment the block at the bottom of `main.rs`, run
   `cargo build -p t02_lifetimes`, and read the error message. Which
   variable is the compiler saying doesn't live long enough, and why?
5. If an LLM gave you a function with three or four lifetime parameters
   (`<'a, 'b, 'c>`), each on a different argument, what would you ask it
   before approving that code?

## Common pitfalls an LLM might introduce here

- Adding a lifetime parameter to a struct that doesn't actually need to
  borrow anything, when it could just own a `String` instead — this adds
  complexity for the reader with no real benefit.
- Giving unrelated reference arguments the *same* lifetime name when they
  don't need to be tied together, which over-constrains callers
  unnecessarily.
- Reaching for `'static` to make a lifetime error disappear, which often
  just relocates the bug (e.g. by leaking memory or forcing an owned value
  to be leaked) rather than fixing the actual borrow relationship.
