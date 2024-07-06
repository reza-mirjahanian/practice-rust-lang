# Atomic Counter
In Rust, atomic counters are used for safely managing shared state across multiple threads. Atomic operations ensure that changes to the counter are performed atomically, which prevents race conditions. Rust's standard library provides atomic types in the `std::sync::atomic` module.

### What is an Atomic Counter?

An atomic counter is a counter variable that supports atomic operations like incrementing or decrementing. Atomic operations are indivisible, ensuring thread safety when multiple threads access and modify the counter concurrently.

### Basic Example: Using AtomicUsize

The `AtomicUsize` type from the standard library can be used to create an atomic counter. Here's a simple example demonstrating how to use `AtomicUsize` to safely increment a counter from multiple threads.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    let counter = AtomicUsize::new(0);

    let handles: Vec<_> = (0..10).map(|_| {
        let counter = &counter;
        thread::spawn(move || {
            for _ in 0..100 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.load(Ordering::SeqCst));
}
```

### Explanation

1. **Import the `AtomicUsize` and `Ordering` types**:
   ```rust
   use std::sync::atomic::{AtomicUsize, Ordering};
   ```

2. **Initialize an atomic counter**:
   ```rust
   let counter = AtomicUsize::new(0);
   ```

3. **Spawn multiple threads to increment the counter**:
   ```rust
   let handles: Vec<_> = (0..10).map(|_| {
       let counter = &counter;
       thread::spawn(move || {
           for _ in 0..100 {
               counter.fetch_add(1, Ordering::SeqCst);
           }
       })
   }).collect();
   ```

4. **Wait for all threads to finish**:
   ```rust
   for handle in handles {
       handle.join().unwrap();
   }
   ```

5. **Print the final counter value**:
   ```rust
   println!("Final counter value: {}", counter.load(Ordering::SeqCst));
   ```

### Using Third-Party Libraries: Crossbeam

The `crossbeam` crate provides additional utilities for concurrency in Rust, including support for atomic operations. Here's an example using `crossbeam`'s `AtomicCell`.

First, add `crossbeam` to your `Cargo.toml`:

```toml
[dependencies]
crossbeam = "0.8"
```

Then, use `AtomicCell` to create and manage an atomic counter:

```rust
use crossbeam::atomic::AtomicCell;
use std::thread;

fn main() {
    let counter = AtomicCell::new(0);

    let handles: Vec<_> = (0..10).map(|_| {
        let counter = &counter;
        thread::spawn(move || {
            for _ in 0..100 {
                counter.fetch_add(1);
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.load());
}
```

### Explanation

1. **Import the `AtomicCell` type from `crossbeam`**:
   ```rust
   use crossbeam::atomic::AtomicCell;
   ```

2. **Initialize an atomic counter**:
   ```rust
   let counter = AtomicCell::new(0);
   ```

3. **Spawn multiple threads to increment the counter**:
   ```rust
   let handles: Vec<_> = (0..10).map(|_| {
       let counter = &counter;
       thread::spawn(move || {
           for _ in 0..100 {
               counter.fetch_add(1);
           }
       })
   }).collect();
   ```

4. **Wait for all threads to finish**:
   ```rust
   for handle in handles {
       handle.join().unwrap();
   }
   ```

5. **Print the final counter value**:
   ```rust
   println!("Final counter value: {}", counter.load());
   ```

### Choosing the Right Atomic Type and Ordering

When using atomic operations, you must specify the memory ordering. The common options are:
- `Ordering::Relaxed`: No synchronization or ordering guarantees, fastest but least safe.
- `Ordering::Acquire` / `Ordering::Release`: Ensure synchronization between threads.
- `Ordering::SeqCst`: Sequential consistency, the safest and most restrictive.

### Summary

Atomic counters are a powerful tool for managing shared state in concurrent programming. Rust's standard library provides `AtomicUsize` for basic atomic operations, and third-party libraries like `crossbeam` offer additional functionality with types like `AtomicCell`. By using atomic operations, you can ensure that your counters remain consistent and free from race conditions when accessed from multiple threads.