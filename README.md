# RLE Merge RS

A Rust implementation of a Run-Length Encoding (RLE) OR iterator.

This project provides a generic iterator `RleOrIterator` that takes two RLE streams (represented as iterators of `Run` structs) and produces a new RLE stream where the values are the result of a bitwise OR operation.

## Usage

To run the example, use `cargo`:

```bash
cargo run
```

This will execute the `main` function in `src/main.rs`, which demonstrates the usage of `RleOrIterator` with two sample RLE streams.

### Example Output

```
Val: false, Len: 1
Val: true, Len: 2
Val: true, Len: 2
```

## Implementation

The core components are:

*   **`struct Run`**: Represents a segment in a Run-Length Encoded stream, containing a `val` (`bool`) and a `len` (`usize`).
*   **`struct RleOrIterator<T>`**: A generic iterator that wraps two base iterators (`T`).
*   **`impl Iterator for RleOrIterator<T>`**: Implements the `next()` method which contains the logic for merging the two RLE streams. It advances both streams, finds the minimum overlapping length, performs an OR operation on the values, and yields a new `Run`.

## License

This project is not yet licensed.
