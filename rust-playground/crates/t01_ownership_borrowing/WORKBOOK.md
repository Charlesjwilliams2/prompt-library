# 01 — Ownership & Borrowing

## What this code does

Builds a small `Report` struct, appends lines to it through a mutable
borrow, prints a summary through a shared borrow, then hands full ownership
of the report to `archive`, after which the original `report` variable can
no longer be used.

## Features to spot

- [ ] A shared borrow (`&Report`) used where the function only needs to read
- [ ] A mutable borrow (`&mut Report`) used where the function needs to modify in place
- [ ] A function that takes ownership (`fn archive(report: Report)`) instead of borrowing
- [ ] `.clone()` used to sidestep a borrow/move restriction
- [ ] An explicit lifetime annotation (`<'a>`) on a function signature
- [ ] A commented-out line that would fail to compile if uncommented

## Guiding questions

1. Why does `summarize` take `&Report` instead of `Report`? What would break
   at the call site if it took `Report` instead?
2. `archive` takes ownership of `report`. Trace through `main`: what is the
   last line that could possibly use `report` again, and why?
3. `longest` needs the `<'a>` lifetime annotation on its signature but
   `summarize` doesn't need one on its return type. What's different about
   what each function returns?
4. Uncomment the last line of `main.rs` and run
   `cargo build -p t01_ownership_borrowing`. Read the compiler error. Does
   it match what you predicted in question 2?
5. Would you approve this code in review? Is `title_copy`'s `.clone()`
   actually necessary, or could the code use `title_ref` instead?

## Common pitfalls an LLM might introduce here

- Taking `Report` by value "just to be safe," forcing every caller to give
  up ownership even when they only needed to read a field.
- Sprinkling `.clone()` everywhere to make borrow-checker errors disappear,
  rather than restructuring the borrows — this compiles, but it's often
  covering up a design that fights the ownership model instead of using it.
- Adding lifetime annotations to functions that don't need them (the
  compiler would have accepted elided lifetimes), which is harmless but
  signals the author wasn't sure why the annotation was there.
