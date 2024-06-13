## converting strings to integers:

### Basic Method: Using `parse`

The most common way to parse a string to an integer is to use the `parse` method, which works with a variety of numeric types.

#### Example
```rust
fn main() {
    let s = "42";
    let number: i32 = s.parse().expect("Not a number!");
    println!("Parsed number: {}", number);
}
```

### Handling Errors: Using `Result`

To handle errors more gracefully, you can use the `Result` type returned by `parse`.

#### Example
```rust
fn main() {
    let s = "42";
    match s.parse::<i32>() {
        Ok(number) => println!("Parsed number: {}", number),
        Err(e) => println!("Failed to parse: {}", e),
    }
}
```

### Using `from_str` Method

The `from_str` method is another way to parse a string to an integer. This method is equivalent to `parse` but can sometimes make the code more readable.

#### Example
```rust
use std::str::FromStr;

fn main() {
    let s = "42";
    let number = i32::from_str(s).expect("Not a number!");
    println!("Parsed number: {}", number);
}
```

### Using `parse` with Turbofish

The turbofish syntax (`::<T>`) can make it explicit what type you're parsing to, which can be helpful for readability or to avoid type inference issues.

#### Example
```rust
fn main() {
    let s = "42";
    let number = s.parse::<i32>().expect("Not a number!");
    println!("Parsed number: {}", number);
}
```

### Default Value on Error

If you want to provide a default value in case of a parsing error, you can use the `unwrap_or` method.

#### Example
```rust
fn main() {
    let s = "not a number";
    let number = s.parse::<i32>().unwrap_or(0);
    println!("Parsed number: {}", number);
}
```

### Using `unwrap_or_else` to Handle Errors

If you want to provide a more complex error handling logic, you can use `unwrap_or_else`.

#### Example
```rust
fn main() {
    let s = "not a number";
    let number = s.parse::<i32>().unwrap_or_else(|_| {
        println!("Parsing error occurred, returning default value.");
        0
    });
    println!("Parsed number: {}", number);
}
```

### Custom Parsing Function

You can also write a custom function to encapsulate your parsing logic, especially if you need to handle multiple numeric types or have more complex error handling.

#### Example
```rust
fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

fn main() {
    let s = "42";
    match parse_int(s) {
        Ok(number) => println!("Parsed number: {}", number),
        Err(e) => println!("Failed to parse: {}", e),
    }
}
```

### Parsing Multiple Numbers

If you need to parse multiple numbers from a string, you can split the string and parse each part individually.

#### Example
```rust
fn main() {
    let s = "42 100 -5";
    let numbers: Vec<i32> = s.split_whitespace()
                             .map(|x| x.parse::<i32>().expect("Not a number!"))
                             .collect();
    println!("Parsed numbers: {:?}", numbers);
}
```

### Using `try_into`

If you want to ensure that the parsed number fits into a smaller numeric type, you can use `try_into`.

#### Example
```rust
use std::convert::TryInto;

fn main() {
    let s = "42";
    let number: i64 = s.parse().expect("Not a number!");
    let smaller_number: i32 = number.try_into().expect("Number too large!");
    println!("Parsed number: {}", smaller_number);
}
```

### Summary

- **Basic Parsing**: `s.parse::<i32>()`
- **Error Handling**: Using `Result` with `match`
- **Default Value**: Using `unwrap_or`
- **Complex Error Handling**: Using `unwrap_or_else`
- **Custom Function**: Encapsulating parsing logic
- **Multiple Numbers**: Splitting and parsing each part
- **Type Conversion**: Using `try_into` for ensuring the parsed number fits into a smaller type


## Method 5: Using  `from_str_radix()`  for Conversions from Different Bases

To convert strings from different number bases (e.g., binary, octal, hexadecimal), you can use the `from_str_radix()` method.


```javascript
let bin = "101101";
let i = i32::from_str_radix(bin, 2).unwrap();
println!("{}", i);
let oct = "43127";
let i = i64::from_str_radix(oct, 8).unwrap();
println!("{}", i);
let hex = "5F259A";
let i = u32::from_str_radix(hex, 16).unwrap();
println!("{}", i);
```