#![allow(unused)]

// Non-generic version (for illustration of limitation)
// trait List {
//     fn count(&self) -> usize;
//     fn first(&self) -> &u32;
// }

// Generic trait: can work over any element type T
trait List<T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T; // Note: returns a reference to T
}

// Implement List<u32> for a concrete tuple type (u32, bool, char)
impl List<u32> for (u32, bool, char) {
    fn count(&self) -> usize {
        3
    }
    fn first(&self) -> &u32 {
        &self.0
    }
}

// Implement List<T> for Vec<T> (generic over any T)
impl<T> List<T> for Vec<T> {
    fn count(&self) -> usize {
        self.len()
    }
    fn first(&self) -> &T {
        &self[0]
    } // Will panic if empty; kept simple for demo
}

fn main() {
    // Tuple example
    let t = (10u32, false, 'x');
    println!("Tuple count: {}", t.count());
    println!("Tuple first: {:?}", t.first());

    // Vector example (u32)
    let v_u32: Vec<u32> = vec![100, 200, 300];
    println!("Vector (u32) count: {}", v_u32.count());
    println!("Vector (u32) first: {:?}", v_u32.first());

    // Vector example (String)
    let v_string: Vec<String> = vec!["hello".into(), "world".into()];
    println!("Vector (String) count: {}", v_string.count());
    println!("Vector (String) first: {:?}", v_string.first());
}
