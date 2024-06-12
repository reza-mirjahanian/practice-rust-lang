


## Error Handling in Rust



### The Panic Macro
First, let's talk about the `panic` macro. If your program fails in a way that's unrecoverable or you can't handle the error gracefully, you can call the `panic` macro, which will:
- Immediately quit your program
- Print out an error message

#### Example: Basic Panic
Here's a simple example:
```rust
fn main() {
    panic!("Crash and burn");
}
```
Running this program will result in:
- An error message
- A backtrace indicating where the error occurred

### Backtraces
A backtrace will list all the functions called leading up to the error. For example:
```rust
fn main() {
    a();
}

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!");
    }
}
```
Running this program with a backtrace will show:
- The chain of function calls leading to the error

### Recoverable Errors with Result Enum
Recoverable errors are those you can handle gracefully and don't want to crash your program. For these cases, Rust provides the `Result` enum.

#### The Result Enum
- Similar to the `Option` enum
- Represents either success or failure
- Variants:
  - `Ok` for success
  - `Err` for failure

#### Example: Handling File Opening
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

### Improving Error Handling
Instead of panicking, you can handle specific errors, like creating a new file if it doesn't exist:
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

### Using Unwrap and Expect
For brevity, especially in example or prototype code, you can use:
- `unwrap` to get the value inside an `Ok` variant or panic on `Err`
- `expect` to provide a custom error message

```rust
let f = File::open("hello.txt").expect("Failed to open hello.txt");
```

### Error Propagation
Instead of handling errors within a function, you can propagate them back to the caller:
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

### The Question Mark Operator
The `?` operator simplifies error propagation:
```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

### Using Result in Main
The `main` function can return a `Result`:
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
```

### When to Use Panic
- Use the `Result` enum and error propagation as the default.
- Use `panic` in exceptional circumstances where recovery isn't possible.

### Creating Custom Types for Validation
To ensure data validation, create custom types:
```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

## Conclusion

- Error handling
- The `panic` macro
- The `Result` enum
- Using `unwrap` and `expect`
- Using the `?` operator and more

