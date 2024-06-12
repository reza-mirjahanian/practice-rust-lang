


# Lifetimes in Rust

## Dangling References

- What is a dangling reference?
  - A reference that points to invalid data
  - Rust does not allow dangling references

### Example 1
```rust
fn main() {
    let r; // 'a
    {
        let x = 5; // 'b
        r = &x; // 'a
    }
    println!("{}", r); // Error: x does not live long enough
}
```
- `r` has a lifetime `'a` that lasts until the end of `main()`
- `x` has a lifetime `'b` that lasts only until the end of the inner scope
- The borrow checker identifies that `r` is a dangling reference because `x` does not live long enough

### Example 2
```rust
fn main() {
    let x = 5; // 'a
    let r = &x; // 'a
    println!("{}", r); // No error
}
```
- `x` has a lifetime `'a` that lasts until the end of `main()`
- `r` also has a lifetime `'a` that lasts until the end of `main()`
- The borrow checker determines that `r` is a valid reference because `x` lives long enough

## Generic Lifetime Annotations

- Generic lifetime annotations describe the relationship between the lifetimes of multiple references
- They do not change the actual lifetime of a reference, but rather explain the relationship between lifetimes

### Example 3
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
    }
    println!("The longest string is {}", result);
}
```
- The `longest()` function declares a generic lifetime `'a` and uses it to annotate the lifetimes of the input parameters and the return value
- This tells the borrow checker that the returned reference will have the same lifetime as the smallest lifetime of the input parameters
- In this case, `string2` has the smaller lifetime, so the returned reference will have the same lifetime as `string2`
- When `result` is printed outside the inner scope, the borrow checker knows that the lifetime of `string2` is still valid, so the code compiles

### Example 4
```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long"); // 'a
    let result;
    {
        let string2 = String::from("xyz"); // 'b
        result = longest(&string1, &string2);
    }
    println!("The longest string is {}", result); // Error: string2 does not live long enough
}
```
- In this example, `string1` has a longer lifetime `'a` than `string2` with lifetime `'b`
- The borrow checker knows that the returned reference will have the same lifetime as `string2`, which does not live long enough for the `println!` statement outside the inner scope

### Example 5
```rust
fn longest<'a>(x: &'a str) -> &'a str {
    x
}

fn main() {
    let string1 = String::from("long string is long"); // 'a
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1);
    }
    println!("The longest string is {}", result); // No error
}
```
- In this example, the `longest()` function always returns a reference to `x`, so the lifetime of the return value is always tied to the lifetime of `x`
- The borrow checker knows that the returned reference will have the same lifetime as `string1`, which lives until the en