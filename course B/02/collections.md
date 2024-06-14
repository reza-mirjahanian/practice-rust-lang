
### collections

### 1. `Vec` (Vector)

A `Vec` is a growable array type, very useful for working with lists of items.

**Creating and Initializing**:
```rust
let mut vec: Vec<i32> = Vec::new();
vec.push(1);
vec.push(2);
vec.push(3);

// Using the vec! macro
let vec = vec![1, 2, 3];
```

**Accessing Elements**:
```rust
let first = vec[0];
if let Some(second) = vec.get(1) {
    println!("Second element: {}", second);
}
```

methods of accessing a value in a vector, with indexing syntax and the  `get`  method.

```rust
 let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```

Listing 8-4: Using indexing syntax or the  `get`  method to access an item in a vector

Note a few details here. We use the index value of  `2`  to get the third element because vectors are indexed by number, starting at zero. Using  `&`  and  `[]`  gives us a reference to the element at the index value. When we use the  `get`  method with the index passed as an argument, we get an  `Option<&T>`  that we can use with  `match`.


**Iterating**:
```rust
for val in &vec {
    println!("{}", val);
}

// Mutable iteration
for val in &mut vec {
    *val += 1;
}
```

**Common Operations**:
```rust
let len = vec.len();
let is_empty = vec.is_empty();
vec.sort();
vec.reverse();
```

### 2. `String`

A `String` is a growable, UTF-8 encoded string type.

**Creating and Initializing**:
```rust
let mut s = String::new();
s.push_str("Hello");
s.push(' ');

// Using the to_string method
let s = "Hello".to_string();

// Using the String::from function
let s = String::from("Hello");
```

**Concatenation**:
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used
```

**Formatting**:
```rust
let s = format!("{}-{}", "Hello", "world");
```

**Slicing**:
```rust
let hello = &s[0..5];
```

### 3. `HashMap` and `BTreeMap`

`HashMap` and `BTreeMap` are key-value storage collections.

**Creating and Initializing**:
```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key1", "value1");
map.insert("key2", "value2");
```

**Accessing Values**:
```rust
if let Some(value) = map.get("key1") {
    println!("Value: {}", value);
}

// Using entry API to update values
map.entry("key1").or_insert("default");
```

**Iterating**:
```rust
for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

**Using `BTreeMap` for Sorted Keys**:
```rust
use std::collections::BTreeMap;

let mut btree_map = BTreeMap::new();
btree_map.insert("key1", "value1");
btree_map.insert("key2", "value2");
```

### 4. `HashSet` and `BTreeSet`

`HashSet` and `BTreeSet` are collections of unique values.

**Creating and Initializing**:
```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert(1);
set.insert(2);
```

**Checking Membership**:
```rust
if set.contains(&1) {
    println!("Set contains 1");
}
```

**Set Operations**:
```rust
let set1: HashSet<_> = [1, 2, 3].iter().cloned().collect();
let set2: HashSet<_> = [2, 3, 4].iter().cloned().collect();

let intersection: HashSet<_> = set1.intersection(&set2).collect();
let difference: HashSet<_> = set1.difference(&set2).collect();
let union: HashSet<_> = set1.union(&set2).collect();
```

**Using `BTreeSet` for Sorted Values**:
```rust
use std::collections::BTreeSet;

let mut btree_set = BTreeSet::new();
btree_set.insert(1);
btree_set.insert(2);
```

### 5. `VecDeque`

A double-ended queue that allows for efficient push and pop operations at both ends.

**Creating and Initializing**:
```rust
use std::collections::VecDeque;

let mut deque: VecDeque<i32> = VecDeque::new();
deque.push_back(1);
deque.push_front(0);
```

**Accessing Elements**:
```rust
let first = deque.front();
let last = deque.back();
```

### 6. `LinkedList`

A doubly linked list.

**Creating and Initializing**:
```rust
use std::collections::LinkedList;

let mut list: LinkedList<i32> = LinkedList::new();
list.push_back(1);
list.push_front(0);
```

**Iterating**:
```rust
for val in &list {
    println!("{}", val);
}
```

### 7. Using Iterators

Iterators provide a powerful and expressive way to work with collections.

**Basic Usage**:
```rust
let vec = vec![1, 2, 3, 4, 5];
let sum: i32 = vec.iter().sum();
println!("Sum: {}", sum);

let filtered: Vec<i32> = vec.iter().filter(|&&x| x % 2 == 0).cloned().collect();
```

**Chaining Methods**:
```rust
let vec = vec![1, 2, 3, 4, 5];
let result: Vec<i32> = vec.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * 2)
    .collect();
```

### 8. Using `Option` and `Result` with Collections

Handling operations that might fail or be optional.

**Option**:
```rust
let vec = vec![1, 2, 3];
if let Some(first) = vec.first() {
    println!("First element: {}", first);
}
```

**Result**:
```rust
use std::collections::HashMap;

fn get_value(map: &HashMap<String, String>, key: &str) -> Result<&String, &str> {
    map.get(key).ok_or("Key not found")
}

let mut map = HashMap::new();
map.insert("key1".to_string(), "value1".to_string());
match get_value(&map, "key1") {
    Ok(value) => println!("Value: {}", value),
    Err(err) => println!("Error: {}", err),
}
```

### 9. Memory Management

Be mindful of memory usage and performance. Use `shrink_to_fit` to reduce memory usage if necessary.

```rust
let mut vec = vec![1, 2, 3];
vec.shrink_to_fit();
```

### Summary

Rust's collections are powerful and versatile, offering a wide range of functionalities. Here are the key points:

1. **`Vec` for Dynamic Arrays**: Use `Vec` for a dynamic list of items.
2. **`String` for Text**: Manage and manipulate UTF-8 encoded text.
3. **`HashMap` and `BTreeMap` for Key-Value Pairs**: Store key-value pairs with different sorting behaviors.
4. **`HashSet` and `BTreeSet` for Unique Values**: Manage collections of unique items.
5. **`VecDeque` for Double-Ended Queue**: Efficient push and pop operations at both ends.
6. **`LinkedList` for Doubly Linked List**: Use when you need frequent insertions and deletions in the middle of the list.
7. **Iterators for Powerful Data Manipulation**: Chain methods for expressive and efficient data handling.
8. **`Option` and `Result` for Safe Operations**: Handle operations that might fail or be optional safely.
9. **Memory Management**: Be conscious of memory usage and optimize when needed.

---------------

The problem in this code arises from the way Rust handles borrowing and mutability. Here’s the code you provided:

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];  // Immutable borrow of v

v.push(6);  // Mutable borrow of v

println!("The first element is: {first}");
```

In Rust, you cannot have a mutable borrow (`v.push(6)`) while there is an active immutable borrow (`let first = &v[0]`). The immutable borrow of `v` as `first` is still in scope when you try to push a new element to `v`, which requires a mutable borrow. This leads to a compilation error due to Rust's strict borrowing rules, ensuring memory safety.

### Solution

You can solve this problem by ensuring the immutable borrow is no longer in use before the mutable borrow occurs. This can be done by reordering the operations so that the push happens before you borrow `v` immutably:

```rust
let mut v = vec![1, 2, 3, 4, 5];

v.push(6);  // Mutable borrow of v

let first = &v[0];  // Immutable borrow of v

println!("The first element is: {first}");
```

Or, if you need to borrow `v` immutably before pushing, you should ensure the immutable borrow goes out of scope before the mutable borrow:

```rust
let mut v = vec![1, 2, 3, 4, 5];

{
    let first = &v[0];  // Immutable borrow of v
    println!("The first element is: {first}");
}  // Immutable borrow goes out of scope here

v.push(6);  // Mutable borrow of v
```

