use crate::foo;

pub fn print() {
    println!("a");
}
pub fn print_foo() {
    foo::print();
}

pub struct S {
    pub id: u32,
    name: String,
}
pub fn build(id: u32) -> S {
    S {
        id,
        name: "".into(),
    }
}
