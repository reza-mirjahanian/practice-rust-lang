### Capturing References or Moving Ownership in Rust

## Capturing References

When closures capture variables from their environment, they can either borrow the variables (capture by reference) or take ownership of them (capture by move). 

### Capturing by Reference

Capturing by reference borrows the variable from its environment. This allows the closure to use the variable without taking ownership.

```rust
fn main() {
    let x = 10;
    let print_x = || println!("{}", x);
    print_x();
    println!("{}", x); // x can still be used here
}
```

In this example, `print_x` borrows `x` by reference. Thus, `x` is still accessible after the closure is executed.

### Mutable References

Closures can also capture mutable references, allowing them to modify the borrowed variable.

```rust
fn main() {
    let mut x = 10;
    {
        let mut modify_x = || x += 1;
        modify_x();
    }
    println!("{}", x); // x has been modified
}
```

Here, `modify_x` borrows `x` mutably, allowing it to modify `x`.

## Moving Ownership

Capturing by move takes ownership of the variable from its environment, which can be useful for ensuring the closure owns the data it operates on.

### Basic Example

Using the `move` keyword forces the closure to take ownership of the captured variable.

```rust
fn main() {
    let x = vec![1, 2, 3];
    let owns_x = move || println!("{:?}", x);
    owns_x();
    // println!("{:?}", x); // This would cause an error, as x has been moved
}
```

In this example, `owns_x` takes ownership of `x`, so `x` cannot be used after the closure.

### Using Moved Values in Threads

Moving ownership is often used when spawning threads to avoid borrowing issues.

```rust
use std::thread;

fn main() {
    let x = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("{:?}", x);
    });
    handle.join().unwrap();
    // println!("{:?}", x); // This would cause an error, as x has been moved
}
```

Here, `move` ensures the thread closure takes ownership of `x`, making it safe to use in the new thread.

## Using Third-Party Libraries

### `crossbeam` Crate

The `crossbeam` crate provides powerful concurrency primitives, allowing easier and safer manipulation of ownership in concurrent contexts.

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
crossbeam = "0.8"
```

#### Example: Using `crossbeam` for Scoped Threads

`crossbeam` allows for scoped threads, which can safely borrow variables.

```rust
use crossbeam::scope;

fn main() {
    let x = vec![1, 2, 3];
    scope(|s| {
        s.spawn(|_| {
            println!("{:?}", x); // Borrowed from main thread
        });
    }).unwrap();
    println!("{:?}", x); // x is still accessible here
}
```

In this example, `crossbeam::scope` ensures that the thread borrows `x` rather than taking ownership, allowing `x` to remain accessible in the main thread.

### Capturing References or Moving Ownership with `rayon`

The `rayon` crate simplifies parallel iteration, often requiring careful handling of ownership and borrowing.

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
rayon = "1.5"
```

#### Example: Using `rayon` for Parallel Iteration

```rust
use rayon::prelude::*;

fn main() {
    let numbers: Vec<i32> = (1..=10).collect();
    let sum: i32 = numbers.par_iter().sum();
    println!("{}", sum); // Safe parallel iteration
}
```

In this example, `par_iter` borrows the elements of `numbers` to perform parallel iteration safely.

-----------------------
Closures will automatically implement one, two, or all three of these  `Fn`  traits, in an additive fashion, depending on how the closure’s body handles the values:

1.  `FnOnce`  applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement  `FnOnce`  and none of the other  `Fn`  traits, because it can only be called once.
2.  `FnMut`  applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
3.  `Fn`  applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

Let’s look at the definition of the  `unwrap_or_else`  method on  `Option<T>`  that we used in Listing 13-1:

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```