# 08 — Derive Macros & Serde

## What this code does

Defines a `User` struct and a `Role` enum with several `#[derive(...)]`
attributes, then serializes/deserializes `User` to and from JSON with
`serde_json`, including a case where fields are missing from the input.

## Features to spot

- [ ] `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]` on a struct
- [ ] `#[derive(Default)]` paired with `#[default]` on one enum variant
- [ ] `#[serde(default)]` on a field to tolerate missing JSON keys
- [ ] `#[serde(skip_serializing_if = "...")]` to omit a field conditionally
- [ ] A round-trip: serialize, then deserialize, then compare with `==`
- [ ] A place where a derived trait (`Clone`, `PartialEq`) is the only reason a line of code compiles

## Guiding questions

1. `assert_eq!(user, round_tripped)` requires `PartialEq` on `User`. If you
   removed `PartialEq` from the derive list, what exactly would the
   compiler error say, and at which line?
2. The `minimal` JSON string has no `"role"` key. Trace what happens
   without `#[serde(default)]` on that field — would `serde_json::from_str`
   panic, return an `Err`, or something else? (Try removing the attribute
   and running `cargo run -p t08_derive_macros_serde` to check.)
3. `Role` derives both `Default` and needs `#[default]` on `Member`. What
   is `#[default]` doing that plain `#[derive(Default)]` alone couldn't do
   for an enum (as opposed to a struct, where every field just needs its
   own default)?
4. `nickname: Option<String>` uses `skip_serializing_if`. What would the
   JSON output look like without that attribute when `nickname` is `None`?
   Is that difference something a downstream consumer would care about?
5. If you saw `#[derive(Serialize, Deserialize)]` on a struct with no
   `serde` import errors, but the struct also derived `Copy`, would you
   expect that to work for every field type? What field type here (`name:
   String`) would make `#[derive(Copy)]` fail to compile?

## Common pitfalls an LLM might introduce here

- Deriving `Clone` (or `Copy`) reflexively on every struct without
  checking whether the fields actually support it, or whether cloning is
  even semantically meaningful for that type (e.g. cloning something that
  represents a unique resource, like a file handle wrapper).
- Forgetting `#[serde(default)]` on newly-added optional fields, which
  silently breaks deserialization of any JSON produced by an older version
  of the struct — a common source of "works in my test, breaks in
  production" bugs.
- Deriving `PartialEq` on floating-point-containing structs without
  realizing `f64`/`f32` comparisons via derived `==` don't handle NaN or
  precision the way domain logic often needs.
