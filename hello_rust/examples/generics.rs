#![allow(unused)]

// Struct generic
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Enum generic sederhana (mirip Option)
#[derive(Debug)]
enum Wrapper<T> {
    Some(T),
    None,
}

// Fungsi generic
fn swap<A, B>(a: A, b: B) -> (B, A) {
    (b, a)
}

fn main() {
    // Point<T>
    let p_i32: Point<i32> = Point { x: 1, y: 2 };
    let p_f32: Point<f32> = Point { x: 1.5, y: 2.5 };
    println!("p_i32 = {:?}", p_i32);
    println!("p_f32 = {:?}", p_f32);

    // Wrapper<T>
    let w1: Wrapper<&str> = Wrapper::Some("hello");
    let w2: Wrapper<i64> = Wrapper::None;
    println!("w1 = {:?}", w1);
    println!("w2 = {:?}", w2);

    // swap<A,B>
    let a: u32 = 1;
    let b: i32 = 2;
    println!("Before swap: a={a}, b={b}");
    let (a, b) = swap(a, b); // a: i32, b: u32
    println!("After swap: a={a}, b={b}");
}
