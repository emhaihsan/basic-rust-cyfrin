pub mod a;

pub fn print() {
    println!("rust");
}

fn private_print() {
    a::print();
    println!("private");
}
