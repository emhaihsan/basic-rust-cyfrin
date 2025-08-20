use std::cmp::PartialOrd;

pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x <= y {
        x
    } else {
        y
    }
}

pub fn zip<T: Copy, U: Copy>(a: Vec<T>, b: Vec<U>) -> Vec<(T, U)> {
    let mut v = vec![];
    let len = std::cmp::min(a.len(), b.len());

    for i in 0..len {
        v.push((a[i], b[i]));
    }

    v
}
