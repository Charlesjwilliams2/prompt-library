# 10 — Reading Cargo.toml & Dependencies

## What this "code" does

Unlike the other topics, the lesson here is the `Cargo.toml` itself, not
`main.rs`. Each dependency line was chosen to illustrate a different
version-requirement style, including one that's a genuine smell.

## Features to spot

- [ ] A caret requirement (`"1"`, meaning `>=1.0.0, <2.0.0`)
- [ ] A fully pinned exact version (`"=1.0.128"`)
- [ ] An unconstrained wildcard version (`"*"`)
- [ ] A `[dev-dependencies]` section, separate from `[dependencies]`
- [ ] A `[features]` entry with no dependency gated behind it
- [ ] `features = ["derive"]` enabling optional functionality in a dependency

## Guiding questions

1. `serde = { version = "1", features = ["derive"] }` — what version range
   does the plain `"1"` actually permit? Would `serde = "2.0.0"` satisfy
   this requirement if it existed?
2. Why might `serde_json` be pinned to an exact version (`"=1.0.128"`) while
   `serde` is left as a caret range? What's the tradeoff either way, and
   which would you pick for a project you plan to maintain for years?
3. `once_cell = "*"` accepts literally any published version, including a
   hypothetical breaking `2.0.0` or `3.0.0`. What's the risk of leaving a
   dependency unconstrained like this in a real project?
4. Why are `serde_test` and similar testing-only crates placed under
   `[dev-dependencies]` instead of `[dependencies]`? What would change
   about a published binary/library if a test-only crate ended up in
   `[dependencies]` by mistake?
5. The `extra-logging` feature under `[features]` gates nothing — no
   dependency, and (check `main.rs`) no `#[cfg(feature = "extra-logging")]`
   anywhere in the code. What would you ask the author of this Cargo.toml
   before approving it?

## Common pitfalls an LLM might introduce here

- Writing `version = "*"` (or omitting a version entirely, which some
  tooling defaults to a wildcard) for a dependency — this is easy for an
  LLM to produce when it's unsure of the "current" version number and
  wants something that "just works," but it removes any protection
  against breaking upstream changes.
- Declaring a feature flag with no corresponding `#[cfg(...)]` anywhere in
  the code, or vice versa — using `#[cfg(feature = "x")]` without ever
  declaring `x` under `[features]`, which fails silently rather than
  loudly.
- Putting a crate needed only for tests under `[dependencies]` instead of
  `[dev-dependencies]`, which bloats the compiled binary and its
  dependency tree for end users who never run the tests.
