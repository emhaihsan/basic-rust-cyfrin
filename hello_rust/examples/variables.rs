#![allow(unused)] // menonaktifkan warning unused pada rust

fn main() {
    // Variabel pada rust itu default immutable
    let mut x = 1;
    x += 1;
    println!("{}", x);

    // type inference
    let y: i32 = -1; // implisit
    let z = -1;
    println!("y: {y}, z: {z}");

    // shadowing
    let x: i32 = 1;
    let x: i32 = 2; // shadow x (i32)
    let x: bool = true; // shadow lagi, kini bool
    println!("x bool: {x}");

    //.. type placeholder
    let x: _ = 1234; // infer i32
    println!("x: {x}");

    // constant:
    const NUM: u32 = 1;
    println!("NUM: {NUM}")
}
