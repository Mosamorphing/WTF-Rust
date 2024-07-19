---
title: 7. Test
tags:
- Rust
- basic
- test
---
# WTF Rust Minimalist Introduction: Rust Testing

Twitter: [@0xAA_Science](https://twitter.com/0xAA_Science)

Community: [Discord](https://discord.gg/5akcruXrsk)｜[WeChat Group](https://docs.google.com/forms/d/e/1FAIpQLSe4KGT8Sh6sJ7hedQRuIYirOoZK_85miz3dw7vA1-YjodgJ-A/viewform?usp=sf_link)｜[Official website wtf.academy](https://wtf.academy)

All code and tutorials are open source on github: [WTF-Rust](https://github.com/WTFAcademy/WTF-Rust)

## Introduction

Author: [Eta](https://twitter.com/pwhattie) Translator: [Mosa](https://twitter.com/mofasasi)

This article mainly introduces the functions, commands and types of Rust tests, which are full of useful information.

- Test functions: Commonly used macros, attributes and enumerations in tests, including `#[test]`, `#[cfg(test)]`, `panic!`, `assert!`, `assert_eq!`, `assert_ne!`, `should_panic`, `Result<T,E>`.

- Test command `cargo test`: Run tests in parallel or continuously `--test-threads`; display the output of the function `--show-output`, run a single test, multiple tests `cargo test function name` `cargo test --test file name`; ignore some tests `#[ignore]`, `--ignored`.

- Test types: unit tests (testing private functions), integration tests (tests directories, shared submodules, binary crates)

## 1. Test functions

Test functions are functions annotated with the `test` attribute. When running tests with the `cargo test` command, functions marked with the `test` attribute are called. When creating a new library project with Cargo, it automatically generates a test module and a test function for us. The `tests` module can also include non-test functions (functions without the test attribute) to perform common operations. You can add any number of test modules or functions.

<details><summary>Tips: Install Rust</summary>
Install rustup on Linux or macOS

```rust
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

On Windows, visit the installation page and follow the instructions to install Rust.

Update and uninstall, and check if Rust is installed correctly

```rust
$ rustup update
$ rustup self uninstall
$ rustc --version
```

Install Rust, see the [Installation Guide](https://doc.rust-lang.org/book/ch01-01-installation.html);

</details>

<details><summary>Tips: Rust Attributes</summary>
Rust attributes are metadata about Rust code snippets. They do not change the logic of the code they modify, but only modify or annotate the code, similar to annotations or metadata in other programming languages, including internal attributes `#![Attr]` and external attributes `#[Attr]`.
According to the purpose, they can be divided into: conditional compilation attributes, such as `#[cfg]` and `#[cfg_attr]`; attributes for crates, such as `#![no_std]`; attributes for functions and modules, such as `#[test]` used to mark test functions, automatically implement the Debug trait, that is, the attribute `#[derive(Debug)]` that prints out debugging information; and attributes for configuring external tools rustfmt and clippy.

For more information, see the Attributes section of the [Rust Reference Manual](https://rustwiki.org/en/reference/attributes.html).

</details>

### 1.1 Create a new library project `adder`

```rust
$ cargo new adder --lib
Created library `adder` project
$ cd adder
```

File name: src/lib.rs

```rust
#[cfg(test)]
mod tests {
#[test]
fn exploration() {
assert_eq!(2 + 2, 4);
}
}
```

The `cargo test` command builds a Test Runner executable that runs all the test functions in the project. The name of the test function is `exploration` and the result of running the test is `ok`. `1 passed; 0 failed; 0 ignored; 0 filtered out;`  indicates the number of passed, failed, ignored and filtered tests, `0 measured`  is for [performance tests](https://doc.rust-lang.org/unstable-book/library-features/test.html), `Doc-tests adder`  is the test result of all documentation comments, ensuring that the documentation and actual code are in sync.

```solidity
$ cargo test
Compiling adder v0.1.0 (file:///projects/adder)
Finished dev [unoptimized + debuginfo] target(s) in 0.22 secs
Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

> Note: All the following sample codes need to be expanded to view

### 1.2 Commonly used macros, properties and enumerations in testing

#### 1.2.1 `panic!` macro

Test failure example: Test function triggers panic The test will fail when the main thread detects an exception in the test thread, and the simplest way to trigger a panic is to use the `panic!` macro. Each test runs in a new thread. When the main thread finds that the test thread is abnormal, it marks the corresponding test as failed, lists the detailed reasons for the test failure, and lists all failed tests.

<details><summary>Click here to expand the sample code</summary>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

```

```rust
running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:10:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed

```

</details>

#### 1.2.2 Assert macros: `assert!`, `assert_eq!`  and  `assert_ne!` check if the code returns the correct value as expected

- `assert!` macro
The standard library's assertion `assert!`  macro is used to check the code, and the parameter is a Boolean value. If the value is true, the test passes. If the value is false, `assert!`  will call  `panic!`  macro, and the test fails. Use glob to globally import all the contents of the external module `use super::*;`, so that all the contents defined in the external module can be used in the  `tests`  module.

<details><summary>Click me to expand the sample code</summary>

  ```rust
  #[derive(Debug)]
  pub struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
      pub fn can_hold(&self, other: &Rectangle) -> bool {
          self.width < other.width && self.height > other.height
      }
  }

  ```

  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn larger_can_hold_smaller() {
          let larger = Rectangle { width: 8, height: 7 };
          let smaller = Rectangle { width: 5, height: 1 };

          assert!(larger.can_hold(&smaller));
      }

      #[test]
      fn smaller_cannot_hold_larger() {
          let larger = Rectangle { width: 8, height: 7 };
          let smaller = Rectangle { width: 5, height: 1 };

          assert!(!smaller.can_hold(&larger));
      }
  }
  ```
  ```rust
  running 2 tests
  test tests::smaller_cannot_hold_larger ... ok
  test tests::larger_can_hold_smaller ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  ```

  </details>

- `assert_eq!`  and  `assert_ne!`
`assert_eq!`  and  `assert_ne!` are used to check if the value of the test code is equal to the expected value, which is equivalent to the `assert!` macro with the `==`  or `!=` operator as the argument. The difference is that they will print the two specific values ​​when the assertion fails, while  `assert!`  will only print  `false`  values. The `assert_eq!`  macro passes if the two values ​​passed to it are equal and fails if they are not equal, while the `assert_ne!`  macro does the opposite.
Note: In some languages ​​and testing frameworks, the arguments of the function that asserts that two values ​​are equal are called  `expected`  and  `actual`, and the order in which the arguments are specified is important. In Rust, however, they are called  `left`  and  `right`, and the order in which the arguments are specified is not important.
The `assert_eq!`  and  `assert_ne!`  macros use  `==`  and  `!=`  under the hood, respectively. When the assertion fails, these macros print their arguments in debug format, which means that the values ​​being compared must implement the  `PartialEq`  and  `Debug` traits. All primitive types and most standard library types implement these two derived traits. Custom structs and enumerations that don’t implement these traits need to be annotated  `#[derive(PartialEq, Debug)]`  , see  [“Derivable traits”](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) .

<details><summary>Click here to expand the sample code</summary>

  ```rust
  pub fn add_two(a: i32) -> i32 {
      a + 2
  }

  #[cfg(test)]
  mod tests {
  use super::\*;

      #[test]
      fn it_adds_two() {
          assert_eq!(4, add_two(2));
      }

  }

  ```

  ```rust
  running 1 test
  test tests::it_adds_two ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  ```

</details>

- Custom failure message

Custom messages can be passed as arguments to `assert!`, `assert_eq!`  and  `assert_ne!`. The size of a `String`  can be increased and its contents can be changed, using the  `+`  operator or the  `format!`  macro to concatenate  `String`  values. Add a custom failure message argument to the test function: the format string with `{}` placeholders, and the value of the  `greeting`  function.

<details><summary>Click here to expand the sample code</summary>

  ```rust
  pub fn greeting(name: &str) -> String {
      format!("Hello {}!", name)
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn greeting_contains_name() {
          let result = greeting("Carol");
          assert!(result.contains("Carol"));
      }
  }
  ```

  ```rust
  pub fn greeting(_name: &str) -> String {
      String::from("Hello!")
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn greeting_contains_name() {
          let result = greeting("Carol");
      assert!(
          result.contains("Carol"),
          "Greeting did not contain name, value was `{}`", result
          );
      }
  }

  ```
  ```rust
  ---- tests::greeting_contains_name stdout ----
  thread 'tests::greeting_contains_name' panicked at 'Greeting did not
  contain name, value was `Hello!`', src/lib.rs:12:9
  note: Run with `RUST_BACKTRACE=1` for a backtrace.

  ```
</details>

#### 1.2.3 `should_panic` attribute

Checks whether the code handles errors as expected. This attribute passes if the code in the function panics, and fails if it does not. The `should_panic` test result simply tells us that the code panicked, and it will pass even if the code panicked for reasons we did not expect. The optional `expected` parameter makes the `should_panic` test result more precise, ensuring that the error message contains the content it provides, that is, if a panic is triggered but the panic text does not contain the content in the expected parameter, the test will still fail.

<details><summary>Click me to expand the sample code</summary>

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

```

```rust
$ cargo test
   Compiling adder v0.1.0 (/Users/panwei/Desktop/code/Youtube/Rust/projects/11test/project/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.32s
     Running unittests src/lib.rs (target/debug/deps/adder-0980e06b8dfc4e08)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

```rust
// --snip--

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(0);
    }
}

```

```rust
$ cargo test
   Compiling adder v0.1.0 (/Users/panwei/Desktop/code/Youtube/Rust/projects/11test/project/adder)
    Finished test [unoptimized + debuginfo] target(s) in 6.26s
     Running unittests src/lib.rs (target/debug/deps/adder-0980e06b8dfc4e08)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
thread 'tests::greater_than_100' panicked at src/lib.rs:9:13:
Guess value must be greater than or equal to 1, got 0.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Guess value must be greater than or equal to 1, got 0."`,
 expected substring: `"Guess value must be less than or equal to 100"`

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

  </details>

#### 1.2.4 `Result<T,E>` enumeration

The previous test failures all triggered panic. You can also use `Result<T,E>` as the return type of the test function to achieve the purpose of test failure. Unlike calling the `assert_eq!` macro, `Result<T,E>` returns Ok if the test passes and Err if the test fails. You cannot use the `#[should_panic]` annotation in tests using `Result<T, E>`, because Err will be returned when the test fails, and panic will not be triggered.

<details><summary>Click me to expand the sample code</summary>

```rust

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

```

```rust

$ cargo test
Compiling adder v0.1.0 (/Users/panwei/Desktop/code/Youtube/Rust/projects/11test/project/adder)
    Finished test [unoptimized + debuginfo] target(s) in 1.46s
    Running unittests src/lib.rs (target/debug/deps/adder-0980e06b8dfc4e08)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

</details>

## 2. Controlling the running of tests: arguments to the test command `cargo test`

`cargo run` compiles the code and runs the resulting binary, `cargo test` compiles the code in test mode and runs the resulting test binary. You can control the running of `cargo test` tests by adding command line arguments, passing some of the command line arguments to  `cargo test`, followed by a separator  `--`, and the rest of the command line arguments to the resulting test binary. Running `cargo test --help`  will show you the arguments to  `cargo test` , while running  `cargo test -- --help`  will show you the arguments to use after the separator  `--` .

<details><summary>Click here to expand the sample code</summary>

```rust
$ cargo test --help
$ cargo test -- --help
```

</details>

### 2.1 Running tests in parallel or serially: `--test-threads`

When running multiple tests, Rust uses threads by default to run them in parallel, because it runs faster. Running in parallel requires making sure that each test reads and writes different files, and that the tests do not depend on each other or on any shared state, otherwise one test might modify a file while another test is reading or writing it, causing each test to interfere with the other. If you do not want tests to run in parallel, or want more precise control over the number of threads, you can pass the `--test-threads` argument and the number of threads you want to use to the test binary.

<details><summary>Click here to expand the sample code</summary>

```rust
$ cargo test -- --test-threads=1
```

</details>

### 2.2 Showing the output of a function: `--show-output`

Rust captures (does not display) all output by default, for example, the output of macros like `println!` is not displayed when the test passes, and all standard output and other error messages are displayed when the test fails. Tell Rust to show the output of a passing test by adding the `--show-output`  parameter to the end.

<details><summary>Click here to expand the sample code</summary>

```rust
$ cargo test -- --test-threads=1
```

</details>

### 2.2 Showing function output: `--show-output`

Rust captures (does not display) all output by default, for example, the output of macros like `println!` is not displayed when the test passes, and all standard output and other error messages are displayed when the test fails. Tell Rust to show the output of the passed test by adding the `--show-output`  parameter to the end.

<details><summary>Click me to expand the sample code</summary>

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

```rust
$ cargo test -- --show-output
```

</details>

### 2.3 Run some tests: single test, multiple tests

Choose which tests to run by passing the test name to cargo test.

Run a single test: Pass any test name to cargo test to run only that test, 2 filtered out means 2 tests were filtered out.

Run multiple tests: Specify the name of some tests, any test matching that name will be run, you can run all tests in a module by module name. For example, run all tests with add in their name.

<details><summary>Click here to expand the sample code</summary>

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

```rust
// Run a single test
$ cargo test one_hundred
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out

```

```rust
// Run multiple tests
$ cargo test add
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

</details>

### 2.4 Ignore some tests: `#[ignore]`, `--ignored`

For tests you don't want to run, you can add `#[ignore]` after `#[test]`. `expensive_test` is listed as `ignored` and is not run.

If you only want to run ignored tests, you can use `cargo test -- --ignored`.

<details><summary>Click here to expand the sample code</summary>

```rust

#[test]
fn it_works() {
assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
// Code that takes an hour to run
}
```

```rust
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
```

```rust
$ cargo test -- --ignored
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

</details>

## 3. Types of tests: unit tests, integration tests

- Unit tests are smaller and more focused. They test one module at a time in an isolated environment, or test private interfaces. They are placed in the same file in the *src* directory as the code being tested, and the `tests` and `cfg(test)` attributes are used to mark functions and modules respectively. The unit tests are introduced above.
- Integration tests are completely outside the tested library. They are called in the same way as other code that uses this library. They only test public interfaces and each test may test multiple modules. You need to create a *tests* directory in a different folder from the tested code. The files in the tests directory are compiled only when `cargo test` is run, so there is no need for the `#[cfg(test)]` annotation. The purpose of integration testing is to check whether multiple parts of the tested library can run properly together, because some code units that can run correctly individually may also have problems when integrated together, so the coverage of integration tests is also very important.

### 3.1 Unit Tests Uint Tests

- Test Module and `#[cfg(test)]`
The test module is automatically generated when you create a project. The `cfg` attribute stands for *configuration*, which tells Rust that the following code is only included in a specific configuration option. The configuration option here is `test` for compiling and running tests, so the `#[cfg(test)]` annotation of the test module tells Rust to compile and run the `helper` function and the `#[test]` annotated function in the module only when executing `cargo test`, and they should not be included in the compilation result when running `cargo build`.

<details><summary>Click me to expand the sample code</summary>

  ```rust

  #[cfg(test)]
  mod tests {
      #[test]
      fn it_works() {
          assert_eq!(2 + 2, 4);
      }
  }
  ```

  </details>

- Testing private functions
In other languages, it is difficult or even impossible to test private functions. But Rust's privacy rules allow testing private functions.

<details><summary>Click here to expand the sample code</summary>

  ```rust
  pub fn add_two2(a: i32) -> i32 {
      internal_adder(a, 2)
  }

  fn internal_adder(a: i32, b: i32) -> i32 {
      a + b
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn internal() {
          assert_eq!(4, internal_adder(2, 2));
      }
  }
  ```

  </details>

### 3.2 Integration Tests

#### 3.2.1 tests directory

To write integration tests, you need to create a *tests* directory in the project root directory, at the same level as *src*. Then you can create as many test files as you want in this directory.
Keep the code in *src/lib.rs* in the test private function. And create a *tests* directory and create a new file *tests/integration_test.rs*, as shown below:

<details><summary>Click me to expand the sample code</summary>

```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two2(2));
}
```

</details>

After running  `cargo test`  , we get three sections of output: unit tests, integration tests, and doc tests.
The first section, unit tests, is the same as before: one line per unit test, followed by a summary line for the unit tests.
The integration tests section starts with the line  `Running tests/integration_test.rs (target/debug/deps/integration-test-ce99bcc2479f4607)` (the hash at the end of the output may be different). Each subsequent line is a test function in the integration test, and there is a summary line for the integration test before the  `Doc-tests adder`  section.

<details><summary>Click here to expand the sample code</summary>

```solidity
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running target/debug/deps/adder-abcabcabc

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-ce99bcc2479f4607

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

</details>

Similarly, you can run some integration tests.

- Run a specific integration test: `cargo test function name`;
- Run all tests in a test file class: `cargo test --test file name`.

<details><summary>Click here to expand the sample code</summary>

```rust
$ cargo test --test integration_test
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/integration_test-952a27e0126bb565

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

</details>

#### 3.2.2 Shared submodules

Cargo compiles each file as a separate crate, and these files do not share behavior (unlike the rules for files under src), so you need to import the tested library `use adder` in each file. However, if you want to create a `helper` helper function, for example, create a *tests/common.rs* file and a function called  `setup`, and hope that this function can be called by test functions in multiple test files.

<details><summary>Click me to expand the sample code</summary>

```solidity
pub fn setup() {
// Write the code required for specific library tests
}
```

</details>

Run the test again, and you will see a new test result section corresponding to the  `common.rs` file in the test results, even though this file does not contain any test functions and there is no call to the  `seup` function anywhere.

<details><summary>Click me to expand the sample code</summary>

```rust
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/common-b8b07b6f1be2db70

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-d993c68b431d39df

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

</details>

If you don't want  `common`  to appear in the test output, you can create  *`tests/common/mod.rs`*  instead of  *`tests/common.rs`* . Subdirectories in the *`tests`*  directories will not be compiled as separate `crate`s or appear as part of the test output. Then, you can use _`tests/common/mod.rs`_ as a module in any integration test file. For example, the  `it_adds_two`  test in *`tests/integration_test.rs`*  declares the module `mod common` and calls the  `setup`  function `common::setup()`.

<details><summary>Click here to expand the example code</summary>

```solidity
use adder;

mod common;

#[test]
fn it_adds_two2() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

</details>

#### 3.2.3 Binary crate

If the project is a binary crate and contains only src/main.rs but no src/lib.rs, it is impossible to create integration tests in the tests directory and import the functions defined in src/main.rs using the use statement. Only library crates expose functions that can be called and used by other crates. Binary crates run independently and only put the core logic code, that is, the code that needs to be tested, in src/lib.rs.

## Summary

This article covers the commonly used functions in Rust testing (common macros, properties and enumerations such as assert!, panic!), commands (cargo test) and types (unit testing, integration testing), to help you quickly get started with Rust testing.




