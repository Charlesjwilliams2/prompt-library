# 11 — Async/Await & Tokio

## What this code does

Runs a couple of `async fn`s concurrently with `tokio::join!`, spawns a
task with `tokio::spawn`, and — deliberately — includes one `async fn`
that blocks the runtime thread instead of yielding, to show a mistake that
compiles and "works" but defeats the point of async.

## Features to spot

- [ ] `#[tokio::main]` on `fn main` (macro that sets up a runtime and runs the async body)
- [ ] An `async fn` and a `.await` on its call
- [ ] `tokio::join!` running multiple futures concurrently
- [ ] `tokio::spawn` handing a future to the runtime as an independent task
- [ ] A `JoinHandle` being `.await`ed to retrieve a spawned task's result
- [ ] A blocking call (`std::thread::sleep`) inside an `async fn`, and why it's a bug

## Guiding questions

1. `fetch_price` calls `sleep(...).await` from `tokio::time`. What would
   happen — compile error, silent bug, or something else — if you replaced
   that with `std::thread::sleep(...)` the way `fetch_price_blocking` does?
2. `tokio::join!(fetch_price(...), fetch_price(...))` runs both futures
   concurrently on the current task. How is that different from writing
   `fetch_price("apple", 30).await; fetch_price("banana", 10).await;` as
   two separate statements?
3. What's the difference between `tokio::join!` and `tokio::spawn`? When
   would awaiting a `JoinHandle` from `tokio::spawn` behave differently
   from just `.await`ing the future directly?
4. `fetch_price_blocking` compiles fine and returns the "right" answer.
   Why is it still a bug? What happens to other tasks on the same runtime
   worker thread while `std::thread::sleep` is blocking it?
5. If an LLM handed you an `async fn` that calls a synchronous,
   CPU-heavy or blocking function (like file I/O without `tokio::fs`, or a
   blocking database driver call) directly with no wrapper, what would you
   ask before approving it? (Hint: look up `tokio::task::spawn_blocking`.)

## Common pitfalls an LLM might introduce here

- Calling a blocking function (`std::thread::sleep`, a blocking HTTP
  client, blocking file I/O) directly inside an `async fn` — this compiles
  without any error or warning, but silently stalls the async runtime,
  and is one of the hardest async bugs to spot just by reading code
  quickly.
- Awaiting futures sequentially in a loop (`for x in xs { f(x).await; }`)
  when they're independent and could run concurrently via `join_all` or
  spawned tasks — not a correctness bug, but a performance smell that's
  easy to miss since the code "looks" async.
- Forgetting to `.await` a spawned task's `JoinHandle` at all, silently
  discarding whether it succeeded, panicked, or is still running when the
  program exits.
