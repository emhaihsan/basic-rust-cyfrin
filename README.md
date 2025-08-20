# Rust Basics — Cyfrin Updraft Learning Notes

This repository documents my learning journey through the Rust Basic Course at Cyfrin Updraft.

## How to run the examples

All example programs live in `hello_rust/examples/`. Run any example with:

```bash
cargo run --example <example_name>
```

Examples covered so far:

- `hello` — Minimal "Hello, World!" program.
  - Run: `cargo run --example hello`
- `variables` — Variable bindings, mutability, and basic assignments.
  - Run: `cargo run --example variables`
- `func` — Defining and calling functions, parameters and return values.
  - Run: `cargo run --example func`
- `scalar` — Scalar types (integers, floats, booleans, chars) basics.
  - Run: `cargo run --example scalar`
- `tuples` — Creating, destructuring, and accessing tuple elements.
  - Run: `cargo run --example tuples`
- `arrays_slices` — Fixed-size arrays and slices (`&[T]`), indexing and ranges.
  - Run: `cargo run --example arrays_slices`
- `strings` — `String` vs `&str`, building, slicing (where safe), and printing.
  - Run: `cargo run --example strings`
- `overflow` — Integer overflow behavior (debug vs release) and checked/wrapping/saturating/overflowing methods.
  - Run: `cargo run --example overflow`
- `if_else` — Basic if/else control flow and if/else as expressions.
  - Run: `cargo run --example if_else`
- `loops` — `loop` with `break`, `while`, `for` with ranges, array/vector iteration, ownership vs `.iter()`/`.iter_mut()`, and `loop` returning a value with `break value`.
  - Run: `cargo run --example loops`
- `vectors` — Create vectors, push/pop, index vs safe `.get()`, mutate elements, and slicing.
  - Run: `cargo run --example vectors`
- `hashmaps` — Create and print `HashMap`, insert, `get` (returns `Option<&V>`), and update with `entry(...).or_insert(...)`.
  - Run: `cargo run --example hashmaps`
- `enums` — Simple enum variants (unit, tuple-like, struct-like) and basic usage.
  - Run: `cargo run --example enums`
- `structs` — Named-field, tuple, and unit-like structs, field access, shorthand init, update syntax, and `#[derive(Debug)]`.
  - Run: `cargo run --example structs`
- `match` — Pattern matching basics, exhaustiveness with `_`, multiple patterns `|`, ranges `..=`, bindings with `@`, matching `Option` and `Result`, and using `match` as an expression.
  - Run: `cargo run --example match`
- `if_let` — Concise single-pattern matching for enums (e.g., `Option`, `Result`), with optional `else`.
  - Run: `cargo run --example if_let`
- `stack_heap` — Perbedaan Stack vs Heap; tipe tetap di stack (`i32`, `[T; N]`), tipe dinamis di heap (`String`, `Vec<T>`), metadata vs data.
  - Run: `cargo run --example stack_heap`
- `ownership` — Tiga aturan ownership, move vs copy, drop saat keluar scope, fungsi mengambil ownership.
  - Run: `cargo run --example ownership`
- `borrowing` — Borrowing & references (`&T`, `&mut T`), aturan kombinasi, NLL (borrow berakhir di pemakaian terakhir), fungsi yang meminjam.
  - Run: `cargo run --example borrowing`

## Practice exercises

Additional practice is under `rust-crash-course/topics/` as small Cargo crates. Run tests per topic with:

```bash
cd rust-crash-course/topics/<topic>/exercises
cargo test
```

Examples of completed topics include variables/functions, scalar types, tuples, arrays/slices, strings, enums, structs, vectors, hash maps, if/else, loops/for, and pattern matching.

## Prerequisites

- Rust toolchain (rustc, cargo): https://www.rust-lang.org/learn/get-started

## Notes

- Examples use standard library only.
- Prints often rely on `#[derive(Debug)]` and the debug formatter (`{:?}` or `{:#?}`).
- Safe patterns are preferred (e.g., `.get()` for vectors, explicit handling with `match`).
