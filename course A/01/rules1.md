
## Ownership Rules

The ownership rules in Rust are straightforward. Each value in Rust has an owner, and there can only be one owner at a time. When the owner goes out of scope, the value will be dropped. Moving values also applies to functions, and when we set a variable equal to another, the value is moved into the new variable, and the original variable is invalidated.

## Borrowing Rules

While the ownership system provides a solid foundation for memory safety, there are often cases where we want to access data without transferring ownership. This is where borrowing comes in. Creating references in Rust is called borrowing because metaphorically the reference borrows access to the data from the owner without taking ownership itself.

There are two borrowing rules. The first rule is that at any given time, you can either have one mutable reference or any number of immutable references to a value. The second rule is that references must always be valid. The borrow checker evaluates the lifetimes of values to ensure the ownership rules are being followed.

## Lifetimes

Lifetimes refer to the span of the program during which a specific piece of data is valid. The borrow checker uses lifetimes to determine if the reference is valid. In cases where the borrow checker needs help understanding relationships between lifetimes, we can use generic lifetime annotations to express relationships between lifetimes.


**Zero Cost Abstractions**

-   Borrowed from C++'s RAII principle
-   High-level abstractions like generics and collections have no runtime overhead
-   Enables writing clean, concise code without sacrificing performance

**Ownership Model**

-   Borrowed from C++'s RAII
-   Resources tied to object lifetimes
-   Eliminates resource leaks and null pointer dereferencing
-   Enforced through ownership rules and borrowing rules

**Algebraic Data Types (ADTs)**

-   Borrowed from functional languages like Haskell
-   Allows creating composite types with variants
-   Enables modeling complex data structures and enforcing constraints
-   Implemented through enums and structs in Rust

**Pattern Matching**

-   Borrowed from functional languages like Haskell
-   Allows handling different variants of enums
-   Enables writing clean, readable code
-   Enforces exhaustiveness to catch errors at compile time

**Polymorphism**

-   Implemented through traits and generics
-   Allows behavior to be added to types without modifying them
-   Enables flexible code design and composition
-   Avoids fragile base class problem

**Async/Await Syntax**

-   Borrowed from JavaScript's async/await
-   Simplifies asynchronous programming by making it look synchronous
-   Enables writing readable, non-nested code
-   Rust's implementation is lazy and incorporates zero cost abstractions

**Cargo and Crates.io**

-   Borrowed from Node.js's npm and mpmjs.com
-   Rust's official build system and package manager
-   Handles building, testing, and dependency management
-   Enables reliable dependency management and simple package distribution