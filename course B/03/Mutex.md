## Using Threads in Rust

Rust provides powerful tools for concurrent programming, with threads being a primary way to achieve this. This guide will cover the basics of using threads in Rust, including simple examples and the use of both standard and third-party libraries.

### Creating and Using Threads

Rust's standard library provides the `thread` module, which allows you to spawn new threads.

#### Example: Spawning a Simple Thread

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from the spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hello from the main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

In this example:
- A new thread is spawned using `thread::spawn`.
- The main thread and the spawned thread both print messages.
- `handle.join().unwrap()` ensures the main thread waits for the spawned thread to finish.

### Passing Data to Threads

You can pass data to threads by moving ownership using the `move` keyword.

#### Example: Passing Data to a Thread

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Data from the thread: {:?}", data);
    });

    handle.join().unwrap();
}
```

In this example:
- The `move` keyword transfers ownership of `data` to the spawned thread.
- The spawned thread then has access to `data`.

### Using Channels for Thread Communication

Rust provides channels for message passing between threads, which can be found in the `std::sync::mpsc` module.

#### Example: Using Channels

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello from the spawned thread");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
```

In this example:
- A channel is created using `mpsc::channel`.
- The transmitter (`tx`) is moved to the spawned thread.
- The receiver (`rx`) is used in the main thread to receive the message.

### Using Third-Party Libraries: Rayon

Rayon is a popular library for data parallelism in Rust. It simplifies parallel processing using threads.

#### Example: Parallel Iteration with Rayon

First, add Rayon to your `Cargo.toml`:

```toml
[dependencies]
rayon = "1.5"
```

Then, use it in your code:

```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    let result: Vec<_> = data.par_iter().map(|&x| x * 2).collect();

    println!("Result: {:?}", result);
}
```

In this example:
- `par_iter()` is used instead of `iter()` to enable parallel iteration.
- The `map` operation is performed in parallel, doubling each element in the vector.

### [Similarities Between  `RefCell<T>`/`Rc<T>`  and  `Mutex<T>`/`Arc<T>`](https://doc.rust-lang.org/book/ch16-03-shared-state.html#similarities-between-refcelltrct-and-mutextarct)

You might have noticed that  `counter`  is immutable but we could get a mutable reference to the value inside it; this means  `Mutex<T>`  provides interior mutability, as the  `Cell`  family does. In the same way we used  `RefCell<T>`  in Chapter 15 to allow us to mutate contents inside an  `Rc<T>`, we use  `Mutex<T>`  to mutate contents inside an  `Arc<T>`.

Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use  `Mutex<T>`. Recall in Chapter 15 that using  `Rc<T>`  came with the risk of creating reference cycles, where two  `Rc<T>`  values refer to each other, causing memory leaks. Similarly,  `Mutex<T>`  comes with the risk of creating  _deadlocks_. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever. If you’re interested in deadlocks, try creating a Rust program that has a deadlock; then research deadlock mitigation strategies for mutexes in any language and have a go at implementing them in Rust. The standard library API documentation for  `Mutex<T>`  and  `MutexGuard`  offers useful information.



## Mutex in Rust

In Rust, a `Mutex` (short for "mutual exclusion") is a synchronization primitive that provides thread-safe shared access to data. A `Mutex` allows only one thread to access the data it protects at a time, ensuring safe concurrent access. This guide covers the basics of using `Mutex` in Rust with simple examples, including the use of both standard and third-party libraries.

### Basics of `Mutex`

The `std::sync::Mutex` type ensures that only one thread can access the data inside the `Mutex` at any time. To use a `Mutex`, you need to lock it, which gives you a `MutexGuard` that dereferences to the underlying data.

#### Example: Basic Usage of `Mutex`

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // MutexGuard is dropped here

    println!("m = {:?}", m);
}
```

In this example:
- A `Mutex` is created to protect an integer value.
- The `lock` method is called to access the data, returning a `MutexGuard`.
- The `MutexGuard` is used to modify the data.
- When the `MutexGuard` goes out of scope, it is automatically released.

### Using `Mutex` with Threads

`Mutex` is commonly used with threads to safely share data among them.

#### Example: Sharing Data Between Threads

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

In this example:
- An `Arc` (atomic reference count) is used to share ownership of the `Mutex` among multiple threads.
- Each thread increments the counter inside the `Mutex`.
- After all threads complete, the final value of the counter is printed.

### Handling Poisoned `Mutex`

If a thread panics while holding a `Mutex`, the `Mutex` becomes poisoned. Attempting to acquire a poisoned `Mutex` returns a `PoisonError`.

#### Example: Handling a Poisoned `Mutex`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut num = data_clone.lock().unwrap();
        *num = 10;
        panic!("Oops!");
    });

    let result = handle.join();
    match result {
        Ok(_) => println!("Thread completed successfully"),
        Err(_) => println!("Thread panicked"),
    }

    match data.lock() {
        Ok(num) => println!("Data is {}", *num),
        Err(poisoned) => println!("Data is poisoned, recovered value: {}", *poisoned.into_inner()),
    }
}
```

In this example:
- A thread panics after modifying the `Mutex`-protected data.
- The `Mutex` becomes poisoned.
- The main thread attempts to lock the `Mutex`, and handles the `PoisonError`.

### Using Third-Party Libraries: `parking_lot`

The `parking_lot` crate provides a more efficient `Mutex` implementation than the standard library.

#### Example: Using `parking_lot`

First, add `parking_lot` to your `Cargo.toml`:

```toml
[dependencies]
parking_lot = "0.11"
```

Then, use it in your code:

```rust
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock());
}
```

In this example:
- The `parking_lot::Mutex` is used in place of `std::sync::Mutex`.
- The code structure remains similar, but the `parking_lot` `Mutex` is generally faster and more efficient.

