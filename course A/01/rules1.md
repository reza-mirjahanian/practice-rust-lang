
## Ownership Rules

The ownership rules in Rust are straightforward. Each value in Rust has an owner, and there can only be one owner at a time. When the owner goes out of scope, the value will be dropped. Moving values also applies to functions, and when we set a variable equal to another, the value is moved into the new variable, and the original variable is invalidated.

## Borrowing Rules

While the ownership system provides a solid foundation for memory safety, there are often cases where we want to access data without transferring ownership. This is where borrowing comes in. Creating references in Rust is called borrowing because metaphorically the reference borrows access to the data from the owner without taking ownership itself.

There are two borrowing rules. The first rule is that at any given time, you can either have one mutable reference or any number of immutable references to a value. The second rule is that references must always be valid. The borrow checker evaluates the lifetimes of values to ensure the ownership rules are being followed.

## Lifetimes

Lifetimes refer to the span of the program during which a specific piece of data is valid. The borrow checker uses lifetimes to determine if the reference is valid. In cases where the borrow checker needs help understanding relationships between lifetimes, we can use generic lifetime annotations to express relationships between lifetimes.