#![allow(unused)]
use std::fmt::Debug;

fn main() {
    // 1) Dasar + ekshaustif
    let x: i32 = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("others"), // wajib ada agar ekshaustif
    }

    // 2) Multi nilai dan rentang
    let y = 7;
    match y {
        1 | 2 | 3 => println!("1, 2, or 3"),
        4..=10 => println!("4 to 10"),
        _ => println!("others"),
    }

    // 3) Binding dengan @
    let z = 10;
    match z {
        i @ 1..=10 => println!("1 to 10: found {}", i),
        _ => println!("others"),
    }

    // 4) Option<T>
    let maybe_num: Option<i32> = Some(9);
    let out: i32 = match maybe_num {
        Some(val) => val,
        None => 0,
    };
    println!("Option output: {}", out);

    // 5) Result<T, E>
    let res_ok: Result<i32, String> = Ok(123);
    let res_err: Result<i32, String> = Err("failed".to_string());

    match res_ok {
        Ok(v) => println!("Success: {v}"),
        Err(e) => println!("Error: {e}"),
    }
    match res_err {
        Ok(v) => println!("Success: {v}"),
        Err(e) => println!("Error: {e}"),
    }

    // 6) match sebagai ekspresi (menghasilkan nilai bertipe konsisten)
    let score = 75u8;
    let grade: char = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };
    println!("grade = {}", grade);
}
