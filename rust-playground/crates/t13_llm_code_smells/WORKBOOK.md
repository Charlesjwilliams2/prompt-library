# 13 — Common LLM Code Smells (Capstone)

## What this code does

Five small functions, each demonstrating one anti-pattern that shows up
disproportionately often in LLM-generated Rust: an unnecessary
`Rc<RefCell<T>>`, `.unwrap()`-everywhere instead of proper error handling,
needless `.clone()` calls, an `Arc<Mutex<T>>` with no concurrency in
sight, and an overengineered generic bound list. None of it fails to
compile — that's exactly why it's easy to miss in review.

## Features to spot

- [ ] `Rc<RefCell<T>>` used with a single owner on a single thread (topic 07)
- [ ] `.unwrap()` used where `?` and a `Result` return type would propagate the error instead (topic 03)
- [ ] Multiple `.clone()` calls in a loop where borrows would suffice (topics 01, 12)
- [ ] `Arc<Mutex<T>>` used with no `std::thread::spawn` or async task anywhere nearby (topic 07)
- [ ] A generic function with trait bounds beyond what its body actually uses (topic 05)

## Guiding questions

1. `Cache` wraps its `Vec` in `Rc<RefCell<Vec<...>>>`, but every method
   takes `&self`, and nothing ever clones the `Cache` itself. What would
   `Cache` look like with a plain `RefCell<Vec<...>>` (no `Rc`) — and
   would anything actually break?
2. `parse_all` will panic on the first non-numeric input. Rewrite its
   signature so it returns `Result<Vec<i32>, std::num::ParseIntError>`
   instead — what's the one-line change inside the function once the
   signature changes?
3. Count the `.clone()` calls in `longest_name`. Which of them are
   cloning a `String` just to call `.len()` on it (which works fine on
   `&String` already), and which, if any, are actually necessary?
4. `count_vowels_unnecessarily_locked` never spawns a thread. What would
   this function look like with a plain `usize` and no `Arc`/`Mutex` at
   all? What did the original version cost at runtime for no benefit?
5. `print_first<T: Clone + Debug + PartialEq + Send + Sync>` only ever
   calls `.first()` and `{:?}` on `T`. Which of those five trait bounds
   does the function body actually require? What's the risk of leaving
   the unused ones in the signature?

## Common pitfalls an LLM might introduce here

This whole crate *is* the pitfalls list — each function above corresponds
to one earlier topic's "common pitfalls" section. The meta-lesson: none
of these five functions fail to compile, none of them crash on the happy
path shown in `main`, and a quick skim reads as plausible, working Rust.
That's exactly the gap this playground exists to close — the compiler
guarantees memory safety, not that the code is well-designed, efficient,
or handles its edge cases. Reading LLM-generated Rust well means checking
for these patterns deliberately, not waiting for the compiler to flag them
(it won't — clippy will catch some, but not all, of what's here).
