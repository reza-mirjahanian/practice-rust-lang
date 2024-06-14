### Errors

### [Unwinding the Stack or Aborting in Response to a Panic](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic)

By default, when a panic occurs, the program starts  _unwinding_, which means Rust walks back up the stack and cleans up the data from each function it encounters. However, this walking back and cleanup is a lot of work. Rust, therefore, allows you to choose the alternative of immediately  _aborting_, which ends the program without cleaning up.

Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding  `panic = 'abort'`  to the appropriate  `[profile]`  sections in your  _Cargo.toml_  file. For example, if you want to abort on panic in release mode, add this:

`[profile.release]
panic = 'abort'`

---------------
 Rust provides powerful tools for error handling through the `Result` and `Option` types, along with the `panic!` macro. Here are tips, tricks, and best practices for error handling in Rust:

### 1. Understanding `Result` and `Option`

- **`Result<T, E>`**: Used for functions that can return a value of type `T` or an error of type `E`.
- **`Option<T>`**: Used for functions that can return a value of type `T` or `None`.

**Basic Usage**:
```rust
fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(dividend / divisor)
    }
}

match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

**Using `Option`**:
```rust
fn find_item(items: Vec<&str>, target: &str) -> Option<usize> {
    items.iter().position(|&item| item == target)
}

match find_item(vec!["a", "b", "c"], "b") {
    Some(index) => println!("Found at index: {}", index),
    None => println!("Not found"),
}
```

### 2. Propagating Errors with `?`

The `?` operator simplifies error propagation in functions that return `Result` or `Option`.

```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

### 3. Creating Custom Error Types

Creating custom error types can provide more context and make error handling more robust.

**Defining Custom Errors**:
```rust
#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::Parse(error)
    }
}
```

**Using Custom Errors**:
```rust
fn read_and_parse_file(path: &str) -> Result<i32, MyError> {
    let contents = std::fs::read_to_string(path)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}
```

### 4. Handling Multiple Error Types with `Box<dyn Error>`

For functions that may return different types of errors, you can use `Box<dyn std::error::Error>`.

```rust
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    let file_content = std::fs::read_to_string("path/to/file")?;
    let number: i32 = file_content.trim().parse()?;
    println!("Number: {}", number);
    Ok(())
}
```

### 5. Using `thiserror` Crate for Custom Errors

The `thiserror` crate simplifies the creation of custom error types.

**Add `thiserror` to `Cargo.toml`**:
```toml
[dependencies]
thiserror = "1.0"
```

**Define Custom Error Using `thiserror`**:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Parse error")]
    Parse(#[from] std::num::ParseIntError),
}

fn read_and_parse_file(path: &str) -> Result<i32, MyError> {
    let contents = std::fs::read_to_string(path)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}
```

### 6. Converting Between Error Types

Implementing `From` trait allows seamless conversion between error types.

```rust
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(ParseIntError),
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<ParseIntError> for MyError {
    fn from(error: ParseIntError) -> Self {
        MyError::Parse(error)
    }
}

fn read_and_parse_file(path: &str) -> Result<i32, MyError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}
```

### 7. Logging Errors

Use the `log` crate to log errors without stopping the program.

**Add `log` and `env_logger` to `Cargo.toml`**:
```toml
[dependencies]
log = "0.4"
env_logger = "0.9"
```

**Initialize Logger**:
```rust
use log::{error, warn};

fn main() {
    env_logger::init();

    if let Err(e) = run() {
        error!("Application error: {}", e);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Some error-prone code
    warn!("This is a warning message");
    Ok(())
}
```

### 8. `panic!` Macro

Use `panic!` for unrecoverable errors. Prefer `Result` for recoverable errors.

```rust
fn main() {
    let index = 10;
    let array = [1, 2, 3];
    if index >= array.len() {
        panic!("Index out of bounds");
    }
    println!("Element at index {}: {}", index, array[index]);
}
```

### 9. Using `unwrap` and `expect`

Use `unwrap` and `expect` sparingly, as they can cause the program to panic.

**Using `unwrap`**:
```rust
let s = "42";
let number: i32 = s.parse().unwrap();
```

**Using `expect` with a Message**:
```rust
let s = "42";
let number: i32 = s.parse().expect("Failed to parse string to number");
```

### 10. Best Practices Summary

- **Prefer `Result` and `Option`**: Use these types for error handling instead of panicking.
- **Use `?` for Propagation**: Simplify error handling with the `?` operator.
- **Create Custom Error Types**: Provide context and handle multiple error types with custom errors.
- **Log Errors**: Use the `log` crate to log errors and warnings.
- **Use `panic!` Sparingly**: Reserve `panic!` for unrecoverable errors.
- **Handle Errors Explicitly**: Avoid `unwrap` and `expect` unless you're certain the value is present.

