#![allow(unused)]

fn main() {
    let x: u32 = 10;

    // 1) if/else dasar
    if x > 0 {
        println!("x > 0");
    } else if x < 0 {
        // Untuk u32, ini selalu false (compiler bisa beri warning)
        println!("x < 0");
    } else {
        println!("x = 0");
    }

    // 2) if/else sebagai ekspresi (menghasilkan nilai)
    let z: i32 = if x > 0 {
        println!("x > 0 (untuk z)");
        1 // nilai cabang if (tanpa ;)
    } else if x < 0 {
        println!("x < 0 (untuk z)");
        -1 // nilai cabang else if (tanpa ;)
    } else {
        println!("x = 0 (untuk z)");
        0 // nilai cabang else (tanpa ;)
    }; // titik koma menutup pernyataan let z = ...

    println!("z = {}", z);

    // 3) Contoh tipe konsisten: semua cabang i32
    let even_or_odd: i32 = if x % 2 == 0 { 0 } else { 1 };
    println!("even_or_odd = {}", even_or_odd);

    // 4) Tidak perlu tanda kurung di kondisi
    if x == 10 {
        println!("x == 10");
    }
}
