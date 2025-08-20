#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point3D(i32, i32, i32);

#[derive(Debug)]
struct Empty; // unit-like struct

#[derive(Debug)]
struct Circle {
    radius: u32,
    center: Point,
}

fn main() {
    // Named-field: inisialisasi biasa
    let p = Point { x: 1, y: 3 };
    println!("p (debug): {:?}", p);
    println!("p (pretty): {:#?}", p);
    println!("akses field p -> x: {}, y: {}", p.x, p.y);

    // Tuple struct: akses dengan indeks .0 .1 .2
    let p3d = Point3D(-1, 0, 5);
    println!("p3d: ({}, {}, {})", p3d.0, p3d.1, p3d.2);
    println!("p3d (debug): {:?}", p3d);

    // Unit-like struct
    let e = Empty;
    println!("empty (debug): {:?}", e);

    // Nested struct
    let circle = Circle {
        radius: 10,
        center: Point { x: 0, y: 0 },
    };
    println!("circle (pretty): {:#?}", circle);
    println!(
        "circle point -> x: {}, y: {}",
        circle.center.x, circle.center.y
    );

    // Field init shorthand: variabel lokal sama nama dengan field
    let x: i32 = 5;
    let y: i32 = 9;
    let p_short = Point { x, y }; // sama dengan Point { x: x, y: y }
    println!("p_short: {:?}", p_short);

    // Struct update syntax: buat baru dari struct lama, ubah sebagian field
    let p0 = Point { x: 1, y: 5 };
    let p1 = Point { x: 5, ..p0 }; // y diambil dari p0.y
    println!("p0: {:?}", p0); // i32 adalah Copy, jadi p0 masih bisa dipakai
    println!("p1 (updated from p0): {:?}", p1);

    // Mutability: ubah field setelah dibuat
    let mut p_update = Point { x: 1, y: 1 };
    println!("before update: {:?}", p_update);
    p_update.x += 1;
    p_update.y = 99;
    println!("after  update: {:?}", p_update);
}
