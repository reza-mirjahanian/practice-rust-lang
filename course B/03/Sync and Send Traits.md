Concurrency in Rust can be managed using various traits and libraries. Two important traits for concurrency are `Sync` and `Send`.

1. **Send**: This trait indicates that ownership of the type implementing `Send` can be transferred across thread boundaries.
2. **Sync**: This trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads.

To understand how these traits work, we can look at some simple examples using both standard and third-party libraries.

### Using Standard Library

#### Example with `std::thread`

First, let's look at how we can use threads from the standard library to achieve concurrency with types that implement the `Send` and `Sync` traits.

```rust
use std::thread;

// A type that is Send and Sync
#[derive(Debug)]
struct Data {
    value: i32,
}

fn main() {
    // Create an instance of Data
    let data = Data { value: 42 };

    // Spawn a new thread and move the data into it
    let handle = thread::spawn(move || {
        println!("Data from thread: {:?}", data);
    });

    // Wait for the thread to finish
    handle.join().unwrap();
}
```

In this example, the `Data` struct implements both `Send` and `Sync` traits automatically because it only contains an `i32`, which is `Send` and `Sync`. The `thread::spawn` function transfers ownership of `data` to the new thread.

### Using Third-Party Library

#### Example with `tokio`

`tokio` is a runtime for writing reliable asynchronous applications with Rust. It is often used for network services.

```rust
use tokio::sync::Mutex;
use std::sync::Arc;

// A type that is Send and Sync
#[derive(Debug)]
struct SharedData {
    value: i32,
}

#[tokio::main]
async fn main() {
    // Create an instance of SharedData wrapped in an Arc and Mutex
    let data = Arc::new(Mutex::new(SharedData { value: 42 }));

    // Clone the Arc to share the data with the new task
    let data_clone = data.clone();

    // Spawn a new asynchronous task
    tokio::spawn(async move {
        let mut locked_data = data_clone.lock().await;
        locked_data.value += 1;
        println!("Data from async task: {:?}", *locked_data);
    }).await.unwrap();

    // Lock and print the data from the main task
    let locked_data = data.lock().await;
    println!("Data from main task: {:?}", *locked_data);
}
```

In this example, we use `Arc` to create a reference-counted pointer to the `SharedData` instance, and `Mutex` to ensure safe concurrent access. The `tokio::spawn` function creates a new asynchronous task, and the `await` keyword is used to wait for the task to complete.

### Explanation

1. **Standard Library Example**: 
   - `thread::spawn` is used to create a new thread.
   - The `move` keyword transfers ownership of `data` to the new thread.
   - The `join` method waits for the thread to complete execution.

2. **Third-Party Library Example**:
   - `tokio::sync::Mutex` is used to safely share and mutate data across tasks.
   - `Arc` is used to create a thread-safe reference-counted pointer.
   - `tokio::spawn` creates an asynchronous task.
   - The `lock` method of `Mutex` is used with `await` to safely access and modify the data.

Both examples demonstrate the use of the `Send` and `Sync` traits to ensure safe concurrent access to data, either through threads or asynchronous tasks.