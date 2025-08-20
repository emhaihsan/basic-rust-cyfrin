#![allow(unused)]
use std::num::ParseIntError;
use std::str::FromStr;

// Contoh fungsi yang mengembalikan Result
fn f1() -> Result<u32, String> {
    println!("f1");
    Ok(1)
}
fn f2() -> Result<u32, String> {
    println!("f2");
    Ok(2)
}

// Versi tanpa ?: verbose
fn f1_f2_match() -> Result<u32, String> {
    let out_1 = match f1() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    let out_2 = match f2() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    Ok(out_1 + out_2)
}

// Versi dengan ?: ringkas
fn f1_f2_question() -> Result<u32, String> {
    let out_1 = f1()?;
    let out_2 = f2()?;
    Ok(out_1 + out_2)
}

// Contoh propagasi error parsing: Result + ?
fn parse_two(a: &str, b: &str) -> Result<u32, ParseIntError> {
    let x = u32::from_str(a)?; // Err(ParseIntError) -> return Err
    let y = u32::from_str(b)?;
    Ok(x + y)
}

// Contoh Option + ?
fn first_char_to_digit(s: &str) -> Option<u32> {
    let ch = s.chars().next()?; // None -> return None
    let d = ch.to_digit(10)?; // None -> return None
    Some(d)
}

// Contoh konversi error via From
#[derive(Debug)]
enum MyErr {
    Parse(ParseIntError),
    Other(String),
}
impl From<ParseIntError> for MyErr {
    fn from(e: ParseIntError) -> Self {
        MyErr::Parse(e)
    }
}
fn parse_to_myerr(a: &str) -> Result<u32, MyErr> {
    let x = u32::from_str(a)?; // ParseIntError otomatis dikonversi ke MyErr via From
    Ok(x)
}

fn main() {
    println!("{:?}", f1_f2_match()); // Ok(3)
    println!("{:?}", f1_f2_question()); // Ok(3)

    println!("{:?}", parse_two("10", "5")); // Ok(15)
    println!("{:?}", parse_two("x", "5")); // Err(ParseIntError)

    println!("{:?}", first_char_to_digit("9abc")); // Some(9)
    println!("{:?}", first_char_to_digit("abc")); // None

    println!("{:?}", parse_to_myerr("42")); // Ok(42)
    println!("{:?}", parse_to_myerr("xx")); // Err(MyErr::Parse(...))
}
