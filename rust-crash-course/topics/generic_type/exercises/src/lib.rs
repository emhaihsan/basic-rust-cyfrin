pub fn first<T, U>(t: (T, U)) -> T {
    t.0
}

pub fn last<T, U>(t: (U, T)) -> T {
    t.1
}

#[derive(Debug)]
pub struct Rectangle<T> {
    pub top: T,
    pub left: T,
    pub width: T,
    pub height: T,
}
