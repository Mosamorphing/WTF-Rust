---
title: Trait
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust minimalist introduction: Trait

In Rust, a trait can be thought of as a collection of features. You can think of it as a way of defining a set of methods that can be implemented by multiple types. Traits are similar to interfaces in other languages, allowing you to define shared behavior. In this section we will have an in-depth understanding of the definition and use of traits, and how to use this feature to improve the modularity and reusability of code.

## Define Trait

Defining a trait in Rust is similar to defining an interface, where you declare methods that can be implemented by multiple data types. The syntax for defining a trait is as follows:

```rust
trait Describable {
    fn describe(&self) -> String;

    fn default_description(&self) -> String {
        String::from("This is a default function")
    }
}
```

In this example, we use the `trait` keyword to declare a trait. `Describeable` is the name of the trait, which contains two methods:
* `describe` is a method that must be provided by the type that implements this trait.
* `default_description` is a default method that does not require implementors to provide their own implementation (unless they want custom behavior).

## Implement Trait

Any type can implement one or more traits. This means you can add methods to custom types and even add new behaviors to types in the Rust standard library.

### Example: Define a structure and implement Trait

Here we define the structure `Person` and implement the `Describeable` trait for it.

```rust
struct Person {
    name: String,
    age: u32,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{}", alice.describe());
}
```
Here `impl Descriptable for Person` means that the `Person` structure implements the `Describable` trait. The `describe` method is now available on every `Person` instance.

## Use Trait as parameter

Traits can also be used as function parameters, which allows you to write more flexible and reusable code.

### impl Trait

`impl Trait` specifies that function parameters can be any type that implements a specific trait, suitable for simple cases.

```rust
fn output_description(item: &impl Describable) {
    println!("{}", item.describe());
}
```

The `output_description` function accepts any type that implements the `Describeable` trait. For the `item` parameter, we specify the `&impl` keyword and the name of the trait, which means you can pass a reference to any instance that implements the trait as a parameter. This approach avoids the transfer of ownership, so the same instance can be used multiple times.

### Trait Bounds

For situations where you accept multiple trait parameters, or have multiple constraints on generic parameters, using the trait bounds syntax can simplify your code, especially when the constraints become complex.

```rust
fn some_function<T: Describable + Clone>(item: T) {
    let item_copy = item.clone();
    println!("Description: {}", item.describe());
}
```

The function `some_function` takes a generic parameter `T` and specifies multiple trait bounds via the `T: Descriptable + Clone` syntax. This means that `T` must implement both the `Describable` and `Clone` traits. Using the `+` symbol to chain multiple traits is a Rust syntax

## Implement Trait as return type

You can specify the return type in a function signature using the `impl Trait` syntax, which allows the function to return any type that implements the specified trait.

```rust
fn return_description() -> impl Describable {
    Person {
        name: String::from("Lisa"),
        age: 18,
    }
}
```

The `return_description` function implements the returned `Describeable` trait type. In this example, we choose to return a `Person` instance.

## Inherit Trait

Through trait inheritance, we can build richer behaviors based on existing traits while ensuring that the functionality it depends on is also implemented.

```rust
trait Printable: Descriptable {
 fn print(&self) {
 println!("{}", self.describe());
 }
}

impl Printable for Person {}
```

In this example, `Printable: Describable` means that the `Printable` trait inherits from `Describable` and provides a default implementation of the `print` method. Any type that implements `Printable` must also implement `Describeable`.

## Summary

Traits are an extremely powerful part of Rust that allow you to define and implement certain behaviors that can be shared by multiple types. By learning how to define, implement, and use traits, you can write code that is more expressive, reusable, and flexible. Traits are also the basis for Rust polymorphism and interface design. By using traits appropriately, you can improve the abstraction level and maintainability of your code!
