## Thread

### 1. **Thread**
A unit of execution within a program that can run concurrently with other threads. In Rust, threads allow you to perform multiple tasks at the same time.

### 2. **Concurrency**
The ability of a program to make progress on multiple tasks simultaneously. Rust provides features to manage concurrency safely.

### 3. **Parallelism**
The simultaneous execution of multiple computations. While concurrency is about managing multiple tasks, parallelism is about executing them at the same time, often on multiple processors.

### 4. **std::thread**
Rust’s standard library module for handling threads. It provides functions and types to create and manage threads.

### 5. **spawn**
A function in the `std::thread` module used to create a new thread. It takes a closure or function to run in the new thread.

### 6. **Join Handle**
An object returned by the `spawn` function, which represents the handle to the newly created thread. It can be used to wait for the thread to finish executing.

### 7. **move**
A keyword used in closures to transfer ownership of variables from the outer environment into the closure, allowing those variables to be used safely within the thread.

### 8. **Arc (Atomic Reference Counting)**
A thread-safe reference-counting pointer used for shared ownership of data across multiple threads. It ensures that the data is not deallocated until all references are done using it.

### 9. **Mutex (Mutual Exclusion)**
A synchronization primitive that ensures that only one thread can access some data at any one time. It provides safe, exclusive access to shared data.

### 10. **lock**
A method on `Mutex` that acquires the lock, blocking the current thread until it can access the data. Once acquired, it returns a guard which releases the lock when it goes out of scope.

### 11. **Condvar (Condition Variable)**
A synchronization primitive for blocking a thread until a condition is met. It's used with a `Mutex` to wait for and signal conditions between threads.

### 12. **Send**
A Rust trait that indicates a type can be transferred safely to another thread. Types that implement `Send` can be moved into a new thread.

### 13. **Sync**
A Rust trait that indicates a type can be referenced from multiple threads safely. Types that implement `Sync` can be accessed by multiple threads simultaneously.

### 14. **race condition**
An undesirable situation that occurs when multiple threads access and modify shared data concurrently, leading to unpredictable results. Rust's ownership and type system help prevent race conditions.

### 15. **deadlock**
A situation where two or more threads are unable to proceed because each is waiting for the other to release a resource. Proper design and synchronization can prevent deadlocks.

Understanding these terms is fundamental when working with threads in Rust, as they provide the building blocks for safe and efficient concurrent programming.


------

Here are the crucial terms associated with threads in Rust, along with brief definitions:

1. **Native OS Threads**: These are threads created by the operating system, which Rust uses for its threading model. Each native OS thread has its own stack and local state.

2. **1:1 Threading Model**: This model, used by Rust, means that each language thread corresponds to one operating system thread. This approach ensures low runtime overhead but can be less efficient than other models.

3. **Thread-local Storage**: This is a method of storing data that each thread has its own copy of, without needing synchronization. It is created using the `thread_local` macro and can contain any value that is `'static` (no borrowed pointers).

4. **Thread Naming**: Threads can be named for identification purposes. This is done using the `Builder` and `Builder::name` methods. The thread name is useful for debugging, as it is printed in panic messages and provided to the OS where applicable.

5. **Stack Size**: The stack size for threads can be manually specified using the `Builder` and `Builder::stack_size` methods. The default stack size is platform-dependent and subject to change.

6. **Thread Synchronization**: This refers to the mechanisms used to coordinate and manage access to shared resources between threads. Rust provides various synchronization primitives, such as `Arc` and `Mutex`, to ensure thread safety.

7. **Atomic Reference Counting (Arc)**: `Arc` is a type that provides atomic reference counting, ensuring thread safety by guaranteeing that the reference count is modified in one logical operation. It is used to share data between threads safely.

8. **Channels**: Channels are a way to send data between threads. They consist of a sender and a receiver, allowing multiple threads to send data to a single receiver. This is useful for communication between threads.

9. **Thread Panic**: When a thread encounters a fatal logic error, it will unwind the stack, running destructors and freeing owned resources. This can be caught and recovered from using `catch_unwind` or resumed with `resume_unwind`.

10. **Thread Join**: This is a mechanism to wait for the termination of a thread. When the main thread of a Rust program terminates, all spawned threads are shut down, whether or not they have finished running.


---
11.  **Thread Safety**: Rust enforces thread safety at compile time through its ownership and type system, ensuring that data races are prevented. This means that you can write concurrent code without needing to use locks or other low-level synchronization primitives in many cases.

12.  **MutexGuard**: When a thread acquires a mutex, it receives a `` `MutexGuard` `` object, which represents the right to access the protected data. The `` `MutexGuard` `` is automatically dropped (and the mutex is released) when the guard goes out of scope, ensuring that the mutex is always properly unlocked.
    

    
13.  **Select**: The `` `select` `` macro in Rust is used for non-blocking operations on channels and other asynchronous operations. It allows a thread to wait on multiple operations and react as soon as one of them becomes ready, enabling more efficient and responsive concurrent programs.


#### **Green Threads (M:N Threading Model)**

-   **Definition**: Green threads refer to a threading model where multiple "green" threads are mapped onto a smaller number of operating system threads. This model is characterized by M green threads running on N OS threads, where M does not equal N. Rust's standard library does not use this model due to its focus on minimal runtime and compatibility with C, but it can be implemented using external crates. 
 