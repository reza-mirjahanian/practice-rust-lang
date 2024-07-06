
### Deadlocks

### 1. Deadlocks

A **deadlock** occurs when two or more threads are blocked forever, each waiting for a resource that the other threads hold. This typically happens with multiple locks when each thread needs to acquire more than one lock and the locking order is not carefully controlled.

#### Example of a Deadlock

Here’s an example of a deadlock in Rust:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));

    let lock1_clone = Arc::clone(&lock1);
    let lock2_clone = Arc::clone(&lock2);

    let thread1 = thread::spawn(move || {
        let _guard1 = lock1_clone.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(50)); // Simulate some work
        let _guard2 = lock2_clone.lock().unwrap();
        println!("Thread 1 acquired both locks");
    });

    let lock1_clone = Arc::clone(&lock1);
    let lock2_clone = Arc::clone(&lock2);

    let thread2 = thread::spawn(move || {
        let _guard2 = lock2_clone.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(50)); // Simulate some work
        let _guard1 = lock1_clone.lock().unwrap();
        println!("Thread 2 acquired both locks");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
```

In this example, `thread1` acquires `lock1` and waits for `lock2`, while `thread2` acquires `lock2` and waits for `lock1`. This results in a deadlock.

### 2. Panics

A **panic** occurs when a thread encounters an unrecoverable error. In Rust, you can handle panics using the `std::panic` module. When a panic occurs, the thread unwinds and cleans up its resources.

#### Example of a Panic

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        panic!("Something went wrong!");
    });

    let result = handle.join();
    match result {
        Ok(_) => println!("Thread finished without error."),
        Err(e) => println!("Thread panicked with error: {:?}", e),
    }
}
```

In this example, the thread panics, and the main thread captures the panic using `handle.join()`, which returns a `Result`.

### 3. Poisoning

When a panic occurs in a thread that holds a lock, the lock is said to be **poisoned**. This means that the data protected by the lock may be in an inconsistent state. Rust’s standard library provides mechanisms to detect and handle poisoned locks.

#### Example of Poisoning

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let lock = Arc::new(Mutex::new(0));

    let lock_clone = Arc::clone(&lock);
    let thread = thread::spawn(move || {
        let mut data = lock_clone.lock().unwrap();
        *data = 1;
        panic!("Thread panicked!");
    });

    let _ = thread.join(); // Wait for the thread to finish

    match lock.lock() {
        Ok(data) => println!("Got the lock, value: {}", *data),
        Err(poisoned) => {
            let data = poisoned.into_inner();
            println!("Lock was poisoned, value: {}", *data);
        }
    }
}
```

In this example, the thread holding the lock panics, causing the lock to be poisoned. The main thread detects the poisoned lock and handles it accordingly.

### Summary

- **Deadlocks** occur when threads are blocked forever, each waiting for a resource held by the other.
- **Panics** occur when a thread encounters an unrecoverable error, causing it to unwind and clean up resources.
- **Poisoning** happens when a thread holding a lock panics, potentially leaving the protected data in an inconsistent state.

