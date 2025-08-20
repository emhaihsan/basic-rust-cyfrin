#![allow(unused)]

use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};

// Generic max with PartialOrd bound (by value)
fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x >= y { x } else { y }
}

// If you want to return by value for non-move usages, add Copy
fn max_copy<T: PartialOrd + Copy>(x: T, y: T) -> T {
    if x >= y { x } else { y }
}

// Avoid requiring Copy by returning a reference
fn max_ref<'a, T: PartialOrd>(x: &'a T, y: &'a T) -> &'a T {
    if x >= y { x } else { y }
}

// Demonstrate multiple bounds and where clause
trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}

impl B for f32 {}

impl B for i32 {}
impl C for i32 {}

// Single bound
fn process_a<T: A>(_item: T) {
    println!("process_a: T implements A");
}

// Multiple bounds inline
fn process_ab<T: A + B>(_item: T) {
    println!("process_ab: T implements A + B");
}

// Where-clause for readability
fn complex_process<T, U>(_t: T, _u: U)
where
    T: A + B,
    U: B + C,
{
    println!("complex_process: T(A+B), U(B+C)");
}

// A small helper to show Display + Debug bound
fn show_pair<T, U>(x: T, y: U)
where
    T: Display + Debug,
    U: Display + Debug,
{
    println!("pair: {x} ({x:?}), {y} ({y:?})");
}

fn main() {
    // max by value
    println!("max(u32): {}", max(10u32, 20u32));
    println!("max(f32): {}", max(3.5f32, 2.0f32));

    // max_copy for Copy types (e.g., char)
    println!("max_copy(char): {}", max_copy('a', 'z'));

    // max_ref to avoid Copy
    let s1 = String::from("apple");
    let s2 = String::from("banana");
    let m = max_ref(&s1, &s2);
    println!("max_ref(String): {}", m);

    // Trait bounds demos
    process_a(42u32); // u32: A
    process_ab(42u32); // u32: A + B
    complex_process(1u32, -2i32); // u32: A+B, i32: B+C

    show_pair(7, "hello");
}
