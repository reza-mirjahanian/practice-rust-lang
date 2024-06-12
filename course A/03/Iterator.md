
## **What is the Iterator Pattern?**

The iterator pattern allows you to iterate over a sequence of elements, regardless of how the elements are stored. For example, the elements could be stored in an array, a hash map, a graph, or some custom data structure that you create. Iterators encapsulate the logic for iterating over these different data structures, allowing you to iterate over various data structures in a uniform way.

## **Code Example: Creating an Iterator**

Let's create a variable called `v1` which is equal to a vector, and then create a variable called `v1_iter` which will be an iterator over the vector. We can create this iterator by calling `v1.iter()`. Note that in Rust, iterators are lazy, so when we create an iterator, nothing special happens until we actually use the iterator.

## **Using the Iterator in a For Loop**

Let's use our iterator in a for loop. We'll type in `for value in v1_iter`, and then we'll print out the value. Let's go ahead and run our program, and you can see we got our vector values printed out.

## **How Iterators Work**

All iterators in Rust implement the `Iterator` trait, which is defined in Rust's standard library. The `Iterator` trait looks something like this:

`trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}` 

Notice there's some new syntax here. We have a type called `Item`, and then we use our `Item` type inside our `Option` return value. This is called an associated type. We'll learn more about associated types in Chapter 19.

## **Defining a Test Function**

Let's define a test function to demonstrate how the `next` method works. Here we create a vector with three elements, and then we create a variable to store our iterator. Notice that the variable has to be mutable because we're going to call `next`, and `next` needs a mutable reference to the iterator.

## **Iterator Trait Methods**

The `Iterator` trait has various methods which have default implementations provided by the standard library. There are two broad categories of methods: adapters and consumers.

## **Adapters**

Adapters take in an iterator and return another iterator. For example, the `map` method takes in a closure and creates an iterator which calls the closure over each element in the sequence.

## **Consumers**

Consumers take in an iterator and return some other type, such as an integer, collection, or any other type. For example, the `sum` method repeatedly calls the `next` method to get each element in the sequence and then adds those elements up and returns the sum.