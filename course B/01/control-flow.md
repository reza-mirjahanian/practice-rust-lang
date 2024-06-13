
### Control flow in Rust 
## `if` Statements

### Basic Usage
The `if` statement allows conditional execution of code blocks.
```rust
let number = 7;

if number < 5 {
    println!("Condition is true");
} else {
    println!("Condition is false");
}
```

### Using `if` as an Expression
In Rust, `if` statements are expressions and can return values.
```rust
let condition = true;
let number = if condition { 5 } else { 10 };

println!("The value of number is: {}", number); // Outputs: The value of number is: 5
```

## `match` Statements

### Basic Usage
The `match` statement allows pattern matching and can replace complex conditional logic.
```rust
let number = 3;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Something else"), // `_` is the default case
}
```

### Matching Enums
`match` is particularly powerful with enums.
```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let direction = Direction::Left;

match direction {
    Direction::Up => println!("Up"),
    Direction::Down => println!("Down"),
    Direction::Left => println!("Left"),
    Direction::Right => println!("Right"),
}
```

### Using `match` as an Expression
Similar to `if`, `match` can also return values.
```rust
let number = 2;

let result = match number {
    1 => "One",
    2 => "Two",
    3 => "Three",
    _ => "Other",
};

println!("The result is: {}", result); // Outputs: The result is: Two
```

## Loops

### `loop`
The `loop` keyword creates an infinite loop, which you can break out of with the `break` keyword.
```rust
let mut counter = 0;

loop {
    counter += 1;

    if counter == 10 {
        break;
    }
}

println!("Counter reached: {}", counter); // Outputs: Counter reached: 10
```

### `while`
The `while` loop executes as long as a condition is true.
```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}

println!("LIFTOFF!!!");
```

### `for`
The `for` loop iterates over a range or collection.
```rust
for number in 1..5 { // `1..5` is a range from 1 to 4
    println!("{}!", number);
}

for number in [1, 2, 3].iter() { // Iterating over an array
    println!("{}!", number);
}
```

### Loop Labels
You can use loop labels to break out of nested loops.
```rust
'outer: for x in 0..5 {
    for y in 0..5 {
        if x == 2 && y == 2 {
            break 'outer; // Breaks out of the outer loop
        }
        println!("x: {}, y: {}", x, y);
    }
}
```

## Advanced Control Flow Tricks

### Using `if let` for Pattern Matching
`if let` simplifies matching for a single pattern.
```rust
let some_option = Some(7);

if let Some(value) = some_option {
    println!("The value is: {}", value);
} else {
    println!("No value");
}
```

### Using `while let` for Looping with Pattern Matching
`while let` can be used to loop while a pattern matches.
```rust
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("The top value is: {}", top);
}
```

### Combining `if` with `match`
You can combine `if` with `match` for more complex conditions.
```rust
let number = Some(5);

if let Some(5) = number {
    println!("Matched five");
} else {
    match number {
        Some(n) => println!("Matched: {}", n),
        None => println!("No match"),
    }
}
```

### Early Returns with `break` and `continue`
Use `break` to exit a loop early and `continue` to skip to the next iteration.
```rust
for i in 0..10 {
    if i == 2 {
        continue; // Skips the rest of the loop body for this iteration
    }
    if i == 5 {
        break; // Exits the loop entirely
    }
    println!("{}", i);
}
```

### Using `match` Guards
Match guards add additional conditions to patterns.
```rust
let number = 4;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    n if n % 2 == 0 => println!("Even number"),
    _ => println!("Other"),
}
```

These control flow constructs and tricks allow you to write robust and expressive Rust programs, handling a variety of conditions and loops efficiently.