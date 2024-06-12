


- **Topic**: The `Deref` trait
- **Purpose**: Allows you to treat pointers like regular references.

## Introduction to the Deref Trait

- **Deref Trait**: Customizes the behavior of the dereference operator (`*`).
- **Dereference Operator**: Used to follow a pointer to the actual value.

## Example: Dereference Operator with References

### Initial Example

```rust
let x = 5;
let y = &x;

assert_eq!(x, 5);
assert_eq!(*y, 5);
```
- **Explanation**:
  - `x` stores the integer `5`.
  - `y` is a reference to `x`.
  - `*y` follows the memory address stored in `y` to the value `5`.

### Removing Dereference Operator

```rust
assert_eq!(y, 5);
```
- **Error**: Cannot compare an integer to a reference to an integer.

## Using Smart Pointers Instead of References

### Using Box Smart Pointer

- Replace the reference `&x` with a `Box` smart pointer.

```rust
let y = Box::new(x);
assert_eq!(*y, 5);
```
- **Box**: Points to a value stored in memory and implements the `Deref` trait.

## Creating a Custom Smart Pointer

### Defining MyBox Struct

```rust
struct MyBox<T>(T);
```
- **Tuple Struct**: Holds one generic value.

### Implementing New Function

```rust
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```
- **Difference**: `x` is not stored on the heap.

### Using MyBox in Main Function

```rust
let y = MyBox::new(x);
assert_eq!(*y, 5);
```
- **Error**: Type `MyBox<i32>` cannot be dereferenced because we haven't implemented the `Deref` trait.

## Implementing the Deref Trait

### Bringing Deref Trait into Scope

```rust
use std::ops::Deref;
```

### Implementing Deref for MyBox

```rust
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```
- **Associated Type**: `Target` is equal to the generic `T`.
- **Deref Method**: Returns a reference to the inner data.

## Understanding the Deref Trait Implementation

- **Dereference Operator**: Works for `MyBox` because it implements the `Deref` trait.
- **Ownership System**: `Deref` returns a reference to avoid moving ownership.

## Implicit Deref Coercion with Functions and Methods

### Example: Function Taking a String Slice

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

let m = MyBox::new(String::from("Rust"));
hello(&m);
```
- **Deref Coercion**: Automatically converts a reference to `MyBox` to a `&str`.

### How Deref Coercion Works

```rust
hello(&(*m)[..]);
```
- **Without Coercion**: More verbose and harder to read.

## Using DerefMut Trait

- **Purpose**: Override the dereference operator for mutable references.

## Rules for Deref Coercion

- **Allowed**:
  - Immutable reference to immutable reference.
  - Mutable reference to mutable reference.
  - Mutable reference to immutable reference.
- **Not Allowed**: Immutable reference to mutable reference.



