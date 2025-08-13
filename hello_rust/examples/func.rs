#[allow(unused)]

fn add_with_return(x: u32, y: u32) -> u32 {
    return x + y;
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn add_multiple(x: u32, y: u32) -> (u32, bool) {
    (x + y, true)
}

fn print_message(s: String) {
    println!("{s}{s}{s}{s}{s}");
}

fn main() {
    let x = 1;
    let y = 2;

    let z1 = add_with_return(x, y);
    let z2 = add(x, y);
    println!("{x} + {y} = {z1} (explicit), {z2} (implicit)");

    let (sum, ok) = add_multiple(10, 20);
    println!("sum={sum}, ok={ok}");

    print_message("🤩".to_string());
}
