trait MyIterator<'a, T> {
    fn has_next(&self) -> bool;
    fn next(&mut self) -> T;
}

struct MyVecIterator<T: Copy> {
    vec: Vec<T>,
    index: usize,
    length: usize,
}

impl<'a, T: Copy> MyVecIterator<T> {
    fn new() -> MyVecIterator<T> {
        MyVecIterator{vec: vec![], index: 0, length: 0}
    }

    fn add(&mut self, value: T) {
        self.vec.push(value);
        self.length += 1;
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl<'a, T: Copy> MyIterator<'a, T> for MyVecIterator<T> {
    fn has_next(&self) -> bool {
        self.index < self.vec.len()
    }

    fn next(&mut self) -> T {
        let value = self.vec[self.index];
        self.index += 1;
        value
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_iterate_empty() {
        let iter = MyVecIterator::<i32>::new();
        
        assert_eq!(iter.is_empty(), true);
        assert_eq!(iter.has_next(), false);
    }

    #[test]
    fn test_iterate_non_empty() {
        let value = 21;
        let mut iter = MyVecIterator::<i32>::new();
        iter.add(value);
        assert_eq!(iter.is_empty(), false);
        assert_eq!(iter.has_next(), true);
        assert_eq!(iter.next(), value);
    }

}