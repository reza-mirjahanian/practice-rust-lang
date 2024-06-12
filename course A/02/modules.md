

### Project Organization



Imagine you had some chunk of code that dealt with **authentication**. You don't want the rest of your code to know everything about how the authentication code works. Maybe you just want to expose a method called **login** and keep the rest of the code hidden from the rest of your program.

### Rust's Module System

To address these needs, Rust has a module system that starts with a **package**. 

### Creating a Package

When you type in `cargo new`, you create a new package. A package stores **crates**. A crate could either be:

- A **binary crate** (code you can execute)
- A **library crate** (code that can be used by other programs)

### Crates and Modules

Crates contain **modules**. Modules allow you to:

- Organize chunks of code
- Control privacy rules

### Example: Authentication Module

Let's say you have a library crate that contains an **authentication module**. You can then make the code inside your authentication module private but expose one public **login** method. If we wanted code outside of the authentication module to call the public login method, it would have to specify a path to that login method.

### Workspaces

Rust also has something called **workspaces**, which are meant for very large projects and allow you to store interrelated packages inside the workspace. We'll talk more about that in **Chapter 14**.

### Creating Packages and Crates

Let's jump into creating some packages and crates.

1. **Create a new package called `my_project`:**
   ```sh
   cargo new my_project
   ```
2. **Navigate to the project directory and open it in VS Code:**
   ```sh
   cd my_project
   code .
   ```
3. **Default File Structure:**
   - **Cargo.toml**: Defines the package and crate.
   - **src/main.rs**: Automatically creates a binary crate.

### Library Crates

To create a library crate:

1. **Create a `lib.rs` file:**
   ```sh
   touch src/lib.rs
   ```
2. **Rust automatically creates a library crate if `lib.rs` is defined in the root of your source directory.**

### Rules Around Crates

1. A package must have at least one crate.
2. A package could have zero or one library crate.
3. A package could have any number of binary crates.

### Example: Adding More Binary Crates

To add more binary crates, create a folder called `bin` and add files to it. Each file represents another binary crate.

### Modules

Let's create a new package called **restaurant** with a library crate.

1. **Create a new package:**
   ```sh
   cargo new restaurant --lib
   ```
2. **Navigate to the directory and open it in VS Code:**
   ```sh
   cd restaurant
   code .
   ```

### Structuring the Restaurant Example

Think of a restaurant as two parts:

- **Front of the House**: Area where the customers are.
- **Back of the House**: Area where food is made, dishes are cleaned, and the manager typically is.

### Example Code for Restaurant

#### Defining Modules

At the top, we have a module called **front_of_house** specified using the `mod` keyword.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}
```

### Module Tree

- **crate**
  - **front_of_house**
    - **hosting**
      - `add_to_waitlist`
      - `seat_at_table`
    - **serving**
      - `take_order`
      - `serve_order`
      - `take_payment`

### Paths

To reference an item in your module tree, specify a path using identifiers separated by double colons `::`.

#### Absolute Path Example

```rust
crate::front_of_house::hosting::add_to_waitlist()
```

#### Relative Path Example

```rust
front_of_house::hosting::add_to_waitlist()
```

### Privacy Rules

By default, a child module and everything inside it is private from the perspective of the parent module.

#### Making Modules Public

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

### Using Super Keyword

In this example, we have a function `serve_order` defined in the top crate module and use the `super` keyword to reference the parent module.

### Structs and Privacy

#### Example Struct

```rust
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}
```

By default, fields within the struct are private.

### Enums and Privacy

#### Example Enum

```rust
pub enum Appetizer {
    Soup,
    Salad,
}
```

Marking an enum as public makes all its variants public.

### Using the `use` Keyword

The `use` keyword allows you to bring a path into scope.

#### Example

```rust
use crate::front_of_house::hosting;
hosting::add_to_waitlist();
```

#### Nested Paths

To bring multiple items into scope from the same path, use nested paths.

#### Example

```rust
use std::io::{self, Write};
```

### Re-exporting

To allow external code to access an internal module, use the `pub` keyword with `use`.

```rust
pub use crate::front_of_house::hosting;
```

### Bringing External Dependencies into Scope

Add dependencies in `Cargo.toml` and use them in your code.

```rust
use rand::Rng;
```

### Moving Modules to Separate Files

To move the definition of a module into another file:

1. Create a new file `front_of_house.rs`.
2. Move the module definition into `front_of_house.rs`.
3. Replace the module definition with a semicolon in `lib.rs`.

#### Example

```rust
mod front_of_house;
```

### Summary

In this video, we learned about Rust's module system:

- Packages
- Crates
- Modules
- Privacy rules
- Paths
- Bringing paths into scope
- Re-exporting
- Separating module content into separate files

