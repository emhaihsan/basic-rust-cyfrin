#![allow(unused)]

// A function returning a reference must tie the return lifetime to input(s)
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Multiple independent lifetimes – no relation needed if nothing is returned
fn print_refs<'a, 'b>(x: &'a str, y: &'b str) {
    println!("print_refs: {x} | {y}");
}

// Struct holding a reference needs a lifetime parameter
#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
}

// Methods for a lifetime-parameterized type must mention that lifetime
impl<'a> Book<'a> {
    fn edit(&mut self, new_title: &'a str) {
        self.title = new_title;
    }
}

// 'static lifetime: lives for the entire program (string literals)
fn static_greeting() -> &'static str {
    "Hello, world!"
}

// Elision example: input and output references – compiler can infer lifetime
fn last<'a>(s: &'a str) -> &'a str {
    // explicit, for clarity; the following could also be `fn last(s: &str) -> &str`
    let bytes = s.as_bytes();
    if let Some(pos) = bytes.iter().rposition(|&b| b == b' ') {
        &s[pos + 1..]
    } else {
        s
    }
}

// Another elision example: a simple identity over &str (no need to name lifetime)
fn id_str(s: &str) -> &str {
    s
}

// A tiny helper to show placeholder lifetime '_
fn underscore_demo() -> &'static str {
    let s: &'_ str = "This has an elided (inferred) lifetime"; // here it's actually 'static
    s
}

// Example of potentially dangling reference (kept commented to compile)
// fn dangling_example<'a>() -> &'a str {
//     let local = String::from("temp");
//     &local // ERROR if uncommented: returning reference to a dropped local
// }

fn main() {
    // longest_str demo
    let a = String::from("Hello");
    let b = String::from("Rust rust");
    let z = longest_str(a.as_str(), b.as_str());
    println!("longest_str: {z}");

    // multiple lifetimes
    print_refs("left", "right");

    // struct + impl with lifetime
    let mut book = Book { title: "Old Title" };
    println!("before edit: {:?}", book);
    book.edit("New Title");
    println!("after  edit: {:?}", book);

    // struct + impl with lifetime
    let mut book = Book { title: "Old Title" };
    println!("before edit: {:?}", book);
    book.edit("New Title");
    println!("after  edit: {:?}", book);
}
