
## Understanding the Deref Trait in Rust

The `Deref` trait in Rust allows instances of smart pointers to behave like references. This is useful for accessing the underlying data of a smart pointer in a way that is seamless and intuitive. In this guide, we'll explore the `Deref` trait with simple examples and see how it can be used with both standard and third-party libraries.

### What is the Deref Trait?

The `Deref` trait is used to override the `*` operator (dereference operator) for custom smart pointers. By implementing `Deref`, you can enable instances of your custom smart pointers to be dereferenced just like regular references.

### Example 1: Using `Deref` with `Box<T>`

Rust's `Box<T>` implements the `Deref` trait, allowing you to access the data stored on the heap as if it were a regular reference.

```rust
fn main() {
    let x = Box::new(5);
    let y = *x;
    println!("{}", y); // Outputs: 5
}
```

In this example, `*x` dereferences the `Box` to get the value `5`.

### Example 2: Custom Smart Pointer with `Deref`

Let's create a custom smart pointer and implement the `Deref` trait for it.

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = MyBox::new(5);
    let y = *x;
    println!("{}", y); // Outputs: 5
}
```

Here, `MyBox` is a custom smart pointer. By implementing `Deref`, we allow instances of `MyBox` to be dereferenced to access the value they hold.

### Example 3: Using `Deref` with `Rc<T>`

`Rc<T>` (Reference Counted) is another smart pointer that implements `Deref`.

```rust
use std::rc::Rc;

fn main() {
    let x = Rc::new(5);
    let y = *x;
    println!("{}", y); // Outputs: 5
}
```

In this example, `*x` dereferences the `Rc` pointer to get the value `5`.

### Example 4: Third-Party Library - `derive_more`

The `derive_more` crate provides an easy way to derive common traits, including `Deref`.

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
derive_more = "0.99"
```

#### Example: Using `derive_more` to Implement `Deref`

```rust
use derive_more::Deref;

#[derive(Deref)]
struct MyBox<T> {
    #[deref]
    value: T,
}

fn main() {
    let x = MyBox { value: 5 };
    let y = *x;
    println!("{}", y); // Outputs: 5
}
```

In this example, the `derive_more` crate simplifies the implementation of the `Deref` trait for `MyBox`.

### Advanced Example: Nested Dereferencing with `Deref`

You can also use `Deref` to enable nested dereferencing.

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct NestedBox<T>(MyBox<T>);

impl<T> NestedBox<T> {
    fn new(x: T) -> NestedBox<T> {
        NestedBox(MyBox::new(x))
    }
}

impl<T> Deref for NestedBox<T> {
    type Target = MyBox<T>;

    fn deref(&self) -> &MyBox<T> {
        &self.0
    }
}

fn main() {
    let x = NestedBox::new(5);
    let y = **x; // Dereferences twice to get the value inside NestedBox and MyBox
    println!("{}", y); // Outputs: 5
}
```

In this example, `NestedBox` contains `MyBox`, and by implementing `Deref` for both, you can dereference twice to access the inner value.

---------
### [Implicit Deref Coercions with Functions and Methods](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods)

_Deref coercion_  converts a reference to a type that implements the  `Deref`  trait into a reference to another type. For example, deref coercion can convert  `&String`  to  `&str`  because  `String`  implements the  `Deref`  trait such that it returns  `&str`. Deref coercion is a convenience Rust performs on arguments to functions and methods, and works only on types that implement the  `Deref`  trait. It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition. A sequence of calls to the  `deref`  method converts the type we provided into the type the parameter needs.

### [How Deref Coercion Interacts with Mutability](https://doc.rust-lang.org/book/ch15-02-deref.html#how-deref-coercion-interacts-with-mutability)

Similar to how you use the  `Deref`  trait to override the  `*`  operator on immutable references, you can use the  `DerefMut`  trait to override the  `*`  operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:

-   From  `&T`  to  `&U`  when  `T: Deref<Target=U>`
-   From  `&mut T`  to  `&mut U`  when  `T: DerefMut<Target=U>`
-   From  `&mut T`  to  `&U`  when  `T: Deref<Target=U>`