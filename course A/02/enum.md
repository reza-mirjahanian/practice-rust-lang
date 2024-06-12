

##  Enums
- **Overview:** 
  - Enums in Rust
  - Option Enum
  - Pattern Matching

---

## Enums in Rust
- **Definition:** Enums allow us to enumerate a list of variants.
- **Comparison to Structs:**
  - Example: IP Addresses
    - Only two variants: Version 4 and Version 6.
  - Using Enums to define IP addresses makes sense.

---

### Creating Enums
- **Syntax:**
  ```rust
  enum IpAddressKind {
      V4,
      V6,
  }
  ```

- **Instantiating Enums:**
  ```rust
  let four = IpAddressKind::V4;
  let six = IpAddressKind::V6;
  ```

---

### Combining Enums with Structs
- **Struct with Enum:**
  ```rust
  struct IpAddress {
      kind: IpAddressKind,
      address: String,
  }
  ```

- **Instantiating Struct:**
  ```rust
  let localhost = IpAddress {
      kind: IpAddressKind::V4,
      address: String::from("127.0.0.1"),
  };
  ```

- **Improvement:** Storing data directly inside the enum variants.
  ```rust
  enum IpAddr {
      V4(String),
      V6(String),
  }

  let localhost = IpAddr::V4(String::from("127.0.0.1"));
  ```

---

### Advanced Enum Variants
- **Storing Different Types:**
  ```rust
  enum IpAddr {
      V4(u8, u8, u8, u8),
      V6(String),
  }
  ```

- **Example Enum with Multiple Types:**
  ```rust
  enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(i32, i32, i32),
  }
  ```

---

### Methods and Associated Functions on Enums
- **Implementation Block:**
  ```rust
  impl Message {
      fn call(&self) {
          // Method body
      }
  }
  ```

---

## The Option Enum
- **Null Values:** Rust does not have null values. Instead, it uses the `Option` enum.
  ```rust
  enum Option<T> {
      Some(T),
      None,
  }
  ```

- **Examples of Optional Values:**
  ```rust
  let some_number = Some(5);
  let some_string = Some("a string");
  let absent_number: Option<i32> = None;
  ```

---

### Handling Optional Values
- **Pattern Matching:**
  ```rust
  let x: i8 = 5;
  let y: Option<i8> = Some(5);

  match y {
      None => None,
      Some(i) => Some(i + 1),
  }
  ```

- **Using `unwrap_or`:**
  ```rust
  let y: Option<i8> = None;
  let sum = x + y.unwrap_or(0); // Defaults to 0 if None
  ```

---

## Match Expressions
- **Exhaustive Matching:**
  ```rust
  enum Coin {
      Penny,
      Nickel,
      Dime,
      Quarter,
  }

  fn value_in_cents(coin: Coin) -> u8 {
      match coin {
          Coin::Penny => 1,
          Coin::Nickel => 5,
          Coin::Dime => 10,
          Coin::Quarter => 25,
      }
  }
  ```

---

### Binding to Values with Match
- **Example:**
  ```rust
  enum UsState {
      Alabama,
      Alaska,
      // ...
  }

  enum Coin {
      Penny,
      Nickel,
      Dime,
      Quarter(UsState),
  }

  fn value_in_cents(coin: Coin) -> u8 {
      match coin {
          Coin::Penny => 1,
          Coin::Nickel => 5,
          Coin::Dime => 10,
          Coin::Quarter(state) => {
              println!("State quarter from {:?}!", state);
              25
          },
      }
  }
  ```

---

## Combining Match with Option Enum
- **Function Example:**
  ```rust
  fn plus_one(x: Option<i32>) -> Option<i32> {
      match x {
          None => None,
          Some(i) => Some(i + 1),
      }
  }

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  ```

---

## If Let Syntax
- **Simplifying Match Statements:**
  ```rust
  let some_value = Some(3);

  if let Some(3) = some_value {
      println!("three");
  }
  ```

- **Syntax:** 
  - Less verbose for specific patterns.
  - Only specifies the pattern we care about, ignoring all other patterns.

---

## Conclusion
- **Chapter Summary:**
  - Enums
  - Pattern Matching
  - Option Enum
  - If-Let Syntax

