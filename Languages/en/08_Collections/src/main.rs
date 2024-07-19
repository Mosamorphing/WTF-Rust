use std::collections::HashMap;

fn main() {
 let mut v: Vec<i32> = Vec::new(); // Create an empty vector
 v.push(5); //Add elements to the vector
 v.push(6);
 v.push(7);
 v.push(8);

 println!("Vector: {:?}", v); // Print vector


 let v = vec![1, 2, 3, 4, 5]; // Use macros to create and initialize vectors
 println!("Vector: {:?}", v);

 for i in &v {
 println!("{}", i);
 }


 let mut s = String::new(); // Create an empty string
 s.push_str("Hello"); // Add text
 s.push(' '); // Add a single character
 s.push_str("world!");

 println!("String: {}", s);


 let s1 = "Hello".to_string();
 let s2 = "World!";
 let s3 = s1 + s2; // Note: s1 has been moved here and can no longer be used

 println!("Combined string: {}", s3);

 let s1: &str = "Hello, world!"; // The type of string literal is &str
 let s2: &str = &s1[0..5]; // Create a substring using slicing syntax
 println!("s1: {}", s1);
 println!("s2: {}", s2);

 let s3: String = s1.to_string(); // Convert &str to String
 println!("s3: {}", s3);

 print_str(s1); // Pass &str to function



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

fn print_str(s: &str) {
 println!("Inside print_str: {}", s);
}
