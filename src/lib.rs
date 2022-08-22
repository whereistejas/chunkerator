pub trait Chunkerator: Iterator + Sized {
    fn chunks(self, size: usize) -> Chunks<Self>;
}

impl<I: Iterator> Chunkerator for I {
    fn chunks(self, size: usize) -> Chunks<Self> {
        Chunks {
            inner: self,
            n: size,
        }
    }
}

pub struct Chunks<I: Iterator> {
    inner: I,
    n: usize,
}

#[derive(Debug)]
pub struct Chunk<T: Clone> {
    items: Vec<T>,
}

impl<T: Clone> Chunk<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl<I: Iterator> Iterator for Chunks<I>
where
    I::Item: Clone,
{
    type Item = Chunk<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { inner, n } = self;

        let mut chunk = Chunk::new();

        let mut count = *n;
        loop {
            if let Some(item) = inner.next() {
                chunk.items.push(item);
                count -= 1;
            } else {
                break None;
            }

            if count == 0 {
                break Some(chunk);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Chunkerator;

    #[test]
    fn chunks_of_two() {
        let collection = vec![1, 1, 2, 2, 3, 3, 4, 4, 5];
        let expected = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5]];

        for (i, chunk) in collection.into_iter().chunks(2).enumerate() {
            println!("i: {i}, chunk: {chunk:?}");
            assert_eq!(chunk.items, expected[i])
        }
    }
}
