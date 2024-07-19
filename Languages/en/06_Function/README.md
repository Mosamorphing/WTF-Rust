### Title: 6. Function

Tags:
- Rust
- basic
- wtfacademy

---

Twitter: [@0xAA_Science](https://twitter.com/0xAA_Science)

Community: [Discord](https://discord.gg/5akcruXrsk) | [WeChat Group](https://docs.google.com/forms/d/e/1FAIpQLSe4KGT8Sh6sJ7hedQRuIYirOoZK_85miz3dw7vA1-YjodgJ-A/viewform?usp=sf_link) | [Official Website wtf.academy](https://wtf.academy)

All code and tutorials are open source on GitHub: [WTF-Rust](https://github.com/WTFAcademy/WTF-Rust)

# WTF Rust Simplified: Functions

In Rust, functions are units of code that perform specific tasks. Using functions enhances code reusability, simplifies complex tasks, and helps in modular code management.

## Function Definition

To define a function, use the `fn` keyword, followed by the function name, a parameter list, and the function body.

```rust
fn print_hello() {
    println!("Hello, world!");
}
```

## Functions with Parameters

Functions can accept parameters. You need to declare parameters and their types inside the parentheses following the function name.

```rust
fn print_sum(a: i32, b: i32) {
    println!("Sum is: {}", a + b);
}
```

## Return Values

Functions can return values. The return type must be specified after the arrow `->`. In Rust, the last expression in a function is used as the return value, or you can explicitly return a value using the `return` keyword. Note that you should not use a semicolon at the end of the expression; otherwise, it will be treated as a statement, not an expression.

```rust
fn add_two(a: i32) -> i32 {
    a + 2
    // or `return a + 2;`
}
```

## Control Flow

Rust provides several ways to control the flow of program execution, including `if` expressions and loops (`loop`, `while`, `for`).

### If Expressions

`if` expressions let you execute different code branches based on conditions.

```rust
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}

let condition = true;
let number = if condition { 5 } else { 6 };
println!("The value of number is: {}", number);
```

### Loops

Rust offers several looping constructs: `loop`, `while`, and `for`.

- **Loop**

The `loop` keyword creates an infinite loop.

```rust
loop {
    println!("again!");
    break; // Infinite loop, but we break out immediately here
}
```

- **While**

The `while` loop executes as long as its condition is true.

```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}
println!("LIFTOFF!!!");
```

- **For**

The `for` loop is used to iterate over a collection, such as an array or range.

```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}
```

- **Loop Control**
  - `break`: Immediately exits the loop.
  - `continue`: Skips the remaining part of the current iteration.

```rust
for number in 1..10 {
    if number % 2 == 0 {
        continue;
    }
    println!("Found an odd number: {}", number);
    if number == 7 {
        break;
    }
}
```

By understanding function definitions and usage, as well as mastering Rust's control flow mechanisms, you will be able to write clear, logical Rust programs. Functions and control flow are fundamental building blocks in Rust, and mastering them will greatly enhance your Rust programming skills.
