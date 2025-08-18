// examples/overflow.rs
fn main() {
    let mut x = u32::MAX;
    println!("Initial x: {}", x);
    x += 1; // Debug: panic. Release: wrap ke 0
    println!("u32 max: {}, x after increment: {}", u32::MAX, x);

    let a = u32::checked_add(u32::MAX, 1);
    let b = u32::checked_add(3, 1);
    println!("checked_add: {:?}, {:?}", a, b);

    let c = u32::wrapping_add(u32::MAX, 1); // 0
    let d = u32::wrapping_add(3, 1); // 4
    println!("wrapping_add: {} {}", c, d);

    // Explicit handling
    println!("checked: {:?}", u32::checked_add(u32::MAX, 1));
    println!("wrapping: {}", u32::wrapping_add(u32::MAX, 1));
    println!("saturating: {}", u32::saturating_add(u32::MAX, 1));
    println!("overflowing: {:?}", u32::overflowing_add(u32::MAX, 1));
}
