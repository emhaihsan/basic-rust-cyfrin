pub trait Iterator<T> {
    fn next(&mut self) -> Option<&T>;
}

pub struct TupleIter<T> {
    pub tuple: (T, T, T),
    pub next: usize,
}

pub struct VecIter<T> {
    pub vec: Vec<T>,
    pub next: usize,
}

impl<T> Iterator<T> for TupleIter<T> {
    fn next(&mut self) -> Option<&T> {
        let res = match self.next {
            0 => Some(&self.tuple.0),
            1 => Some(&self.tuple.1),
            2 => Some(&self.tuple.2),
            _ => None,
        };
        self.next += 1;
        res
    }
}

impl<T> Iterator<T> for VecIter<T> {
    fn next(&mut self) -> Option<&T> {
        if self.next < self.vec.len() {
            let value = &self.vec[self.next];
            self.next += 1;
            Some(value)
        } else {
            None
        }
    }
}
