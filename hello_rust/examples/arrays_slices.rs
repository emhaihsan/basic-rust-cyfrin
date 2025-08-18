fn main() {
    // Arrays
    let mut arr: [u32; 5] = [10, 20, 30, 40, 50];
    println!("arr len = {}, arr = {:?}", arr.len(), arr);
    arr[2] = 99;
    println!("arr after write = {:?}", arr);

    let zeros: [u32; 4] = [0; 4];
    println!("zeros = {:?}", zeros);

    // Slices (immut)
    let first3: &[u32] = &arr[..3];
    let last2: &[u32] = &arr[3..];
    let mid2: &[u32] = &arr[1..3];
    println!("first3 = {:?}", first3);
    println!("last2  = {:?}", last2);
    println!("mid2   = {:?}", mid2);

    // Mutable slice
    let s: &mut [u32] = &mut arr[1..4];
    s[0] += 1;
    s[1] += 1;
    s[2] += 1;
    println!("arr after mut slice = {:?}", arr);

    // Aman dengan get
    println!("arr.get(10) = {:?}", arr.get(10)); // None (tidak panic)
}
