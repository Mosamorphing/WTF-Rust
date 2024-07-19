---
title: Errors
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust Minimalist Introduction: Error Handling

Handling potential errors and exceptions is crucial in any programming task. Rust provides powerful and flexible error handling mechanisms designed to write safer and more predictable code. This chapter will delve into error handling methods in Rust, including the `panic!` macro, the `Result` type, and how to propagate errors.

## 1. `panic!` Macro

In Rust, when a program encounters an unrecoverable error, it can call the `panic!` macro. This causes the program to clean up the stack of the current thread and immediately terminate.

### Example: Triggering Panic

```rust
fn main() {
    panic!("This is a critical error!");
}
```

This code will trigger a panic when executed, printing an error message and terminating the program. During early development, `panic!` provides quick feedback on errors, but in final product code, more refined error handling mechanisms are usually employed.

## 2. `Result` Type

Rust provides a standardized way to handle operations that may succeed or fail through the `Result` type. The `Result` type is a generic enum with two variants: `Ok` and `Err`, representing success and failure of an operation, respectively. The generic `T` represents the value returned on success, and `E` represents the error message on failure.
```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Example: Using `Result`

```rust
fn divide(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
    if divisor == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let result = divide(10.0, 0.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
```

In this example, the `divide` function attempts to perform a division operation, returning a `Result` type. If the divisor is not zero, it returns `Ok` with a `f64` value; otherwise, it returns `Err` with a string error message.

## 3. Error Propagation

When you want to return an error from a function so that the caller can decide how to handle it, Rust allows you to propagate errors easily using the `?` operator. This is a very elegant error handling mechanism: if an error occurs in a line marked with `?`, the current execution flow is terminated, and the error is returned to the caller. Otherwise, the program continues execution. Its behavior is similar to the following pseudocode:
```
if error {
    return errMsg 
} else {
    your normal business logic    
}
```

### Example: Error Propagation

```rust
fn get_data_from_file() -> Result<String, std::io::Error> {
    let path = "data.txt";
    let content = std::fs::read_to_string(path)?;

    Ok(content)
}

fn main() {
    match get_data_from_file() {
        Ok(data) => println!("File content: {}", data),
        Err(e) => println!("Failed to read file: {}", e),
    }
}
```

If the `read_to_string` call fails, the `?` operator immediately returns `Err` from the `get_data_from_file` function with the corresponding error message. If no error occurs at `?`, the program continues execution.

## 4. Custom Errors

For finer control over error handling, you can use custom types to represent errors.

### Example: Defining Custom Errors

```rust
use std::fmt;

#[derive(Debug)]
struct MyError {
    message: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}

fn risky_operation() -> Result<(), MyError> {
    if true { // Assume this is the result of some business logic
        Err(MyError { message: String::from("Something went wrong") })
    } else {
        Ok(())
    }
}
```

In this example, the `MyError` struct implements `fmt::Display`, allowing it to output error messages.

## Summary

This chapter introduced error handling mechanisms in Rust, including how to use `panic!` for urgent handling, how to use `Result` for recoverable errors, and how to propagate and define custom errors. Understanding and correctly applying these error handling techniques is essential for writing robust and maintainable Rust programs.
