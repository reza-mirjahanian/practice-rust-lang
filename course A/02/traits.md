

## Defining a Trait

-   Traits allow us to define a set of methods that are shared across different types.
-   Example:  `` `trait Summary { fn summarize(&self) -> String; }` ``

## Implementing a Trait

-   To implement a trait for a type, we use the  `` `impl` ``  keyword followed by the trait, the type, and the method implementation.
-   Example:  `` `impl Summary for NewsArticle { fn summarize(&self) -> String { // ... } }` ``

## Default Implementations

-   Traits can have default implementations for methods, which are used if a type doesn't provide its own implementation.
-   Example:  `` `trait Summary { fn summarize(&self) -> String { String::from("Default implementation") } }` ``

## Trait Bounds

-   Trait bounds allow us to specify that a generic type parameter must implement a certain trait.
-   Example:  `` `fn notify<T: Summary>(item: &T) { println!("Breaking news: {}", item.summarize()); }` ``

## Where Clause

-   The  `` `where` ``  clause allows us to specify trait bounds in a more readable way, especially when there are multiple bounds.
-   Example:  `` `fn sum_function<T, U>(t: T, u: U) -> T where T: Display + Clone, U: Clone + Debug { // ... }` ``

## Return Types with Trait Bounds

-   Return types can also have trait bounds, allowing us to return any type that implements a certain trait.
-   Example:  `` `fn returns_summarizable() -> impl Summary { Tweet { username: String::from("bogdan"), content: String::from("Just learned about traits!"), reply: false, retweet: false, } }` ``

## Conditionally Implementing Methods

-   Trait bounds can be used to conditionally implement methods based on the types that implement the trait.
-   Example: `struct Pair<T>  { x: T, y: T, }

impl<T> Pair<T> { fn new(x: T, y: T) -> Self { Self { x, y } } }

impl<T: Display + PartialOrd> Pair<T> { fn compare_display(&self) { if self.x > self.y { println!("The first element is greater: {}", self.x); } else { println!("The second element is greater: {}", self.y); } } }`

## Blanket Implementations

-   Blanket implementations allow us to implement a trait for any type that implements another trait.
-   Example: `impl<T>  ToString for T where T: Display {}

## Conclusion

-   Traits, trait bounds, and default implementations provide powerful ways to define and share behavior across multiple types in Rust.
-   Understanding these concepts is essential for writing flexible and reusable code.
-   In the next video, we'll explore lifetimes, one of the more challenging topics in Rust.