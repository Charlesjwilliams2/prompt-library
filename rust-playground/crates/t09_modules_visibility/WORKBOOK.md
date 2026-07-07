# 09 — Modules & Visibility

## What this code does

Splits code across three files — `main.rs`, `billing/mod.rs`, and
`billing/invoice.rs` — to show how Rust's module tree controls what's
visible where: private-by-default, `pub(crate)`, and `pub` with
re-exports.

## Features to spot

- [ ] `mod billing;` in `main.rs` pulling in a directory as a module
- [ ] `mod invoice;` inside `billing/mod.rs` pulling in a child module
- [ ] `pub use invoice::Invoice;` re-exporting a child's item at the parent's level
- [ ] A `pub(crate)` item, visible crate-wide but not beyond
- [ ] A private (no modifier) item, visible only within its own module (and children, via `super::`)
- [ ] `super::` used from a child module to reach a private item in its parent

## Guiding questions

1. Draw the module tree: `crate` → `billing` → `invoice`. For each of
   `tax_rate`, `round_to_cents`, and `total_with_tax` in `billing/mod.rs`,
   mark whether `main.rs` can call it directly.
2. `invoice.rs`'s `total_with_tax` method calls `super::total_with_tax`.
   Why does `super::` work here when `round_to_cents` (also in the parent)
   is completely private with no `pub` at all?
3. Uncomment the `billing::tax_rate()` line in `main.rs` and build. It
   compiles — why? What would have to change about `tax_rate`'s visibility
   modifier for that same call to fail?
4. Uncomment the `billing::round_to_cents(...)` line instead (comment the
   first back out) and build. Read the compiler error — which word in the
   error message tells you this is a visibility problem rather than a
   missing-item problem?
5. `billing/mod.rs` does `pub use invoice::Invoice;` instead of requiring
   callers to write `billing::invoice::Invoice`. What's the benefit of
   that re-export for a caller in `main.rs`?

## Common pitfalls an LLM might introduce here

- Marking everything `pub` by default to "make it compile faster," which
  erases the visibility boundaries that would otherwise tell a reader
  (or a future refactor) what's actually meant to be an internal detail.
- Flattening a module that should be split (e.g. one 800-line `mod.rs`
  instead of child modules like `invoice.rs`), making it hard to see the
  visibility boundaries at all because everything lives in one scope.
- Using `pub(crate)` everywhere out of caution without ever using plain
  `pub` or fully private items, which suggests the visibility levels were
  applied mechanically rather than intentionally.
