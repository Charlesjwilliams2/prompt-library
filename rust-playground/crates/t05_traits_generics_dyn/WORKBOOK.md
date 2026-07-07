# 05 — Traits, Generics & Trait Objects

## What this code does

Defines a `Notifier` trait with two implementations, then calls it two
ways: `notify_once<N: Notifier>` (static dispatch, generic) and
`notify_all(&[Box<dyn Notifier>])` (dynamic dispatch, trait object) over a
`Vec` that mixes both concrete types.

## Features to spot

- [ ] A trait definition (`trait Notifier`)
- [ ] Two `impl Trait for Type` blocks for different concrete types
- [ ] A generic function with a trait bound (`fn f<N: Notifier>(...)`)
- [ ] A trait object behind a pointer (`Box<dyn Notifier>`)
- [ ] A collection holding multiple concrete types unified by one trait

## Guiding questions

1. Could `notify_all` be rewritten as `fn notify_all<N: Notifier>(notifiers:
   &[N], ...)`? Try it against the `mixed` vector in `main` — what breaks,
   and why?
2. What does "monomorphization" mean for `notify_once`, and why does that
   make it *not* a good fit for the `mixed` vector's use case even though
   it compiles for `email` and `sms` individually?
3. `notify_all` takes `&[Box<dyn Notifier>]` rather than `&[&dyn Notifier]`.
   What's the practical difference for the caller, and when would you
   prefer one over the other?
4. If `Notifier` only ever had one implementor in this program, would
   `dyn Notifier` still be justified anywhere? What would you simplify?
5. Would you flag `Box<dyn Notifier>` in a code review if the list of
   notifiers were always exactly two known types, `EmailNotifier` and
   `SmsNotifier`, and never grew? What would you suggest instead?

## Common pitfalls an LLM might introduce here

- Reaching for `Box<dyn Trait>` by default even when the set of concrete
  types is small, fixed, and known at compile time — an enum with one
  variant per type is often simpler and faster, and still lets you `match`
  when you need type-specific behavior beyond the trait.
- Writing an elaborate generic function with several trait bounds
  (`<T: Trait1 + Trait2 + Clone + Send + Sync>`) for something that's only
  ever called with one concrete type — the generality adds no value and
  makes the signature harder to read.
- Mixing static and dynamic dispatch inconsistently in the same module
  with no clear reason, making it hard to predict which functions allocate
  and which get inlined/specialized.
