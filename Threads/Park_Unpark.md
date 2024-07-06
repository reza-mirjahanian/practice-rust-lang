### Parking Threads in Rust

Parking threads is a concept where a thread is put to sleep until it's explicitly woken up by another thread. This can help with performance and efficiency by reducing the number of active, spinning threads when they don't have work to do. Rust's standard library provides the `std::thread::park` and `std::thread::unpark` functions to handle this.

Here, we'll explore how to use `std::thread::park` and `std::thread::unpark` to park and unpark threads.

### Example: Basic Usage of `park` and `unpark`

In this example, we'll create two threads: one that parks itself and another that wakes it up.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Create a new thread that will park itself
    let parked_thread = thread::spawn(|| {
        println!("Thread is about to park.");
        thread::park();
        println!("Thread has been unparked.");
    });

    // Sleep the main thread for 2 seconds
    thread::sleep(Duration::from_secs(2));

    // Unpark the parked thread
    println!("Unparking the thread.");
    parked_thread.thread().unpark();

    // Wait for the parked thread to finish
    parked_thread.join().unwrap();
}
```

### Explanation

1. **Creating the Thread**: We create a new thread using `thread::spawn`. Inside the thread, we print a message and then park the thread using `thread::park()`.
2. **Sleeping the Main Thread**: We make the main thread sleep for 2 seconds using `thread::sleep`.
3. **Unparking the Thread**: After 2 seconds, we unpark the parked thread using `parked_thread.thread().unpark()`.
4. **Joining the Thread**: Finally, we wait for the parked thread to finish using `join`.

### Example: Using `parking_lot` Crate

The `parking_lot` crate provides more efficient implementations of parking and unparking mechanisms, including `Mutex`, `RwLock`, and other synchronization primitives. Here's an example of using `parking_lot` to park and unpark a thread:

First, add the `parking_lot` crate to your `Cargo.toml`:

```toml
[dependencies]
parking_lot = "0.11"
```

Then, use it in your code:

```rust
use parking_lot::{Mutex, Condvar};
use std::sync::Arc;
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let parked_thread = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock();
        while !*started {
            cvar.wait(&mut started);
        }
        println!("Thread has been unparked.");
    });

    // Sleep the main thread for 2 seconds
    thread::sleep(std::time::Duration::from_secs(2));

    // Unpark the parked thread
    let (lock, cvar) = &*pair;
    {
        let mut started = lock.lock();
        *started = true;
        println!("Unparking the thread.");
    }
    cvar.notify_one();

    // Wait for the parked thread to finish
    parked_thread.join().unwrap();
}
```

### Explanation

1. **Shared State**: We use an `Arc` to share a tuple of `Mutex` and `Condvar` between threads.
2. **Parking the Thread**: The thread locks the mutex and waits on the condition variable until it's signaled.
3. **Unparking the Thread**: The main thread sleeps for 2 seconds, then locks the mutex, changes the shared state, and notifies the condition variable to wake up the parked thread.
4. **Joining the Thread**: Finally, we wait for the parked thread to finish using `join`.

