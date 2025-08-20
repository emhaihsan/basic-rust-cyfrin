#![allow(unused)]

fn main() {
    // 1) Pitfall: into_iter() (default for-loop) consumes the Vec
    let vals_move = vec![1, 2, 3, 4, 5];
    println!("-- into_iter() (implicit in `for v in vals_move`) --");
    for v in vals_move {
        // moves vals_move into the iterator
        print!("{v} ");
    }
    println!("");
    // println!("{:?}", vals_move); // Uncommenting would fail: E0382 use of moved value

    // 2) iter(): borrow immutably, can iterate multiple times
    let vals_borrow = vec![10, 20, 30];
    println!("-- iter() pass 1 --");
    for v_ref in vals_borrow.iter() {
        // &i32
        print!("{v_ref} ");
    }
    println!("");

    println!("-- iter() pass 2 --");
    for v_ref in vals_borrow.iter() {
        // can borrow again
        print!("{v_ref} ");
    }
    println!("");
    println!("still have vals_borrow: {:?}", vals_borrow);

    // 3) iter_mut(): borrow mutably, modify in place
    let mut vals_mut = vec![2, 4, 6];
    println!("-- iter_mut() (doubling) --");
    for v_mut in vals_mut.iter_mut() {
        // &mut i32
        *v_mut *= 2;
    }
    println!("modified vals_mut: {:?}", vals_mut); // [4, 8, 12]
}
