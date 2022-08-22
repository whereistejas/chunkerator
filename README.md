## Chunkerator.
A simple trait to convert an iterator into something that returns chunks.

```rust
use chunkerator::Chunkerator;

fn main() {
    let collection = vec![1, 1, 2, 2, 3, 3, 4, 4, 5];
    let expected = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5]];

    for (i, chunk) in collection.into_iter().chunks(2).enumerate() {
        println!("i: {i}, chunk: {chunk:?}");
        assert_eq!(chunk.items(), &expected[i]);
    }
}
```