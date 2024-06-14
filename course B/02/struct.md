### Structs


### [Associated Functions](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions)

All functions defined within an  `impl`  block are called  _associated functions_  because they’re associated with the type named after the  `impl`. We can define associated functions that don’t have  `self`  as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this: the  `String::from`  function that’s defined on the  `String`  type.




### 1. Basic Structs
Define a basic struct using the `struct` keyword.

```rust
struct Point {
    x: i32,
    y: i32,
}
```

### 2. Instantiate Structs
Create instances of structs using field init shorthand.

```rust
let p = Point { x: 5, y: 10 };
```

### 3. Struct Update Syntax
Use struct update syntax to create a new instance from an existing one, changing some fields.

```rust
let p2 = Point { x: 15, ..p };
```

### 4. Tuple Structs
Define tuple structs for simple structures without named fields.

```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

### 5. Unit-Like Structs
Create unit-like structs for types that don't hold data but can implement traits.

```rust
struct Marker;
```

### 6. Implementing Methods
Use the `impl` block to add methods to structs.

```rust
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn x(&self) -> i32 {
        self.x
    }
}
```

### 7. Associated Functions
Create associated functions (like constructors) using `impl`.

```rust
impl Point {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}
```

### 8. Debug and Display Traits
Derive the `Debug` trait for printable structs. Implement the `Display` trait for custom formatting.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

use std::fmt;

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle: {} x {}", self.width, self.height)
    }
}
```

### 9. Default Trait
Implement the `Default` trait for default values.

```rust
#[derive(Default)]
struct MyStruct {
    foo: i32,
    bar: String,
}

let default_instance = MyStruct::default();
```

### 10. Destructuring
Destructure structs to access fields directly.

```rust
let Point { x, y } = p;
println!("x: {}, y: {}", x, y);
```

### 11. Nested Structs
Define and use nested structs for complex data.

```rust
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

let rect = Rectangle {
    top_left: Point { x: 0, y: 0 },
    bottom_right: Point { x: 10, y: 10 },
};
```

### 12. Structs with Lifetimes
Use lifetimes for structs holding references.

```rust
struct Slice<'a> {
    data: &'a [i32],
}
```

### 13. Pattern Matching
Use pattern matching with structs.

```rust
match p {
    Point { x: 0, y } => println!("On the y-axis at y = {}", y),
    Point { x, y: 0 } => println!("On the x-axis at x = {}", x),
    Point { x, y } => println!("At ({}, {})", x, y),
}
```

### 14. Using `#[derive]`
Automatically implement common traits using `#[derive]`.

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

### 15. Enums with Structs
Combine enums and structs for more complex types.

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

let shape = Shape::Circle { radius: 10.0 };

match shape {
    Shape::Circle { radius } => println!("Circle with radius {}", radius),
    Shape::Rectangle { width, height } => println!("Rectangle: {} x {}", width, height),
}
```

### 16. Newtype Pattern
Use the newtype pattern for type safety.

```rust
struct Inches(i32);

impl Inches {
    fn to_centimeters(self) -> f64 {
        self.0 as f64 * 2.54
    }
}

let length = Inches(10);
println!("{} inches is {} cm", length.0, length.to_centimeters());
```

### 17. Using `self` in Methods
Understand the difference between `self`, `&self`, and `&mut self`.

```rust
impl Point {
    fn move_by(self, dx: i32, dy: i32) -> Self {
        Self { x: self.x + dx, y: self.y + dy }
    }

    fn move_by_ref(&self, dx: i32, dy: i32) -> Point {
        Point { x: self.x + dx, y: self.y + dy }
    }

    fn move_by_mut(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}
```

### 18. Using Structs in Collections
Use structs with standard collections.

```rust
let points = vec![
    Point { x: 1, y: 2 },
    Point { x: 3, y: 4 },
];
```

### 19. Immutability and Mutability
Understand when to use `mut` with structs.

```rust
let mut p = Point { x: 0, y: 0 };
p.move_to(5, 5);
```

### 20. Documentation
Document your structs and methods.

```rust
/// A point in 2D space.
struct Point {
    /// The x coordinate.
    x: i32,
    /// The y coordinate.
    y: i32,
}

impl Point {
    /// Creates a new point.
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
```

------------
1.  **Use  `new`  function for initialization**: It's a common convention to include a  `new`  function in your struct that initializes the fields. This function should return a new instance of the struct.

```
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

```
2.  **Use  `derive`  for common traits**: Rust's  `derive`  macro can automatically implement common traits for your struct, such as  `Debug`,  `Clone`,  `Copy`,  `PartialEq`, and  `Eq`.
