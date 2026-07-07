# 06 — Closures & Iterators

## What this code does

Builds a small list of orders, then computes totals and filters several
ways: a `filter`/`map`/`sum` chain, a closure that captures a variable from
its environment, a manual `for` loop doing the same work for comparison,
and a mutating closure counted through `FnMut`.

## Features to spot

- [ ] A closure passed as an argument to `filter`
- [ ] A closure that captures a variable from its enclosing scope (`threshold`)
- [ ] An iterator chain combining `filter`, `map`, and a terminal op (`sum`/`collect`)
- [ ] `fold` used to build up a value with no matching built-in adapter
- [ ] A manual loop equivalent to one of the iterator chains
- [ ] A closure that mutates a captured variable (`FnMut`), and where that matters

## Guiding questions

1. `is_fulfilled` has the type `|o: &&Order| ...` — a reference to a
   reference. Where does the second `&` come from? (Hint: look at what
   `.iter().filter(...)` hands its predicate.)
2. The `threshold` closure captures `threshold` without writing `move`.
   What would change if you added `move` before `|o| o.total_cents >
   threshold`, given that `threshold` is a plain `i32`?
3. Compare the `fulfilled_total` iterator chain to the manual `for` loop
   below it. Which one would you rather debug with a breakpoint, and
   which would you rather modify to add a new condition?
4. `counting_double` is declared `mut` and captures `call_count` by mutable
   reference. Why couldn't this closure be passed anywhere that expects a
   plain `Fn`?
5. If an LLM handed you a five-stage `.filter().map().flat_map().filter_map().collect()`
   chain with no intermediate variables, how would you go about verifying
   it does what its comment claims?

## Common pitfalls an LLM might introduce here

- Chaining so many iterator adapters in one expression that no human can
  verify it at a glance — breaking it into named intermediate steps (even
  at a small performance cost) is often the more reviewable choice.
- Using `.clone()` inside a `.map()` closure to avoid a borrow-checker
  complaint, when restructuring to iterate by value (`.into_iter()`) or
  borrow correctly would avoid the allocation entirely.
- Reaching for `.collect::<Vec<_>>()` in the middle of a chain "to be
  safe," forcing an intermediate allocation where a lazy iterator adapter
  would have done the same job without it.
