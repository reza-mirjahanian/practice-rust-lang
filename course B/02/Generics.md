Generics

### 1. Basics of Generics

**Defining Generic Functions**:
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

**Defining Generic Structs**:
```rust
struct Point<T> {
    x: T,
    y: T,
}

let integer_point = Point { x: 5, y: 10 };
let float_point = Point { x: 1.0, y: 4.0 };
```

**Defining Generic Enums**:
```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 2. Trait Bounds

Use trait bounds to specify what capabilities a generic type must have.

**Using Trait Bounds**:
```rust
fn print_largest<T: std::fmt::Display + PartialOrd>(list: &[T]) {
    let largest = largest(list);
    println!("The largest number is {}", largest);
}
```

### 3. Implementing Methods for Generic Types

You can implement methods for structs and enums with generic types.

**Methods on Generic Structs**:
```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

### 4. Combining Generic Types and Traits

**Defining and Using Traits with Generics**:
```rust
trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Point<i32> {
    fn summarize(&self) -> String {
        format!("Point at ({}, {})", self.x, self.y)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

**Using Trait Bounds with `where` Clause**:
```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // Function body
}
```

### 5. Generic Lifetimes

Use lifetimes to ensure references are valid.

**Basic Lifetime Annotation**:
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### 6. Using `PhantomData` for Advanced Generics

`PhantomData` is useful when you need to indicate that a generic type parameter is used, but it's not actually stored in the struct.

**Using `PhantomData`**:
```rust
use std::marker::PhantomData;

struct MyStruct<T> {
    value: i32,
    _marker: PhantomData<T>,
}

impl<T> MyStruct<T> {
    fn new(value: i32) -> MyStruct<T> {
        MyStruct {
            value,
            _marker: PhantomData,
        }
    }
}
```

### 7. Avoiding Code Bloat

Generics can lead to code bloat (larger binaries) because Rust generates code for each type instantiation. Use generics judiciously and prefer concrete types where appropriate.

**Use Generic Sparingly**:
```rust
fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// Prefer using concrete types if the function is usually used with specific types
fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}
```

### 8. Best Practices Summary

- **Use Generics for Reusability**: Write generic functions, structs, and enums to handle multiple types while maintaining type safety.
- **Specify Trait Bounds**: Ensure your generics fulfill certain requirements using trait bounds.
- **Combine Generics with Traits**: Implement traits for generic types and use them with trait bounds to enforce type constraints.
- **Use Lifetimes**: Ensure references are valid and prevent dangling references by using lifetimes.
- **Employ `PhantomData`**: Use `PhantomData` for advanced generic programming to mark unused type parameters.
- **Avoid Overusing Generics**: Be mindful of code bloat and prefer concrete types when generics are not necessary.

### 9. Additional Tips

- **Use Type Aliases for Readability**: Type aliases can simplify complex generic type signatures.
```rust
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
```

- **Leverage Default Type Parameters**: Provide default types for generic parameters to reduce the need for specifying them explicitly.
```rust
struct MyStruct<T = i32> {
    value: T,
}
```

- **Document Your Generic Code**: Clearly document the purpose and constraints of your generic types and functions to make your code more maintainable.

By following these tips and best practices, you can effectively use generic types in Rust to write flexible, reusable, and type-safe code.


3.  **Use lifetime parameters for references**: Lifetime parameters allow you to specify the lifetime of references in generic types. This can help to ensure memory safety and prevent data races.

```rust
struct Ref<'a, T: 'a>(&'a T);

impl<'a, T> Ref<'a, T> {
    fn new(value: &'a T) -> Self {
        Ref(value)
    }
}
```