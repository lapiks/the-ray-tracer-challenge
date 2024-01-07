#[derive(Debug, Clone, PartialEq)]
pub struct Sequence<T: Copy> {
    values: Vec<T>,
    index: usize,
}

impl<T: Copy> Sequence<T> {
    pub fn new(values: Vec<T>) -> Self {
        Self { 
            values,
            index: 0
        }
    }
}

impl<T: Copy> Iterator for Sequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.values.len() {
            self.index = 0
        }
        let result = Some(self.values[self.index]);
        self.index += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_sequence_returns_a_cyclic_sequence_of_numbers() {
        let mut gen = Sequence::new(vec![0.1, 0.5, 1.0]);
        assert_eq!(gen.next().unwrap(), 0.1);
        assert_eq!(gen.next().unwrap(), 0.5);
        assert_eq!(gen.next().unwrap(), 1.0);
        assert_eq!(gen.next().unwrap(), 0.1);
    }
}