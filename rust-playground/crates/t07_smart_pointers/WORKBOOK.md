# 07 — Smart Pointers

## What this code does

Builds a small tree using `Rc<RefCell<TreeNode>>` to allow multiple owners
of a mutable node on a single thread, then contrasts that with `Arc<Mutex<T>>`
for a counter genuinely shared across threads, and a plain `Box<T>` for
simple heap allocation.

## Features to spot

- [ ] `Rc<T>` used for shared ownership on a single thread
- [ ] `RefCell<T>` used for interior mutability (borrow-checked at runtime, not compile time)
- [ ] `Rc::clone` (a cheap pointer-count bump) vs. a deep `.clone()`
- [ ] `Arc<T>` used instead of `Rc<T>` because the value crosses threads
- [ ] `Mutex<T>` used instead of `RefCell<T>` because it needs to be thread-safe
- [ ] A plain `Box<T>` with no sharing or shared mutability involved
- [ ] A commented-out line demonstrating a `RefCell` runtime borrow panic

## Guiding questions

1. `Rc::clone(&child_a)` doesn't copy the `TreeNode` — what does it
   actually do, and how is that different from calling `.clone()` on the
   `TreeNode` itself?
2. Why can't `single_threaded_counter`'s tree just use plain `RefCell<TreeNode>`
   without the `Rc`? What does `Rc` add that `RefCell` alone doesn't?
3. `multi_threaded_counter` uses `Arc<Mutex<i32>>` where the single-threaded
   version uses `Rc<RefCell<TreeNode>>`. Swap them (mentally, or actually
   try it) — what would the compiler say if you used `Rc<RefCell<i32>>`
   across `std::thread::spawn` instead?
4. Uncomment the two-borrow block in `single_threaded_counter`, run
   `cargo run -p t07_smart_pointers`, and read the panic message. Why is
   this a runtime panic rather than a compile error, unlike a similar
   double-borrow mistake with plain references?
5. The final `Box<i32>` in `main` wraps a value that's `Copy` and tiny.
   Is the `Box` doing anything useful here, or is it the kind of wrapper
   an LLM adds out of habit? What would you change?

## Common pitfalls an LLM might introduce here

- Reaching for `Rc<RefCell<T>>` as a default "fix" whenever the borrow
  checker complains, even in single-owner code where a plain owned value
  or a `&mut` would work — this is probably the most common LLM-Rust
  over-engineering pattern, and it trades compile-time safety for
  runtime panics.
- Using `Arc<Mutex<T>>` in code that never actually spawns a thread,
  paying lock overhead for no concurrency benefit — a sign the code was
  copied from a concurrent example without checking whether concurrency
  applies here.
- Wrapping already-`Copy` types (like `i32`, `bool`) in `Box`, `Rc`, or
  `Arc` with no sharing need, adding an allocation and an indirection for
  no reason.
