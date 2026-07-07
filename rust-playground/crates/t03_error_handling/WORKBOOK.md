# 03 — Error Handling

## What this code does

Parses a tiny `key = value` config format. `parse_config` returns a
specific `ConfigError` enum (built with `thiserror`) so callers can match
on *why* it failed. `main`'s `run` function uses `anyhow::Result` at the
top level, where the exact variant no longer matters — only whether it
succeeded.

## Features to spot

- [ ] `#[derive(Error)]` from `thiserror` with `#[error("...")]` messages
- [ ] A `Result<T, E>` return type with a specific error enum
- [ ] `anyhow::Result` used at a boundary that doesn't need to match on error type
- [ ] The `?` operator propagating an error out of a function
- [ ] `.ok_or(...)` converting an `Option` into a `Result`
- [ ] `.map_err(...)` converting one error type into another
- [ ] A `match` on a `Result` instead of `?`, and a reason it's used there instead

## Guiding questions

1. Why does `parse_config` return `Result<Config, ConfigError>` instead of
   `anyhow::Result<Config>`? Who benefits from the more specific type?
2. `run()` uses `anyhow::Result<()>` and the `?` operator on a function that
   returns `Result<_, ConfigError>`. What has to be true about `ConfigError`
   for that `?` to compile? (Hint: look at what `thiserror`'s derive
   generates.)
3. Find the one place in `run()` that uses `match` instead of `?`. Why does
   it need to inspect the error there rather than just propagating it?
4. If this code used `.unwrap()` everywhere instead of `?`/`match`, what
   would happen the first time `parse_config` hit a malformed line — and
   would you notice the difference just by looking at the function
   signatures?
5. `ConfigError::NotANumber` stores both `key` and `value` as owned
   `String`s rather than borrowed `&str`. Why does it need to own them
   here, given where the error is constructed and returned from?

## Common pitfalls an LLM might introduce here

- Using `.unwrap()` or `.expect()` throughout "prototype" code and never
  circling back — this is probably the single most common LLM-Rust smell,
  and it silently turns every recoverable error into a crash.
- Returning `Box<dyn std::error::Error>` everywhere by default, even in
  library code where callers would benefit from matching on specific
  variants — it compiles, but it pushes the "what went wrong" question
  onto string-matching error messages instead of structured types.
- Mixing `anyhow` and specific error enums inconsistently across a
  codebase, so some functions are matchable and others aren't, with no
  clear rule for which is which.
