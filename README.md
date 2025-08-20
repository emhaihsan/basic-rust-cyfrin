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
- `if_let` — Concise single-pattern matching for enums (like `Option`, `Result`) with optional `else` clause.
  - Run: `cargo run --example if_let`
- `stack_heap` — Stack vs Heap: fixed-size types (`i32`, `[T; N]`) live on the stack; dynamic types (`String`, `Vec<T>`) store data on the heap; metadata vs data distinction.
  - Run: `cargo run --example stack_heap`
- `ownership` — The three ownership rules, moves vs copies, dropping at end of scope, and functions taking ownership.
  - Run: `cargo run --example ownership`
- `borrowing` — Borrowing and references (`&T`, `&mut T`), borrowing rules, non-lexical lifetimes (NLL), and functions that borrow.
  - Run: `cargo run --example borrowing`
- `error_handling` — `panic!` for unrecoverable errors, `Option` for optional values (`Some`/`None`), `Result` for recoverable errors (`Ok`/`Err`), use of the `?` operator, and custom error enums.
  - Run: `cargo run --example error_handling`
- `unwrap_expect` — Using `unwrap()` and `expect()` on `Option`/`Result` for quick extraction or panicking with a custom message.
  - Run: `cargo run --example unwrap_expect`
- `question` — The question mark operator `?` for concise error propagation on `Result`/`Option`, including error conversion via `From`.
  - Run: `cargo run --example question`
- `mods` — Rust modules: `mod`, `pub`, nested modules, `super`/`crate` paths, and file/directory organization.
- `bounds` — Trait bounds (`PartialOrd`, multiple bounds with `+`, where-clause), by-value vs by-ref.
  - Run: `cargo run --example bounds`
- `generic_traits` — Generic trait `List<T>`, concrete and generic impls, usage on tuples and vectors.
  - Run: `cargo run --example generic_traits`
- `lifetimes` — Explicit lifetimes on functions, struct with reference field, impl blocks, `'static`, and elision.
  - Run: `cargo run --example lifetimes`
- `async_hamburger` — Async sequential vs concurrent with Tokio (`join!`, `spawn`); demonstrates that futures are lazy and require `.await`.
  - Run: `cargo run --example async_hamburger`
- `threads_vs_async` — Comparison of native threads vs async/await (Tokio), including anti-patterns (blocking in a task) and the `spawn_blocking` solution.
  - Run: `cargo run --example threads_vs_async`
- `joinselect` — Demonstrates `tokio::join!` (wait for all) vs `tokio::select!` (first-wins + cancels others), including equal-time race and timeout pattern.
  - Run: `cargo run --example joinselect`

## Practice exercises

Additional practice is under `rust-crash-course/topics/` as small Cargo crates. Run tests per topic with:

```bash
cd rust-crash-course/topics/<topic>/exercises
cargo test
```

Examples of completed topics include variables/functions, scalar types, tuples, arrays/slices, strings, enums, structs, vectors, hash maps, if/else, loops/for, pattern matching, trait bounds, generic traits, and lifetimes.

## Prerequisites

- Rust toolchain (rustc, cargo): https://www.rust-lang.org/learn/get-started

## Notes

- Most examples use the standard library only; `async_hamburger` uses the Tokio runtime.
- Prints often rely on `#[derive(Debug)]` and the debug formatter (`{:?}` or `{:#?}`).
- Safe patterns are preferred (e.g., `.get()` for vectors, explicit handling with `match`).

## Iteration and Ownership: into_iter vs iter vs iter_mut

In Rust, `for x in collection` desugars to `for x in collection.into_iter()`, which moves ownership of the collection into the iterator. For `Vec<T>`, this consumes the vector; using it again afterwards results in `E0382: use of moved value`.

Minimal illustration (why a second loop fails):

```rust
let vals = vec![1, 2, 3];
for v in vals { /* owns each T */ }
// for v in vals { /* E0382: vals moved above */ }
```

To iterate without consuming, borrow with `.iter()` (immutable) or `.iter_mut()` (mutable):

```rust
let vals = vec![1, 2, 3];
for v_ref in vals.iter() {           // v_ref: &i32
    println!("{v_ref}");
}
for v_ref in vals.iter() {           // can borrow again immutably
    println!("{v_ref}");
}

let mut vals2 = vec![1, 2, 3];
for v_mut in vals2.iter_mut() {      // v_mut: &mut i32
    *v_mut *= 2;
}
println!("{:?}", vals2); // [2, 4, 6]
```

Quick reference:

- `into_iter(self)` → moves/consumes, yields `T`.
- `iter(&self)` → borrows immutably, yields `&T`, can reuse/loop again.
- `iter_mut(&mut self)` → borrows mutably, yields `&mut T`, can modify elements.

Tip: If you need to access the collection after a loop, prefer `iter()` or `iter_mut()` explicitly instead of relying on the default `into_iter()` semantics of `for`.

## Tokio Concurrency: join! vs select!

- `join!`:
  - Menjalankan beberapa future secara bersamaan dan menunggu semuanya selesai.
  - Mengembalikan tuple hasil sesuai urutan argumen.
  - Gunakan saat Anda butuh semua hasil (total waktu ≈ tugas paling lambat).
- `select!`:
  - Menjalankan beberapa future dan mengembalikan yang selesai pertama.
  - Future lain langsung dibatalkan (hemat resource, cocok untuk race/timeout).
  - Gunakan untuk mirror tercepat, fallback cepat, dan timeout (`select! { _ = sleep(dt) => ..., }`).

Contoh runnable ada di `hello_rust/examples/joinselect.rs`:

- Menunjukkan `join!` vs `select!`, tie-breaking (waktu sama), dan pola timeout.
- Jalankan: `cargo run --example joinselect`.
