
In Rust, the `println!` macro is commonly used to print formatted strings to the console. Here are various ways to use `println!` with different formatting options and scenarios:

### Basic Usage

**Print a simple string:**

```rust
fn main() {
    println!("Hello, world!");
}
```

**Print a string with a newline character:**

```rust
fn main() {
    println!("Hello,\nworld!");
}
```

### Printing Variables

**Print a single variable:**

```rust
fn main() {
    let name = "Alice";
    println!("Hello, {}!", name);
}
```

**Print multiple variables:**

```rust
fn main() {
    let name = "Alice";
    let age = 30;
    println!("Name: {}, Age: {}", name, age);
}
```

### Formatting Numbers

**Print integers and floats:**

```rust
fn main() {
    let x = 42;
    let y = 3.14;
    println!("x = {}, y = {}", x, y);
}
```

**Print numbers with specific formatting:**

```rust
fn main() {
    let x = 42;
    let y = 3.14159;
    println!("x = {:08}, y = {:.2}", x, y); // x padded with zeros, y with 2 decimal places
}
```

### Special Formatting

**Print in hexadecimal, binary, or octal:**

```rust
fn main() {
    let x = 255;
    println!("Hex: {:x}, Binary: {:b}, Octal: {:o}", x, x, x);
}
```

**Print a debug representation of a value:**

```rust
fn main() {
    let tuple = (1, "hello", 4.5);
    println!("Debug: {:?}", tuple);
}
```

### Using Named Arguments

**Print using named arguments:**

```rust
fn main() {
    println!("{greeting}, {name}!", greeting = "Hello", name = "Bob");
}
```

### Escape Sequences

**Print with escape sequences:**

```rust
fn main() {
    println!("This is a quote: \"");
    println!("This is a backslash: \\");
}
```

### Using Format Specifiers

**Align text:**

```rust
fn main() {
    println!("{:<10} | {:>10}", "left", "right"); // Left-align and right-align
}
```

**Print with precision and width:**

```rust
fn main() {
    let pi = 3.141592;
    println!("{:.3}", pi); // Print with 3 decimal places
    println!("{:10.3}", pi); // Print with 3 decimal places and width of 10
}
```

### Conditional Formatting

**Conditional formatting using `if`:**

```rust
fn main() {
    let condition = true;
    println!("Condition is: {}", if condition { "true" } else { "false" });
}
```

### Printing in Loops

**Print in a loop:**

```rust
fn main() {
    for i in 0..5 {
        println!("Number: {}", i);
    }
}
```

### Printing Custom Types

**Implement `Debug` for custom types:**

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Point: {:?}", p);
}
```

**Implement `Display` for custom types:**

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Point: {}", p);
}
```

These examples showcase the flexibility and power of the `println!` macro in Rust for various formatting and printing needs.