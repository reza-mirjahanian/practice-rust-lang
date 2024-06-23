


### Type aliases 
1. Basic Type Alias

```rust
type Age = u32;

fn print_age(age: Age) {
    println!("Age is: {}", age);
}

fn main() {
    let my_age: Age = 30;
    print_age(my_age);
}
```

This example demonstrates a simple type alias for `u32`.

2. Type Alias with Generic Types

```rust
use std::collections::HashMap;

type Cache<T> = HashMap<String, T>;

fn insert_into_cache<T>(cache: &mut Cache<T>, key: &str, value: T) {
    cache.insert(key.to_string(), value);
}

fn main() {
    let mut string_cache: Cache<String> = Cache::new();
    insert_into_cache(&mut string_cache, "key1", "value1".to_string());
    
    println!("Cache: {:?}", string_cache);
}
```

This example shows how to create a type alias with generic types.

3. Function Pointer Type Alias

```rust
type MathOperation = fn(i32, i32) -> i32;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn apply_operation(a: i32, b: i32, op: MathOperation) -> i32 {
    op(a, b)
}

fn main() {
    println!("10 + 5 = {}", apply_operation(10, 5, add));
    println!("10 - 5 = {}", apply_operation(10, 5, subtract));
}
```

This example demonstrates using a type alias for function pointers.

4. Type Alias for Complex Types

```rust
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

type SharedQueue<T> = Arc<Mutex<VecDeque<T>>>;

fn add_to_queue<T>(queue: &SharedQueue<T>, item: T) {
    let mut q = queue.lock().unwrap();
    q.push_back(item);
}

fn main() {
    let queue: SharedQueue<i32> = Arc::new(Mutex::new(VecDeque::new()));
    add_to_queue(&queue, 42);
    
    println!("Queue: {:?}", queue.lock().unwrap());
}
```

This example shows how type aliases can simplify complex type declarations.

5. Using a Third-Party Library: `tokio`

Let's use the `tokio` crate to demonstrate type aliases with asynchronous programming:

```rust
use tokio::sync::mpsc;

type Sender<T> = mpsc::Sender<T>;
type Receiver<T> = mpsc::Receiver<T>;

#[tokio::main]
async fn main() {
    let (tx, mut rx): (Sender<String>, Receiver<String>) = mpsc::channel(100);

    tokio::spawn(async move {
        tx.send("Hello from task!".to_string()).await.unwrap();
    });

    if let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
}
```

In this example, we create type aliases for Tokio's channel types, simplifying the channel creation syntax.

Key points about type aliases:

1. They don't create new types; they're just alternate names for existing types.
2. They can help improve code readability and reduce repetition.
3. They're often used to create more descriptive or domain-specific type names.
4. Type aliases can include generic parameters.
5. They can be especially useful when working with complex types or external libraries.

