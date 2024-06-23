
### Macros 

Macros in Rust are a powerful metaprogramming tool that allow you to write code that writes other code. They come in two main forms: declarative macros (macro_rules!) and procedural macros.

1. Declarative Macros (macro_rules!)

Declarative macros use pattern matching to define code generation.

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    say_hello!();
    say_hello!("Alice");
}
```

This macro demonstrates two patterns: one without arguments and one with a name argument.

2. Variadic Macros

```rust
macro_rules! print_all {
    ($($arg:expr),*) => {
        $(
            print!("{} ", $arg);
        )*
        println!();
    };
}

fn main() {
    print_all!(1, "hello", 3.14);
}
```

This macro can take any number of arguments and print them.

3. Using Standard Library Macros

The Rust standard library provides many useful macros:

```rust
fn main() {
    let v = vec![1, 2, 3];  // vec! macro
    assert_eq!(v[0], 1);    // assert_eq! macro
    
    let map = hashmap! {    // hashmap! macro from maplit crate
        "a" => 1,
        "b" => 2
    };
    println!("{:?}", map);
}
```

Note: The `hashmap!` macro is from the `maplit` crate, not the standard library.

4. Procedural Macros

Procedural macros are more powerful but require a separate crate. Here's a simple derive macro using the `syn` and `quote` crates:

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    let expanded = quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    };
    
    TokenStream::from(expanded)
}
```

To use this macro:

```rust
use hello_world_derive::HelloWorld;

#[derive(HelloWorld)]
struct MyStruct;

fn main() {
    MyStruct::hello_world();
}
```

5. Attribute-like Procedural Macros

These macros can be used to create custom attributes:

```rust
#[proc_macro_attribute]
pub fn log_function_call(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    
    let expanded = quote! {
        #input
        
        #[allow(non_upper_case_globals)]
        static #name: () = {
            println!("Function '{}' is defined", stringify!(#name));
        };
    };
    
    TokenStream::from(expanded)
}
```

Usage:

```rust
#[log_function_call]
fn my_function() {
    println!("Inside my_function");
}

fn main() {
    my_function();
}
```

6. Using Third-Party Macro Libraries: `serde`

The `serde` crate provides macros for easy serialization and deserialization:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };
    let json = serde_json::to_string(&point).unwrap();
    println!("JSON: {}", json);
    
    let deserialized: Point = serde_json::from_str(&json).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
```

Macros in Rust are a powerful feature that allows for metaprogramming, code generation, and domain-specific languages. They can significantly reduce boilerplate and enable expressive APIs, but should be used judiciously as they can also make code harder to understand if overused.
