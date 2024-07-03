
In Rust, `iter` and `into_iter` are methods that create iterators, but they differ in how they treat the ownership of the collection they are called on. Here’s an explanation of each, along with some examples to illustrate their usage:

### `iter`

- **Purpose**: Creates an iterator that borrows each element of the collection (returns immutable references).
- **Ownership**: Does not take ownership of the collection; it borrows it.
- **Use Case**: When you want to iterate over a collection without consuming it.

**Example:**

```rust
fn main() {
    let vec = vec![1, 2, 3];
    let iter = vec.iter();
    
    for val in iter {
        println!("{}", val);
    }

    // vec can still be used here because iter only borrows it
    println!("Vector is still available: {:?}", vec);
}
```

In this example, `vec.iter()` returns an iterator that yields immutable references to each element in `vec`.

### `into_iter`

- **Purpose**: Creates an iterator that takes ownership of the collection (consumes the collection).
- **Ownership**: Takes ownership of the collection, consuming it so it can no longer be used after creating the iterator.
- **Use Case**: When you want to iterate over a collection and consume it in the process.

**Example:**

```rust
fn main() {
    let vec = vec![1, 2, 3];
    let into_iter = vec.into_iter();
    
    for val in into_iter {
        println!("{}", val);
    }

    // vec cannot be used here because into_iter takes ownership of it
    // println!("Vector is still available: {:?}", vec); // This would cause a compile error
}
```

In this example, `vec.into_iter()` returns an iterator that yields values by taking ownership of each element in `vec`. The original `vec` cannot be used after this point.

### Comparison

Here’s a comparison of `iter` and `into_iter` in a table format:

| Method    | Ownership        | Yields           | Can collection be used afterward? |
|-----------|------------------|------------------|-----------------------------------|
| `iter`    | Borrows          | Immutable refs   | Yes                               |
| `into_iter` | Takes ownership | Values           | No                                |

### Mutable Iteration: `iter_mut`

For completeness, it's also useful to mention `iter_mut`, which is another way to create an iterator:

- **Purpose**: Creates an iterator that yields mutable references to each element.
- **Ownership**: Borrows the collection mutably.
- **Use Case**: When you need to iterate over a collection and modify each element.

**Example:**

```rust
fn main() {
    let mut vec = vec![1, 2, 3];
    let iter_mut = vec.iter_mut();
    
    for val in iter_mut {
        *val += 1;
    }

    // vec can still be used here and its elements are modified
    println!("Modified vector: {:?}", vec);
}
```

In this example, `vec.iter_mut()` returns an iterator that yields mutable references to each element in `vec`, allowing the elements to be modified in place.

### Summary

- **`iter`**: Borrows the collection, yields immutable references, does not consume the collection.
- **`into_iter`**: Takes ownership of the collection, yields values, consumes the collection.
- **`iter_mut`**: Borrows the collection mutably, yields mutable references, does not consume the collection.

