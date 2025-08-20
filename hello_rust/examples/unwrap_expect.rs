fn main() {
    // Option
    let some_val: Option<&str> = Some("hello");
    println!("some_val.unwrap() = {}", some_val.unwrap());

    let none_val: Option<i32> = None;
    // Uncomment satu-satu untuk melihat panic:
    // println!("{}", none_val.unwrap());                 // default panic message
    // println!("{}", none_val.expect("expected a value"));// custom panic message

    // Result
    let ok: Result<i32, &str> = Ok(10);
    println!("ok.unwrap() = {}", ok.unwrap());

    let err: Result<i32, &str> = Err("boom");
    // println!("{}", err.unwrap());                      // default panic with Err shown
    // println!("{}", err.expect("calc failed"));         // custom panic message
}
