# 12 — Reading Compiler Errors & Clippy Output

## What this code does

Compiles cleanly under plain `cargo build`, but contains three
intentional style issues — two of which `cargo clippy` will flag, and one
it won't — plus one commented-out block that produces a genuine compiler
error (not a lint) when uncommented.

## Features to spot

- [ ] A `bool == true` comparison clippy will flag (`clippy::bool_comparison`)
- [ ] A `match` with one real arm and a `_ => {}` fallback clippy flags as preferring `if let` (`clippy::single_match`)
- [ ] A `.clone()` call that's just as wasteful as the two above, but that clippy does *not* flag by default
- [ ] A commented-out block that would cause a real `error[E0382]`, not a warning

## Guiding questions

1. Run `cargo clippy -p t12_reading_compiler_clippy` before reading
   further. Clippy should report exactly two warnings. Find both in the
   source before checking the terminal output against your guess.
2. For the `is_empty == true` line: what does clippy suggest instead, and
   why does it consider the literal comparison unnecessary rather than
   just a style preference?
3. `total_length` clones every `String` before calling `.len()`, but
   clippy stays silent about it (confirm this yourself — it doesn't
   appear in the two warnings from question 1). Since `.len()` only needs
   `&str`/`&String`, what would the fixed version look like, and does it
   change the function's behavior at all — or only its cost? Why do you
   think a linter would miss something this mechanical?
4. Uncomment the block at the bottom of `main.rs`, run
   `cargo build -p t12_reading_compiler_clippy`, and read the full error
   output. Which line number does rustc point to for the *move*, and which
   line does it point to for the *use after move*? Are they the same line?
5. Clippy warnings don't stop `cargo build` from succeeding, but they can
   stop `cargo clippy -- -D warnings` (used in CI) from succeeding. Why
   might a team choose to run clippy with `-D warnings` in CI rather than
   just leaving warnings as warnings?

## Common pitfalls an LLM might introduce here

- Treating "it compiles, and clippy is quiet" as equivalent to "it's good
  code" — as `total_length` shows, not every needless-clone-style smell is
  clippy-flagged by default. Clippy catches a lot, but it's a floor, not a
  guarantee.
- Reacting to a real compiler error by adding `.clone()` or restructuring
  awkwardly just to make the red text go away, without reading *which*
  specific error code (`E0382`, `E0499`, etc.) was reported or what it
  actually means — the fix that "makes it compile" isn't always the fix
  that's actually correct.
- Never running `cargo clippy` at all during an LLM-assisted session,
  since a plain `cargo build` or `cargo run` will report success even
  when clippy would flag real issues.
