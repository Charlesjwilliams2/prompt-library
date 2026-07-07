# 04 — Enums & Pattern Matching

## What this code does

Models a background job's state as a `JobStatus` enum where each variant
carries different data, then reads that data back out with `match`, a
match guard, `matches!`, and `if let`.

## Features to spot

- [ ] An enum with variants that carry different data shapes (unit, struct-like)
- [ ] An exhaustive `match` with no `_` catch-all arm
- [ ] A match guard (`if *progress_percent >= 100`)
- [ ] Matching on a field's exact value inside a struct-like variant (`retryable: true`)
- [ ] `matches!` used to test a shape without binding its data
- [ ] `if let` used to handle exactly one variant

## Guiding questions

1. `describe` has no `_ => ...` arm. What does the compiler guarantee you,
   in exchange for having to write out every variant?
2. If someone added a new `JobStatus::Cancelled` variant tomorrow, what
   would happen to `describe` — would it silently ignore the new case, or
   would something make you fix it?
3. `is_terminal` uses `matches!(status, JobStatus::Completed { .. } |
   JobStatus::Failed { .. })` instead of a `match` with `true`/`false`
   arms. Why is `matches!` a better fit here?
4. The two `Failed` arms in `describe` match on `retryable: true` and
   `retryable: false` separately instead of one arm with an `if` guard on
   `retryable`. Is there a meaningful difference, or is this just style?
5. Would you trust `JobStatus::Failed { reason: String, retryable: bool }`
   to prevent an invalid state, or can you construct a `Failed` value here
   that doesn't make sense? Compare that to using two separate structs,
   `RetryableFailure` and `PermanentFailure`, instead.

## Common pitfalls an LLM might introduce here

- Adding a `_ => {}` catch-all "just in case," which silently swallows
  future variants instead of forcing a compile error when someone extends
  the enum — this is one of the biggest reasons to prefer exhaustive
  matches over `_`.
- Modeling what should be an enum as a struct with several `Option<T>` or
  `bool` fields instead (e.g. `is_running: bool, is_failed: bool, reason:
  Option<String>`), which allows nonsensical combinations the enum version
  makes impossible to construct.
- Overusing `if let` chains instead of `match` when there are actually
  three or more cases to handle, losing the compiler's exhaustiveness
  check in the process.
