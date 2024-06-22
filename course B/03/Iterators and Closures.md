

## Iterators in Rust

### Using Standard Library Iterators

#### Basic Iteration
Iterators in Rust are lazy and perform operations only when consumed. Here's a basic example:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    for num in numbers.iter() {
        println!("{}", num);
    }
}
```

#### Using Iterator Methods
Iterator methods such as `map`, `filter`, and `collect` provide a functional approach to process collections:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squares: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("{:?}", squares);
}
```

Iterators in Rust are lazy, meaning they don't compute the items they yield until they are consumed. `collect()` is one of the methods to consume an iterator, collecting the resulting items into a collection, such as a vector, hash map, or another collection type.
Without `collect()`, converting an iterator into a collection would require manually iterating over the iterator and inserting each item into the collection. `collect()` encapsulates this pattern, providing a concise and more ergonomic way to achieve this common task.

-   **Converting an Iterator to a Vector**

  


```rust
let my_iter = vec![1, 2, 3].into_iter();
let my_vec: Vec<_> = my_iter.collect(); // Converts iterator to vector

```

  

-   **Collecting Into a HashMap**



```rust
let pairs = vec![("one", 1), ("two", 2), ("three", 3)];
let map: HashMap<_, _> = pairs.into_iter().collect(); // Collects into a HashMap

```



-   **Filtering and Collecting**

 

```rust
let numbers = vec![1, 2, 3, 4, 5];
let even_numbers: Vec<_> = numbers.into_iter()
    .filter(|&x| x % 2 == 0)
    .collect(); // Filters and collects even numbers

```

#### Why do I get double references when filtering a Vec?

Getting double references when filtering a `Vec` in Rust often occurs due to how the `filter` method operates on iterators. In Rust, iterators are designed with zero-cost abstraction in mind, meaning they aim to impose no additional runtime overhead. When you call `.iter()` on a `Vec`, you get an iterator that yields references to the elements of the vector, not the elements themselves. This behavior is intentional, allowing iteration without taking ownership of the vector or its items.
-   `numbers.iter()`  yields items of type  `&i32`.
-   The closure argument  `|&&x|`  uses pattern matching to automatically dereference the  `&i32`  into  `i32`  for easy comparison.
----------------------------------

#### Chaining Iterator Methods
You can chain multiple iterator methods to build complex operations:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let evens_squared: Vec<i32> = numbers.iter()
                                         .filter(|&&x| x % 2 == 0)
                                         .map(|&x| x * x)
                                         .collect();
    println!("{:?}", evens_squared);
}
```

### Custom Iterators

You can create custom iterators by implementing the `Iterator` trait for your types:

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new();
    for num in counter {
        println!("{}", num);
    }
}
```

### Using Third-Party Libraries

#### `itertools` Crate
The `itertools` crate provides additional iterator functionalities:

Add the dependency to your `Cargo.toml`:
```toml
[dependencies]
itertools = "0.10"
```

Example usage:

```rust
use itertools::Itertools;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let result: Vec<_> = numbers.iter()
                                .filter(|&&x| x % 2 == 0)
                                .map(|&x| x * x)
                                .collect();
    println!("{:?}", result);

    // Using join method from itertools
    let joined = numbers.iter().join(", ");
    println!("{}", joined);
}
```

## Closures in Rust

### Basic Closure

Closures are anonymous functions you can store in variables or pass as arguments to other functions:

```rust
fn main() {
    let add_one = |x: i32| x + 1;
    println!("{}", add_one(5));
}
```

### Capturing Environment Variables

Closures can capture variables from their environment:

```rust
fn main() {
    let x = 10;
    let print_x = || println!("{}", x);
    print_x();
}
```

### Specifying Types for Clarity

Although Rust can infer types, specifying them can enhance clarity:

```rust
fn main() {
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    println!("{}", multiply(2, 3));
}
```

### Using `move` Keyword

The `move` keyword forces the closure to take ownership of the captured variables:

```rust
fn main() {
    let x = vec![1, 2, 3];
    let owns_x = move || println!("{:?}", x);
    owns_x();
    // println!("{:?}", x); // This would cause an error
}
```

### Passing Closures to Functions

You can pass closures as arguments to functions to create higher-order functions:

```rust
fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    let double = |x| x * 2;
    println!("{}", apply_to_3(double)); // Outputs 6
}
```

### Returning Closures from Functions

You can return closures from functions using `impl Fn`:

```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let closure = returns_closure();
    println!("{}", closure(2)); // Outputs 3
}
```

### Using Third-Party Libraries

#### `rayon` Crate for Parallel Iterators

The `rayon` crate allows you to perform parallel iterations:

Add the dependency to your `Cargo.toml`:
```toml
[dependencies]
rayon = "1.5"
```

Example usage:

```rust
use rayon::prelude::*;

fn main() {
    let numbers: Vec<i32> = (1..=10).collect();
    let sum: i32 = numbers.par_iter().sum();
    println!("{}", sum);
}
```

