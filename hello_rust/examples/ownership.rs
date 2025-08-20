#![allow(unused)]

fn take_ownership(some_string: String) {
    // some_string mengambil ownership
    println!("Inside take_ownership: {}", some_string);
} // some_string di-drop di sini (keluar scope)

fn process_copy(value: i32) {
    // i32 adalah Copy, jadi yang diterima adalah salinan
    println!("Inside process_copy: {}", value);
}

fn main() {
    // 1) Setiap nilai punya owner
    let s = String::from("rust"); // owner: s
    let i = 1; // owner: i (i32, Copy)

    println!("s = {}, i = {}", s, i);

    // 2) Satu owner pada satu waktu (move)
    let s = String::from("dog");
    let s1 = s; // move: s -> s1 (s invalid)
    let s2 = s1; // move: s1 -> s2 (s1 invalid)
    println!("{}", s2);

    // Jika coba pakai s atau s1 di sini, compile error (moved):
    // println!("{}", s);  // error[E0382]
    // println!("{}", s1); // error[E0382]

    // 3) Owner keluar scope => drop
    {
        let s_inner = String::from("cat");
        println!("inner owner: {}", s_inner);
    } // s_inner di-drop di sini

    // Fungsi yang mengambil ownership
    let s3 = String::from("bird");
    take_ownership(s3);
    // println!("{}", s3); // moved ke take_ownership -> error jika diaktifkan

    // Pengecualian: Copy
    let i = 1; // i32 adalah Copy
    let i1 = i; // copy
    let i2 = i1; // copy lagi
    println!("i = {}, i1 = {}, i2 = {}", i, i1, i2);

    process_copy(i); // pass by copy
    println!("After process_copy, i = {}", i); // tetap valid
}
