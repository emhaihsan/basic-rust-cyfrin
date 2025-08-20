use std::fmt::Debug;

fn main() {
    // 1) loop tak hingga + break kontrol
    let mut i = 0;
    loop {
        println!("loop {}", i);
        i += 1;
        if i > 5 {
            break; // keluar dari loop
        }
    }
    println!("Loop finished.");

    // 2) while dengan kondisi
    let mut j = 0;
    while j <= 5 {
        println!("while loop {}", j);
        j += 1;
    }
    println!("While loop finished.");

    // 3) for + range (eksklusif vs inklusif)
    for k in 0..6 {
        println!("for (exclusive) {}", k); // 0..6 → 0..=5
    }
    println!("For loop (exclusive range) finished.");

    for k in 0..=5 {
        println!("for (inclusive) {}", k);
    }
    println!("For loop (inclusive range) finished.");

    // 4) Iterasi array
    let arr = [10, 20, 30, 40, 50];

    // a) Index-based (butuh usize)
    let n: usize = arr.len();
    for idx in 0..n {
        println!("arr index: {}, value: {}", idx, arr[idx]);
    }

    // b) Direct iteration (lebih idiomatik)
    for val in arr {
        println!("arr value: {}", val);
    }

    // 5) Iterasi vector dan ownership
    let v = vec![10, 20, 30, 40, 50];

    // a) into_iter() implisit: mengambil ownership (v tidak bisa dipakai lagi)
    for val in v {
        println!("vec move {}", val);
    }
    // println!("{}", v.len()); // ERROR: v sudah pindah (moved)

    // b) iter(): meminjam, v tetap bisa dipakai
    let mut w = vec![1, 2, 3, 4, 5];
    for val_ref in w.iter() {
        println!("vec ref {}", val_ref); // &i32
    }
    // c) iter_mut(): pinjaman mutable → boleh ubah elemen
    for val_mut in w.iter_mut() {
        *val_mut *= 10;
    }
    println!("w after iter_mut: {:?}", w);

    // 6) loop sebagai ekspresi yang mengembalikan nilai via break value
    let mut t = 0;
    let result: &str = loop {
        println!("loop computation {}", t);
        t += 1;
        if t > 5 {
            break "loop computation ends here";
        }
    };
    println!("Loop returned: {}", result);
}
