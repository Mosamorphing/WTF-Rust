// Define a trait Descriptable, including a method that must be implemented and a method that is implemented by default
trait Descriptable {
 fn describe(&self) -> String;

 // The method implemented by default can be overridden by the type that implements the trait
 fn default_descriptiton(&self) -> String {
 String::from("This is default functions")
 }
}

// Define a structure Person and automatically generate an implementation of the Clone trait for it
#[derive(Clone)]
struct Person {
 name: String,
 age: u32,
}

// Implement the Describable trait for the structure Person
impl Descriptable for Person {
 fn describe(&self) -> String {
 format!("{} is {} years old", self.name, self.age)
 }
}

//Define a function that accepts a reference to any type that implements the Describeable trait and outputs a description
fn output_description(item: &impl Describeable) {
 println!("{}", item.describe());
}

//Define a function using a generic parameter T, which must implement the Describeable and Clone traits
fn some_function<T: Describeable + Clone>(item: T) {
 let item_copy = item.clone(); // Clone item
 println!("Description: {}", item_copy.describe());
}

//Define a function that returns a type that implements the Describeable trait
fn return_description() -> impl Descriptable {
 Person {
 name: String::from("Lisa"),
 age: 18,
 }
}

// Define a trait Printable that inherits Describeable and provide a default implemented method print
trait Printable: Descriptable {
 fn print(&self) {
 println!("{}", self.describe());
 }
}

// Implement Printable trait for structure Person
impl Printable for Person {}

fn main() {
 //Create a Person instance
 let alice = Person {
 name: String::from("Alice"),
 age: 30,
 };

 //Call describe method
 println!("{}", alice.describe());

 //Call default_description method
 println!("{}", alice.default_descriptiton());

 //Use output_description function
 output_description(&alice);
 // Use some_function function, passing a cloned alice
 some_function(alice.clone());

 // Call the return_description function and print its description
 println!("{}", return_description().describe());

 //Create another Person instance and call the print method
 let bob = Person {
 name: String::from("Bob"),
 age: 25,
 };
 bob.print();
}
