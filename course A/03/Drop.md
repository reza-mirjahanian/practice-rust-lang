
**The Drop Trait**

-   The Drop trait can be implemented on any type and allows you to customize what happens when a value goes out of scope.
-   The Drop trait is almost always used when implementing smart pointers.
-   For example, with the Box smart pointer, the custom behavior we want when a Box goes out of scope is to deallocate the data stored on the heap.

**Automatic Cleanup**

-   In some languages, you have to manually free memory or resources when you're done using an instance of a smart pointer.
-   But with the Drop trait, this cleanup happens automatically when a value goes out of scope.
-   You don't have to worry about manually cleaning up resources.

**Example Code**

-   We have a struct called `CustomSmartPointer` with one field called `data`, which is a string.
-   We implement the Drop trait for our `CustomSmartPointer` struct.
-   The Drop trait requires that we implement one method called `drop`, which takes a mutable reference to `self`.
-   Inside the `drop` method, we call our cleanup code. In this case, we simply print out "Dropping CustomSmartPointer with data" and then the data stored in our smart pointer.

**Important Notes**

-   The Drop trait is included in the prelude, so it's already in scope.
-   Variables will be dropped in the reverse order of their creation.
-   Rust will automatically call the `drop` method when a value goes out of scope.

**Customizing Cleanup Behavior**

-   In most cases, this isn't necessary, but in some cases, you might want to clean up a value early.
-   One example is when using smart pointers to manage locks. You might want to call the `drop` method to release a lock.
-   Rust doesn't allow you to call the `drop` method directly because when our variable goes out of scope, Rust will still automatically call the `drop` method, which will result in a double free.

**Manually Cleaning Up a Value**

-   Instead of calling the `drop` method on a value, we can call the `drop` function provided by the Rust standard library and pass in our value.
-   This `drop` function is different from the `drop` method on our custom smart pointer struct.
-   This `drop` function is provided by the Rust standard library and is included in the prelude, so we don't have to bring it into scope manually.