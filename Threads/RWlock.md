## RwLock

### Step-by-Step Tutorial: Read-Write Lock with `once_cell` in Rust

In this tutorial, we'll use the `once_cell` crate to create a global read-write lock (`RwLock`). This will allow multiple readers or a single writer to access the shared data. We'll use `once_cell::sync::Lazy` to initialize our `RwLock` in a thread-safe way.

#### Step 1: Add Dependencies

First, add the necessary dependencies to your `Cargo.toml` file:

```toml
[dependencies]
once_cell = "1.16"
```

#### Step 2: Import Libraries

Next, import the required libraries in your Rust program:

```rust
use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::thread;
```

#### Step 3: Define a Global `RwLock` with `Lazy`

Define a global `RwLock` using `Lazy`. This ensures that the `RwLock` is only initialized once in a thread-safe manner:

```rust
static DATA: Lazy<RwLock<Vec<i32>>> = Lazy::new(|| RwLock::new(Vec::new()));
```

#### Step 4: Reading from the `RwLock`

To read data, acquire a read lock (`read`) on the `RwLock`. This allows multiple readers to access the data simultaneously:

```rust
fn read_data() {
    let read_guard = DATA.read().unwrap();
    for val in read_guard.iter() {
        println!("{}", val);
    }
}
```

#### Step 5: Writing to the `RwLock`

To write data, acquire a write lock (`write`) on the `RwLock`. This ensures exclusive access to the data, blocking other readers and writers:

```rust
fn write_data(value: i32) {
    let mut write_guard = DATA.write().unwrap();
    write_guard.push(value);
}
```

#### Step 6: Using the Read and Write Functions in Threads

Spawn multiple threads to read from and write to the `RwLock`:

```rust
fn main() {
    let mut handles = vec![];

    // Writer thread
    let writer_handle = thread::spawn(|| {
        for i in 0..10 {
            write_data(i);
            thread::sleep(std::time::Duration::from_millis(10));
        }
    });
    handles.push(writer_handle);

    // Reader threads
    for _ in 0..3 {
        let handle = thread::spawn(|| {
            for _ in 0..5 {
                read_data();
                thread::sleep(std::time::Duration::from_millis(15));
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Full Code Example

```rust
use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::thread;

// Define a global RwLock with Lazy
static DATA: Lazy<RwLock<Vec<i32>>> = Lazy::new(|| RwLock::new(Vec::new()));

// Function to read data
fn read_data() {
    let read_guard = DATA.read().unwrap();
    for val in read_guard.iter() {
        println!("{}", val);
    }
}

// Function to write data
fn write_data(value: i32) {
    let mut write_guard = DATA.write().unwrap();
    write_guard.push(value);
}

fn main() {
    let mut handles = vec![];

    // Writer thread
    let writer_handle = thread::spawn(|| {
        for i in 0..10 {
            write_data(i);
            thread::sleep(std::time::Duration::from_millis(10));
        }
    });
    handles.push(writer_handle);

    // Reader threads
    for _ in 0..3 {
        let handle = thread::spawn(|| {
            for _ in 0..5 {
                read_data();
                thread::sleep(std::time::Duration::from_millis(15));
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
```

### Explanation

1. **Dependency Management**: Add `once_cell` to `Cargo.toml`.
2. **Global `RwLock`**: Use `Lazy` to define a global `RwLock`, ensuring it's initialized once.
3. **Read and Write Functions**: Implement functions to read from and write to the `RwLock`.
4. **Thread Management**: Create threads for reading and writing, ensuring proper synchronization with the `RwLock`.

