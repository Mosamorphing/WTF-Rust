---
title: 8. Collection
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust Minimalist Introduction: Collections

Twitter: [@0xAA_Science](https://twitter.com/0xAA_Science)

Community: [Discord](https://discord.gg/5akcruXrsk) | [WeChat Group](https://docs.google.com/forms/d/e/1FAIpQLSe4KGT8Sh6sJ7hedQRuIYirOoZK_85miz3dw7vA1-YjodgJ-A/viewform?usp=sf_link) | [Official Website wtf.academy](https://wtf.academy)

All code and tutorials are open-sourced on GitHub: [WTF-Rust](https://github.com/WTFAcademy/WTF-Rust)

This chapter focuses on collection types in Rust, which are data structures frequently used in development. We will explore `Vector`, strings, and `HashMap`, understanding how to use them and their respective features. These collection types provide various flexible ways to store and access datasets.

## 1. Using Vector

A `Vector` (often abbreviated as `vec`) is a growable array that can store values of the same type. Unlike arrays, the size of a vector can change at runtime.

### Creating and Using Vectors

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();  // Create an empty vector
    v.push(5);                        // Add elements to the vector
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector: {:?}", v);       // Print the vector
}
```

You can also quickly initialize a vector using the `vec![]` macro:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];       // Create and initialize a vector using a macro
    println!("Vector: {:?}", v);
}
```

Iterating over a vector:

```rust
for i in &v {
    println!("{}", i);
}
```

## 2. String Handling

In Rust, there are two common string types: `String` and `&str`. `String` is a growable, mutable, owned, UTF-8 encoded string type, while `&str` is a slice pointing to a valid UTF-8 sequence. `&str` is typically used in function parameters, while `String` is used when ownership or mutability is required, such as when modifying a string.

### Creating and Modifying Strings

```rust
fn main() {
    let mut s = String::new();        // Create an empty string
    s.push_str("Hello");              // Add text
    s.push(' ');                      // Add a single character
    s.push_str("world!");

    println!("String: {}", s);
}
```

Concatenating strings:

```rust
let s1 = "Hello".to_string();
let s2 = " World!";
let s3 = s1 + s2;  // Note: s1 is moved here and can no longer be used

println!("Combined string: {}", s3);
```

### Strings and Slices

```rust
fn main() {
    let s1: &str = "Hello, world!"; // The type of a string literal is &str
    let s2: &str = &s1[0..5]; // Create a substring using slice syntax
    println!("s1: {}", s1);
    println!("s2: {}", s2);

    let s3: String = s1.to_string(); // Convert &str to String
    println!("s3: {}", s3);

    print_str(s1); // Pass &str to a function
}

fn print_str(s: &str) {
    println!("Inside print_str: {}", s);
}
```

## 3. HashMap

A `HashMap` is used to store key-value pairs where the keys and values can be of different types. This is a very useful data structure for quickly accessing and managing associated data.

### Creating and Using a HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    if let Some(score) = scores.get("Blue") {
        println!("Blue team's score: {}", score);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

A HashMap provides faster data access speeds in linear table searches, especially efficient for large datasets.

## Summary

This chapter introduced Rust's capabilities in handling different collection types. Understanding and mastering these collection operations will greatly enhance your flexibility and efficiency when dealing with datasets. Using collection types appropriately is crucial for writing efficient and maintainable code in everyday programming challenges. We hope this chapter helps you take a solid step in your Rust journey.
