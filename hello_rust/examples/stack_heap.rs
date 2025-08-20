fn main() {
    // STACK: ukuran diketahui saat compile
    let x: i32 = 1; // i32 disimpan di stack
    let arr: [i32; 10] = [1; 10]; // seluruh elemen array ada di stack

    println!("x = {}", x);
    println!("arr[0] = {}, len = {}", arr[0], arr.len());

    // HEAP: ukuran dinamis
    // String: metadata (ptr, len, cap) di stack; teks di heap
    let mut s: String = "hello".to_string();
    s += " world";
    println!("s = {}", s);

    // Vec<T>: metadata di stack; elemen di heap
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("v = {:?}, len = {}, cap = {}", v, v.len(), v.capacity());

    // Box<T>: memaksa data ke heap
    let boxed_num: Box<i32> = Box::new(42);
    println!("boxed_num = {}", boxed_num);
}
