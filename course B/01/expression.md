

## Expressions

### Definition
An **expression** is a piece of code that evaluates to a value. Expressions can be simple, like literals, or complex, involving operations and function calls. Importantly, expressions always produce a value and often have side effects.

### Characteristics
- **Evaluates to a value**: Every expression results in a value.
- **Can be part of other expressions**: Expressions can be nested or combined.
- **Does not end with a semicolon**: Typically, an expression without a trailing semicolon returns its value to the surrounding code.

### Examples

#### Simple Literals
```rust
let x = 5; // `5` is an expression that evaluates to 5
```

#### Arithmetic Operations
```rust
let y = 2 + 3; // `2 + 3` is an expression that evaluates to 5
```

#### Function Calls
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // `a + b` is an expression
}

let result = add(2, 3); // `add(2, 3)` is an expression
```

#### Blocks
```rust
let z = {
    let a = 2;
    let b = 3;
    a + b // The block itself is an expression, evaluating to 5
};
```

## Statements

### Definition
A **statement** is an instruction that performs an action. Statements are used to organize the code into meaningful units. They do not return a value.

### Characteristics
- **Does not evaluate to a value**: Statements perform actions and do not produce values that can be used.
- **Forms the building blocks of a program**: Statements structure the flow of control in a program.
- **Ends with a semicolon**: Most statements end with a semicolon, which distinguishes them from expressions.

### Examples

#### Variable Declarations
```rust
let x = 5; // This is a statement that declares and initializes a variable
```

#### Expressions as Statements
```rust
let y = 3 + 4; // `3 + 4` is an expression, but the whole line is a statement
```

#### Control Flow Statements
```rust
if x > 0 {
    println!("x is positive");
} // The entire if block is a statement

for i in 0..5 {
    println!("{}", i);
} // The for loop is a statement
```

### Expression vs. Statement

#### Distinguishing Characteristics
- **Semicolons**: Adding a semicolon turns an expression into a statement. For example, `let x = 5` is an expression, but `let x = 5;` is a statement.
- **Return Values**: Expressions return values, while statements do not.

#### Example Comparison
```rust
// Expression
let x = 3 + 4; // The `3 + 4` is an expression resulting in 7, assigned to x

// Statement
let y = {
    let a = 2;
    let b = 3;
    a + b // This block is an expression, but the whole line `let y = { ... };` is a statement
};

// If we add a semicolon to the inner expression
let z = {
    let a = 2;
    let b = 3;
    a + b; // This becomes a statement, and z will be assigned `()`, the unit type
};
```

### Practical Implications

Understanding the difference between expressions and statements helps in writing more readable and correct Rust code, particularly in scenarios involving control flow and function return types.

#### Control Flow
In Rust, many control flow constructs like `if` and `match` are expressions. This allows for more concise and flexible code.

```rust
let condition = true;
let number = if condition { 5 } else { 10 }; // `if` is an expression
println!("Number is: {}", number); // Output: Number is: 5
```

#### Returning Values
Functions in Rust often use expressions to return values implicitly.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // Expression returning the sum
}

fn main() {
    let result = add(2, 3);
    println!("Result: {}", result); // Output: Result: 5
}
```

Understanding expressions and statements helps in leveraging Rust's design for cleaner, more efficient, and idiomatic code.