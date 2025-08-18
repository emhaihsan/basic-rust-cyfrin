fn return_many() -> (u32, bool) {
    (1u32, true)
}

fn main() {
    let t: (bool, char, u32) = (true, 'a', 1);
    println!("x: {}, y: {}", t.0, t.1);

    let (a, b, c) = t;
    println!("a: {}, b: {}, c: {}", a, b, c);

    let (_, _, d) = t;
    println!("partial destructure: d={}", d);

    let unit = ();
    println!("unit: {:?}", unit);

    let nested = (('a', 'b'), ('c', 'd'));
    println!("nested: {:?}", nested);

    let (num, flag) = return_many();
    println!("num: {}, flag: {}", num, flag);
}
