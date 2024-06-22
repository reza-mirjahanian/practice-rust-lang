Object-oriented programming (OOP) in Rust can be achieved using structs and traits. While Rust does not have classes like traditional OOP languages, it supports encapsulation, inheritance (through traits), and polymorphism. Here are simple examples demonstrating OOP principles using both standard and third-party libraries.

### Using Standard Library

#### Example with Structs and Traits

Let's create a basic example involving a `Shape` trait and two structs, `Circle` and `Rectangle`, implementing this trait.

```rust
// Define the Shape trait
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// Define a Circle struct
struct Circle {
    radius: f64,
}

// Implement the Shape trait for Circle
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// Define a Rectangle struct
struct Rectangle {
    width: f64,
    height: f64,
}

// Implement the Shape trait for Rectangle
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 4.0, height: 5.0 };

    // Using polymorphism with trait objects
    let shapes: Vec<&dyn Shape> = vec![&circle, &rectangle];

    for shape in shapes {
        println!("Area: {}, Perimeter: {}", shape.area(), shape.perimeter());
    }
}
```

In this example:
- The `Shape` trait defines the interface with `area` and `perimeter` methods.
- The `Circle` and `Rectangle` structs implement the `Shape` trait.
- We use a vector of trait objects (`&dyn Shape`) to demonstrate polymorphism.

### Using Third-Party Library

#### Example with `derive_more` for Additional Functionality

We can use the `derive_more` crate to simplify some repetitive code like constructor methods.

First, add the dependency to your `Cargo.toml` file:
```toml
[dependencies]
derive_more = "0.99.17"
```

Now, let's create a similar example using `derive_more` for automatic implementation of some methods.

```rust
use derive_more::Constructor;

// Define the Shape trait
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// Define a Circle struct with a constructor
#[derive(Constructor)]
struct Circle {
    radius: f64,
}

// Implement the Shape trait for Circle
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// Define a Rectangle struct with a constructor
#[derive(Constructor)]
struct Rectangle {
    width: f64,
    height: f64,
}

// Implement the Shape trait for Rectangle
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    let circle = Circle::new(3.0);
    let rectangle = Rectangle::new(4.0, 5.0);

    // Using polymorphism with trait objects
    let shapes: Vec<&dyn Shape> = vec![&circle, &rectangle];

    for shape in shapes {
        println!("Area: {}, Perimeter: {}", shape.area(), shape.perimeter());
    }
}
```

In this example:
- The `derive_more::Constructor` attribute macro automatically generates constructor methods for `Circle` and `Rectangle`.
- The rest of the implementation remains the same, demonstrating OOP principles with simplified struct initialization.

### Explanation

1. **Standard Library Example**:
   - Defines a `Shape` trait with methods for calculating area and perimeter.
   - Implements the trait for `Circle` and `Rectangle` structs.
   - Demonstrates polymorphism by storing different shapes in a vector of trait objects.

2. **Third-Party Library Example**:
   - Uses the `derive_more` crate to automatically generate constructor methods.
   - Simplifies struct initialization while maintaining the same OOP principles.

Both examples showcase how Rust supports OOP concepts like encapsulation, inheritance through traits, and polymorphism, leveraging both the standard library and third-party crates for additional functionality.

---------------------

As a seasoned technical writer, I'll explain object-oriented programming (OOP) in Rust, providing clear examples with both standard and third-party libraries.

Rust supports many OOP concepts, though it implements them differently from traditional OOP languages. Let's explore key OOP features in Rust:

1. Encapsulation

Rust uses modules and visibility rules for encapsulation:

```rust
mod car {
    pub struct Car {
        make: String,
        model: String,
        year: u32,
    }

    impl Car {
        pub fn new(make: String, model: String, year: u32) -> Self {
            Car { make, model, year }
        }

        pub fn info(&self) -> String {
            format!("{} {} ({})", self.make, self.model, self.year)
        }
    }
}

fn main() {
    let my_car = car::Car::new(String::from("Toyota"), String::from("Corolla"), 2022);
    println!("Car info: {}", my_car.info());
}
```

2. Inheritance and Polymorphism

Rust doesn't have traditional inheritance but uses traits for shared behavior:

```rust
trait Vehicle {
    fn start(&self);
    fn stop(&self);
}

struct Car;
struct Motorcycle;

impl Vehicle for Car {
    fn start(&self) {
        println!("Car engine starts");
    }
    fn stop(&self) {
        println!("Car engine stops");
    }
}

impl Vehicle for Motorcycle {
    fn start(&self) {
        println!("Motorcycle engine starts");
    }
    fn stop(&self) {
        println!("Motorcycle engine stops");
    }
}

fn main() {
    let vehicles: Vec<Box<dyn Vehicle>> = vec![
        Box::new(Car),
        Box::new(Motorcycle),
    ];

    for vehicle in vehicles {
        vehicle.start();
        vehicle.stop();
    }
}
```

3. Composition

Rust encourages composition over inheritance:

```rust
struct Engine {
    horsepower: u32,
}

struct Car {
    engine: Engine,
    model: String,
}

impl Car {
    fn new(model: String, horsepower: u32) -> Self {
        Car {
            engine: Engine { horsepower },
            model,
        }
    }

    fn describe(&self) {
        println!("{} with {}hp engine", self.model, self.engine.horsepower);
    }
}

fn main() {
    let car = Car::new(String::from("Mustang"), 300);
    car.describe();
}
```

4. Using a third-party library for OOP

Let's use the `derive_more` crate to simplify some common OOP patterns:

```rust
use derive_more::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
struct Stack<T> {
    #[deref]
    #[deref_mut]
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Stack size: {}", stack.len());
    println!("Popped: {:?}", stack.pop());
}
```

This example uses `derive_more` to automatically implement `Deref` and `DerefMut` traits, allowing `Stack` to behave like a `Vec` while adding custom methods.

-----------

-   **rust-oop**: A library that provides a set of traits and macros for object-oriented programming in Rust.