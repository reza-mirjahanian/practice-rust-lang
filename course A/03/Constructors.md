


**Creating New Instances**

In Rust, there is only one true way to create a new instance of a struct or enum, which is to specify the struct or enum name and initialize all of its fields.

Example:
```rust
let user1 = User {
    username: "testuser123",
    role: Role::Creator,
    id: 1,
};
```

**Private Fields**

If we add a private field to the `User` struct, such as `id`, we will get an error when trying to create a new instance of `User` in `main.rs`, because we are not initializing the private field.

To get around this error, we can follow a Rust convention by creating an associated function on the `User` struct called `new`, which will return a new instance of `User`. This function acts as a constructor function, and can take arguments to initialize the fields of the struct.

Example:
```rust
impl User {
    fn new(username: &str) -> Result<User, &str> {
        if username == "testuser123" {
            Err("Username already exists")
        } else {
            Ok(User {
                username: username.to_string(),
                role: Role::Creator,
                id: rand::random(),
            })
        }
    }
}
```

**Default Constructors**

The Rust standard library has a trait called `Default` which when implemented on a type gives useful default values. We can implement the `Default` trait for `User` to provide default values for its fields.

Example:
```rust
impl Default for User {
    fn default() -> User {
        User {
            username: "guest".to_string(),
            role: Role::Guest,
            id: rand::random(),
        }
    }
}
```

**Derive New Crate**

The `derive_new` crate allows you to implement the `new` constructor function for structs and enums using a derived macro. This can save time and make the code more concise.

Example:
```rust
#[derive(new, Debug)]
struct Post {
    content: String,
    #[new(value = "vec![\""rusty"\".to_string()]")]
    tags: Vec<String>,
    #[new(default)]
    likes: u32,
}

let post2 = Post::new("example content");
```

In summary, there is only one true constructor for structs and enums in Rust, which is to specify the name of the structure or enum and initialize all of its fields. However, there are conventions in Rust for adding a constructor function called `new` to your type, which can take arguments, set default values, and perform some work before creating a new instance. 