#![allow(unused)]

fn borrow_string(s_ref: &String) {
    // Tidak mengambil ownership, hanya meminjam (read-only)
    println!("borrow {}", s_ref);
    // s_ref.push_str("x"); // ERROR: &String tidak bisa memodifikasi
}

fn modify_string(s_ref: &mut String) {
    // Meminjam secara mutable: boleh ubah isi
    s_ref.push_str(" is awesome!");
    println!("modified in function: {}", s_ref);
}

fn main() {
    // 1) Banyak immutable references diperbolehkan
    let s = String::from("rust");
    let s1 = &s;
    let s2 = &s;
    let s3 = s2; // sama-sama &s
    println!("s: {}, s1: {}, s2: {}, s3: {}", s, s1, s2, s3);

    // 2) Satu mutable reference pada satu waktu
    let mut t = String::from("rust");
    {
        let t_mut = &mut t; // pinjam mut
        t_mut.push_str(" 🦀"); // modifikasi melalui ref
        // t_mut last use di baris ini (NLL akan mengakhiri borrow di sini)
    }
    // Setelah t_mut tak lagi digunakan, kita boleh buat pinjaman mut lain
    {
        let t_mut2 = &mut t;
        t_mut2.push_str("🦀");
    }
    println!("t now: {}", t); // "rust 🦀🦀"

    // 3) Tidak boleh campur &T aktif dengan &mut T aktif
    let mut u = String::from("rust");
    let u1 = &u;
    let u2 = &u;
    println!("u1: {}, u2: {}", u1, u2); // last use dari u1/u2 di sini
    // Setelah pemakaian terakhir u1/u2, kita boleh buat &mut
    let u_mut = &mut u;
    u_mut.push_str(" crab");
    println!("u after mut: {}", u);

    // 4) Fungsi dengan referensi: tidak memindahkan ownership
    let original_s = String::from("borrow me");
    borrow_string(&original_s);
    println!("still have: {}", original_s);

    let mut modifiable_s = String::from("Rust");
    modify_string(&mut modifiable_s);
    println!("after function: {}", modifiable_s);

    // 5) Anti contoh (tidak dikompilasi, hanya ilustrasi):
    // - Dangling reference: mengembalikan &String ke data milik fungsi sendiri
    // fn dangle() -> &String { let s = String::from(\"x\"); &s }
    // error: returns a reference to data owned by the current function
}
