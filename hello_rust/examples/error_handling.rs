#![allow(unused)]
use std::str::FromStr;

#[derive(Debug)]
enum MathError {
    DivisionByZero,
}

fn safe_divide(n: i32, d: i32) -> Result<i32, MathError> {
    if d == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(n / d)
    }
}

fn parse_and_divide(a: &str, b: &str) -> Result<i32, String> {
    let n = i32::from_str(a).map_err(|e| e.to_string())?;
    let d = i32::from_str(b).map_err(|e| e.to_string())?;
    Ok(n / d)
}

fn main() {
    // 1) panic implicit (commented):
    // let v = vec![1, 2, 3];
    // println!("{}", v[99]); // panic: out of bounds

    // 2) Option
    let v = vec![1, 2, 3];
    match v.get(1) {
        Some(val) => println!("v[1] via get = {val}"),
        None => println!("no v[1]"),
    }

    // 3) Result + custom enum
    match safe_divide(10, 2) {
        Ok(val) => println!("10/2 = {val}"),
        Err(err) => println!("Error: {:?}", err),
    }
    match safe_divide(10, 0) {
        Ok(val) => println!("10/0 = {val}"),
        Err(err) => println!("Error: {:?}", err),
    }

    // 4) Result + ? for propagation
    match parse_and_divide("20", "4") {
        Ok(val) => println!("20/4 = {val}"),
        Err(e) => println!("parse_and_divide error: {e}"),
    }
    match parse_and_divide("x", "4") {
        Ok(val) => println!("x/4 = {val}"),
        Err(e) => println!("parse_and_divide error: {e}"),
    }
}
