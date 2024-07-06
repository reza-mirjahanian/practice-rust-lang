## Turbofish

The term "turbofish" in programming refers to a specific syntax used in the Rust programming language for specifying type parameters explicitly. This syntax looks like `::<>` and is used to disambiguate types when calling generic functions or methods, especially when the types cannot be inferred automatically by the compiler.

The name "turbofish" comes from the shape of the syntax `::<...>`, which somewhat resembles a fish.

### When and Why to Use Turbofish

1. **Disambiguation in Type Inference**:
   Sometimes, the Rust compiler cannot infer the type of a generic parameter. In such cases, you can use the turbofish syntax to explicitly specify the type.

   ```rust
   let x: Vec<i32> = vec![1, 2, 3];
   let sum: i32 = x.iter().sum::<i32>(); // Using turbofish to specify the type of the sum
   ```

2. **Calling Generic Functions**:
   When calling a generic function, you might need to specify the type parameters explicitly if they cannot be inferred from the context.

   ```rust
   fn get_zero<T: Default>() -> T {
       T::default()
   }

   let x: i32 = get_zero::<i32>(); // Using turbofish to specify that T is i32
   ```

3. **Method Calls with Generic Return Types**:
   When calling methods that return a generic type, you often need to specify the type to avoid ambiguity.

   ```rust
   let s: String = "42".parse::<String>().unwrap(); // Using turbofish to specify the type of the result
   ```

### Examples

Here are some examples to illustrate the use of turbofish in Rust:

#### Example 1: Sum of Iterator

```rust
fn main() {
    let v = vec![1, 2, 3];
    let sum: i32 = v.iter().sum::<i32>(); // Specifying the type of the sum explicitly
    println!("Sum: {}", sum);
}
```

#### Example 2: Parsing a String

```rust
fn main() {
    let num: i32 = "42".parse::<i32>().unwrap(); // Specifying the type to parse the string into
    println!("Parsed number: {}", num);
}
```

#### Example 3: Using a Generic Function

```rust
fn main() {
    let zero: f64 = get_zero::<f64>(); // Specifying the type parameter for the function
    println!("Zero: {}", zero);
}

fn get_zero<T: Default>() -> T {
    T::default()
}
```

### Why It's Useful

- **Clarity**: Makes the programmer's intent clear when type inference is not sufficient.
- **Disambiguation**: Helps the compiler understand which specific type to use for a generic parameter.
- **Flexibility**: Allows for more precise control over type parameters in generic programming.

