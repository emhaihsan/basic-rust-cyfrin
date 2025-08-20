fn main() {
    // 1) Buat vector kosong + push
    let mut v: Vec<i32> = Vec::new();
    v.push(4);
    v.push(2);
    v.push(3);
    println!("v (push): {:?}", v); // [4, 2, 3]

    // 2) Buat vector dengan vec! macro
    let v2 = vec![1, 2, 3]; // infer: Vec<i32>
    let v3: Vec<i8> = vec![1, 2, 3]; // anotasi tipe eksplisit
    let v4 = vec![1u8, 2, 3]; // suffix menentukan tipe: Vec<u8>
    println!("v2={:?}, v3={:?}, v4={:?}", v2, v3, v4);

    // 3) Vector dengan nilai berulang
    let zeros: Vec<i8> = vec![99i8; 5]; // 5 elemen, semua 99
    println!("zeros: {:?}", zeros);

    // 4) Akses elemen
    let v5 = vec![10, 20, 30];
    println!("v5[1] = {}", v5[1]); // unsafe style: panic jika out-of-bounds
    // println!("{}", v5[1000]); // JANGAN: akan panic

    // 5) Akses aman dengan get() -> Option<&T>
    println!("v5.get(1) = {:?}", v5.get(1)); // Some(&20)
    println!("v5.get(1000) = {:?}", v5.get(1000)); // None

    // 6) Update elemen (butuh mutable)
    let mut v6 = vec![1, 2, 3];
    println!("v6 awal: {:?}", v6);
    v6[0] = 99; // update index 0
    println!("v6 update: {:?}", v6);

    // 7) pop() menghapus elemen terakhir -> Option<T>
    let mut v7 = vec![1, 2, 3];
    println!("v7 awal: {:?}", v7);
    let x1 = v7.pop();
    println!("pop1: {:?}, v7: {:?}", x1, v7); // Some(3), [1, 2]
    let x2 = v7.pop();
    println!("pop2: {:?}, v7: {:?}", x2, v7); // Some(2), [1]
    let x3 = v7.pop();
    println!("pop3: {:?}, v7: {:?}", x3, v7); // Some(1), []
    let x4 = v7.pop();
    println!("pop4: {:?}, v7: {:?}", x4, v7); // None, []

    // 8) Slice dari vector (&[T])
    let v8 = vec![1, 2, 3, 4, 5];
    let s: &[i32] = &v8[1..4]; // ambil 2,3,4 (end index eksklusif)
    println!("slice s: {:?}", s);
}
