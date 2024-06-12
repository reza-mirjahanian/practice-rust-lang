

##  Structs

#### Structs: Grouping Related Data Together
Structs allow you to group related data together. Think about them as object attributes in an object-oriented programming language.



2. **Defining a Struct**
   - Use the `struct` keyword followed by the name of the struct, curly brackets, and then list the attributes.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

#### Creating and Using Instances of Structs
- **Creating a New Instance:**
  - In `main`, create a new variable called `user1` and set it equal to a new instance of `User`.

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

- **Accessing Struct Fields:**
  - Use dot notation to get specific values.

```rust
let name = user1.username;
```

- **Modifying Struct Fields:**
  - Make the struct mutable to modify values.

```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
user1.email = String::from("anotheremail@example.com");
```

#### Functions to Construct Instances
- **Creating a Function:**

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

- **Using the Function:**
  - Call the function to create another user.

```rust
let user2 = build_user(
    String::from("another@example.com"),
    String::from("anotherusername123"),
);
```

#### Using Existing Instances
- **Creating New Instances from Existing Ones:**

```rust
let user3 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername123"),
    ..user1
};
```

### Tuple Structs and Unit-Like Structs
- **Tuple Structs:**

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

- **Unit-Like Structs:**
  - These are structs without any fields.

### Ownership and Lifetimes
- Our struct's fields own the data, but they can reference borrowed data using string slices. This requires lifetimes, which will be covered in Chapter 10.

## Example: Refactoring with Structs
- **Calculating the Area of a Rectangle:**

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}
```

### Methods and Associated Functions
- **Adding Methods to a Struct:**

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

- **Using Methods:**

```rust
let rect1 = Rectangle { width: 30, height: 50 };
let rect2 = Rectangle { width: 10, height: 40 };
let rect3 = Rectangle { width: 60, height: 45 };

println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
```

- **Associated Functions:**

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```
