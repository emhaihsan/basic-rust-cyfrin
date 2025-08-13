#![allow(unused)]
use std::mem::size_of;

fn main() {
    // Signed
    let i0: i8 = -1;
    let i1: i16 = 2;
    let i2: i32 = -3;
    let i3: i64 = 4;
    let i4: i128 = -5;

    // Unsigned
    let u0: u8 = 1;
    let u1: u16 = 2;
    let u2: u32 = 3;
    let u3: u64 = 4;
    let u4: u128 = 5;

    // Arsitektur
    let i5: isize = -6;
    let u5: usize = 6;
    println!("sizes: isize {i5}, usize {u5}");

    // Float
    let f0: f32 = 0.01;
    let f1: f64 = 0.02_f64; // default f64

    // bool
    let b0: bool = true;
    let is_active = false;

    // char
    let c: char = 'a';
    let emoji: char = '👌';
    let space: char = ' ';
    let new_line: char = '\n';

    // Casting
    let ii: i32 = -1;
    let uu: u32 = ii as u32;

    // Min/Max
    let i_max = i32::MAX;
    let i_min = i32::MIN;

    println!("ints: {i0}, {i1}, {i2}, {i3}, {i4}, {i5}");
    println!("arch: {i5}, usize {u5}");
    println!("floats: {f0}, {f1}");
    println!("bools: {b0}, {is_active}");
    println!("chars: {c}, {emoji}, {space}, {new_line}");
    println!("casting: {ii}, {uu}");
    println!("min/max: {i_min}, {i_max}");
}
