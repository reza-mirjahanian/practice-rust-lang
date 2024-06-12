
# Eight Common Mistakes New Rust Developers Make

## 1. Unnecessary Indirection

-   New Rust developers may not notice that  `` `String` ``  can be automatically coerced to  `` `&str` ``
-   Changing  `` `s` ``  to  `` `&str` ``  makes the API more flexible and avoids unnecessary indirection

## 2. Overusing Slice Indexing

-   Indexing into a slice or array can crash the program with an "index out of bounds" error
-   Rust provides safer alternatives like  `` `array_windows` ``  and  `` `map` ``  methods to avoid these issues

## 3. Using Sentinel Values

-   Rust's type system allows representing optional values without sentinel values
-   Using enums and  `` `Option` ``  can make code safer and more flexible

## 4. Not Using Enums Enough

-   Rust's powerful enums can be used for pattern matching and other operations
-   New Rust developers may not utilize enums enough, leading to less efficient and less readable code

## 5. Not Properly Handling Errors

-   Rust's error handling can be complex, but it's important to use  `` `?` ``  operator for error propagation and implement  `` `Error` ``  trait for custom error types

## 6. Not Using Standard Library Traits

-   Rust's standard library provides useful traits like  `` `Default` ``,  `` `From` ``,  `` `TryFrom` ``, and  `` `Display` ``
-   New Rust developers may not take advantage of these traits, leading to more verbose and less efficient code

## 7. Not Using Macros Enough

-   Rust's macros can simplify and improve code, but new developers may not utilize them enough
-   Using macros like  `` `to_do` ``,  `` `if_let` ``, and  `` `concat` ``  can make code more concise and readable

## 8. Not Using Rust's Tooling

-   Rust provides powerful tools like  `` `cargo format` ``  and  `` `cargo clippy` ``  for code formatting and linting
-   New Rust developers may not take advantage of these tools, leading to inconsistent and less maintainable code

### Bonus Mistake: Overusing Smart Pointers

-   Using  `` `Rc` ``  and  `` `RefCell` ``  or  `` `Arc` ``  and  `` `Mutex` ``  can introduce unnecessary complexity and decrease memory safety
-   It's better to change the API or code structure to avoid the need for smart pointers