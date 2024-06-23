

### Higher-Order Functions

Higher-order functions are functions that take other functions as arguments or return them as results.

#### Example with Standard Library

```rust
fn apply_function<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

fn main() {
    let square = |x: i32| x * x;
    let double = |x: i32| x * 2;

    let result1 = apply_function(square, 5);
    let result2 = apply_function(double, 5);

    println!("Square: {}", result1); // Output: Square: 25
    println!("Double: {}", result2); // Output: Double: 10
}
```

In this example:
- `apply_function` is a higher-order function that takes a function `f` and a value `value`, applying `f` to `value`.
- The `square` and `double` closures are passed to `apply_function`.

### Closures Capturing Their Environment

Closures can capture variables from their environment, which can be done in different ways: by borrowing immutably, borrowing mutably, or by taking ownership.

#### Example with Standard Library

```rust
fn main() {
    let mut num = 5;

    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }

    println!("Num: {}", num); // Output: Num: 10
}
```

In this example:
- The closure `add_num` captures `num` mutably, allowing it to modify `num`.

### Using Third-Party Library

#### Example with `rayon` for Parallel Iteration

The `rayon` crate is used for data parallelism, allowing you to parallelize operations on iterators. Add the dependency to your `Cargo.toml` file:

```toml
[dependencies]
rayon = "1.6"
```

Now, let's create an example that uses `rayon` for parallel iteration.

```rust
use rayon::prelude::*;

fn main() {
    let numbers: Vec<i32> = (1..=100).collect();

    let sum: i32 = numbers.par_iter().map(|&x| x * 2).sum();

    println!("Sum: {}", sum); // Output: Sum: 10100
}
```

In this example:
- `rayon::prelude::*` is imported to use parallel iterators.
- `par_iter` is used to create a parallel iterator.
- The `map` operation is applied in parallel, followed by `sum` to aggregate the results.

### Returning Closures

Functions can return closures, but the type must be specified using `impl Fn`.

#### Example with Standard Library

```rust
fn create_multiplier(multiplier: i32) -> impl Fn(i32) -> i32 {
    move |x| x * multiplier
}

fn main() {
    let multiply_by_2 = create_multiplier(2);
    let multiply_by_3 = create_multiplier(3);

    println!("3 * 2 = {}", multiply_by_2(3)); // Output: 3 * 2 = 6
    println!("3 * 3 = {}", multiply_by_3(3)); // Output: 3 * 3 = 9
}
```

In this example:
- `create_multiplier` returns a closure that multiplies its input by `multiplier`.
- The `move` keyword ensures the closure captures `multiplier` by value.

### Explanation

1. **Higher-Order Functions**:
   - Functions that take other functions as arguments or return them as results.
   - Enable functional programming techniques like map, filter, and reduce.

2. **Closures Capturing Their Environment**:
   - Closures can capture variables from their environment.
   - Capturing can be by borrowing (immutably or mutably) or by taking ownership.

3. **Third-Party Library (`rayon`)**:
   - Used for data parallelism with parallel iterators.
   - Enables parallel processing of collections.

4. **Returning Closures**:
   - Functions can return closures with the `impl Fn` syntax.
   - Useful for creating customized functions dynamically.

\