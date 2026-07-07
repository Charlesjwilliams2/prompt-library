# Rust Playground

A learning workspace for reading and evaluating Rust code — especially Rust
that an LLM produced. The goal isn't to become a Rust expert who writes
idiomatic Rust from memory; it's to be able to open a piece of LLM-generated
Rust and answer: *what does this do, what could go wrong, and would I approve
it in review?*

## How it's organized

- **`crates/`** — thirteen small, numbered topics. Each one pairs a runnable
  example (`src/main.rs`) with a `WORKBOOK.md` lesson and a blank `NOTES.md`
  for your own answers. Work through them in order the first time; after
  that, jump to whichever topic a real snippet needs.
- **`inbox/`** — where you drop real LLM-generated Rust snippets you run into
  elsewhere (another repo, an agentic coding session, a PR). Each entry gets
  its own dated folder with the snippet plus an analysis file built from
  `inbox/_TEMPLATE.md`. This is the "agentic study" loop: paste in a snippet,
  ask Claude to help annotate it against the curriculum checklists below,
  and build a running record of what you've learned to spot.

## Curriculum

Each topic below is a crate under `crates/`. Run any of them independently
with `cargo run -p <crate-name>` from this directory.

| # | Topic | Crate |
|---|-------|-------|
| 1 | Ownership & borrowing | `t01_ownership_borrowing` |
| 2 | Lifetimes | `t02_lifetimes` |
| 3 | Error handling (`Result`/`Option`/`?`/anyhow/thiserror) | `t03_error_handling` |
| 4 | Enums & pattern matching | `t04_enums_pattern_matching` |
| 5 | Traits, generics & trait objects | `t05_traits_generics_dyn` |
| 6 | Closures & iterators | `t06_closures_iterators` |
| 7 | Smart pointers (Box/Rc/Arc/RefCell/Mutex) | `t07_smart_pointers` |
| 8 | Derive macros & serde | `t08_derive_macros_serde` |
| 9 | Modules & visibility | `t09_modules_visibility` |
| 10 | Reading Cargo.toml & dependencies | `t10_cargo_and_deps` |
| 11 | Async/await & tokio | `t11_async_tokio` |
| 12 | Reading compiler errors & clippy output | `t12_reading_compiler_clippy` |
| 13 | Common LLM code smells (capstone) | `t13_llm_code_smells` |

## Working the loop

1. Read a topic's `src/main.rs` cold — before reading `WORKBOOK.md` — and try
   to explain it out loud.
2. Open `WORKBOOK.md`: check off the features you spotted, then answer the
   guiding questions in that topic's `NOTES.md`.
3. Run it: `cargo run -p <crate-name>`, `cargo test -p <crate-name>`,
   `cargo clippy -p <crate-name>`.
4. When you find a real LLM-generated snippet elsewhere, add it to `inbox/`
   using `_TEMPLATE.md` and link it back to whichever topics apply.

## Commands

From `rust-playground/`:

```sh
cargo build --workspace          # everything compiles
cargo test --workspace           # sanity-check tests pass
cargo clippy --workspace --all-targets -- -D warnings   # lint (see note below)
cargo fmt --all                  # format
cargo run -p t03_error_handling   # run one topic
```

Topics 12 and 13 intentionally contain code clippy will flag — that's the
lesson. Their `WORKBOOK.md` says which warnings to expect, so run those two
crates' clippy checks separately from the rest of the workspace.
