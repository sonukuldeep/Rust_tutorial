# Rust

Following this tutorial [link](https://www.youtube.com/watch?v=PpWR6zungUk&list=PLlrxD0HtieHjbTjrchBwOVks_sr8EVW1x&index=1)

## Chapter-1

### Compile rust code

Syntax:-
```rs
rustc <file name>
```

Example:-
```rs
rustc main.rs
```

Run file without extention after compilation
```rs
./main
```

## Chapter-2

### Variable

- Varible in rust are declared using 'let' keyword
- Variable are immutable by default
- To make a variable mutable we use 'mut' keyword

Example:-
```rs
let x = 100;
x = 150; // error x is immutable by default and cannot be changed
let x = 200; // allowed since the varible x is re-initallised. This is called shadowing
let mut y = 150; // declared immutable can can be changed
y = 20; // allowed
```

### Constant

- A constant is declared using 'const' keyword
- They are immutable and cannot change
- Constants can be set only to a constant expression and not to the result of a function call or any other value that will be computed at runtime.
- Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of the code need to know about.
- Constant declaration must have a specified data type.
- Naming convention is all caps with underscore btw words

Example:-
```rs
const SCORE_LIMIT: u32 = 100;
```

## Chapter-3

### Datatypes

Integers
- Integers are whole numbers
- They are either signed or unsigned
- Rust defaults to i32 which is the fastest

Floating
- Numbers with decimals
- f32 and f64 are two tyes
- default is f64

Boolean
- Have a value of either true or false
- specified using keyword bool
- one byte in size

Characters
- Represent letters
- Specified using char keyword
- Use single quotes
- Four bytes in size

## Chapter-4

### Compound datatypes

Array
- Continuous group of items
- fixed length
- Length known at compile time
- homogeneous

Syntax:-
```rs
let array: [u32; 3] = [1,2,3];

let first_item = array[0];
let will_warn = array[100]; // compiler will warn while at runtime it will crash
```

Tuples
- Linear group of items
- fixed length
- length known at compile time
- heterogenous
- Empty tuple is called unit

```rs
let tuple: (bool, u16, u8) = (true, 2, 3);

let first_item = tuple.0;
let error = tuple.100; // will give error
```

## Chapter-5

Function
- Argument type is always required
- Return type is required if value is returned
- If no value is retured, return type is unit

Syntax:-
```rs
fn function_name(argument: argumentType) -> returnType {
    //body
}
```

Example:-
```rs
fn change_to_upper_case(input: String) -> String {
    let mut output = input.to_uppercase();
    output.push('!');
    output // mark missing semi-colon which tell the compiler to return this
}

let output = change_to_upper_case("Hello world");
```

## Chapter-6

### Struct

Use the keyword struct followed by a name in order to create a struct

- A type that's composed of other types
- Can contain different types
- It is further divided into 3 rtpes
    - Classic
    - Tuple
    - Unit

Classic structs
- Most commonly used
- Each field has a name and a type

Tuple
- Similar to classic structs
- Their fields have no names

Uint structs
- Have no field
- Similar to the () unit type

Syntax
```rs
struct Car {
    make: String
    model: String,
    Yeat: u32,
}

// instanciate a struct
let car1 = Car {
    make: String::from("Ford"),
    model: String::from("Mustang"),
    year: 1967,
}

car1.year // return year 
car1.make // return make
```

## Chapter-7

### Enums

Use the word enum folled by a name to create an enum

Enums
- An enum variant can include any kind of data
- An enum can have a variety of types
- List all variables of same data or of the same type
- Common feature across programming languages
- Referred to as algebric data types

Example:-
```rs
enum CardinalDirections {
    North,
    South,
    East,
    West,
}

let north = CardinalDirections::North;
let south = CardinalDirections::South;
let east = CardinalDirections::East;
let west = CardinalDirections::West;

// another way
enum CardinalDirections {
    North(String),
    South(String),
    East(String),
    West(String),
}

let west = CardinalDirections::West(String::from("West"));
```

## Chapter-8

### If
syntax: 
```rs
let price = 10;
if price > 0 {
    println!("Price is greater than zero");
}
```

### If else 
syntax: 
```rs
let price = 10;
if(price > 0) {
    println!("Price is greater than zero");
} else {
    println!("Price less than or equal to zero");
}
```

### If else if
syntax: 
```rs
let price = 10;
if(price > 0) {
    println!("Price is greater than zero");
} else if price == 0 {
    println!("Price equal to zero");
} else {
    println!("Price less than zero");
}
```

### Match
- Rust provides pattern matching with the match keyword
- A scrutinee expression is provided to compare to the patterns.
- Arms are evaluated and compared with the scrutinee expression

syntax:-
```rs
let x = 1;
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```

## Chapter-9

### loop
It is used to execute over a blockof code forever. Or until it is stopped, or the program quits.

syntax:- 
```rs
loop {
    println!("I loop forever!");
}
```

### break
syntax:- 
```rs
loop {
    println!("Stops when program hits break!");
    break;
}
```

### While loop
syntax:- 
```rs
let mut number = 3;
while number != 0 {
    println!("{}", number);
    number -= 1;
}
println!("End of while");
```

### for loop

syntax:-
```rs
let a = [10, 20, 30, 40, 50];
for element in a.iter() {
    println!("The value is: {}", element);
}
```

## Chapter-10

### Error handline
- Panic
- Option
- Result

#### Panic
Simplest way to handle errors

What happens when a panic is
encountered?
- Failure message is printed
- Program unwinds and cleans up the
Stack
- Program quits

Should only be used when program comes to an unrecoverable state

Syntax:-
```rs
panic!("Farewell");

let v = vec![0, 1, 2, 3];
println!("{}", v[6]); // This will cause a panic
```

#### Option enum
- Manages the possibility of
none existent values
- Type T is generic and
associated with the Some
variant.
- None indicates that no
element was found.
- Some means that an element
of type T was found

Syntax:-
```rs
enum Option<T> {
    None,
    Some(T),
}
```

#### Result enum
- Used for input/output operations (I/O)
    - Parsing strings
    - File access
    - Data validation
- Best for expected failures

Syntax:- 
```rs
fn main() {
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
```

- Used for recoverable errors
that are more common
- The Ok(T) variant represents a
success and contains a value
- The Err(E) variant represents an
error

> Unwrap and Expect
- Helper methods Of the Result
type
- Unwrap returns the value
inside the Ok variant. Returns
a panic! macro for the Err
variant.
- Expect returns a value or
called the panic! macro with a
detailed error message

```rs
fn main() {
    // ok
    File::open("hello.txt").unwrap();

    // Err
    File::open("hello.txt").expect("Failed to open hello.txt");
}
```

> The ? operator
- Similar to a match statement
- For Result type:
    - Unwraps the value if 0k variant
    - Returns an error if Err variant.
- For Option type:
    - Returns a value is with the Some variant
    - Returns nothing for the None variant

## Chapter-11
### Memory
In rust memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

- Rust stores data in two differently structured parts of memory
1. Stack:
- The stack is a region of memory that is used for storing local variables and function call information.
- It operates on a Last-In, First-Out (LIFO) principle.
- Stack memory allocation and deallocation are managed automatically by the compiler.
- Stack allocations have a fixed size known at compile time.
- The lifetime of stack-allocated variables is determined by the scope in which they are defined.
- Stack memory is generally faster to allocate and deallocate than heap memory.

2. Heap:
- The heap is a region of memory used for dynamic memory allocation.
- It allows for the allocation of memory whose size is not known at compile time or that needs to outlive the scope it was created in.
- Heap memory allocation and deallocation are managed explicitly by the programmer.
- Heap allocations have a dynamic size and are requested using functions like Box::new(), Vec::new(), or via system APIs.
- Memory on the heap needs to be manually deallocated to prevent memory leaks.
- Heap memory access is generally slower than stack memory due to indirection and dynamic allocation overhead.

## Chapter-12
### Ownership
Ownership is a core concept in Rust that governs how memory is managed and ensures memory safety without the need for a garbage collector or runtime overhead. It revolves around the idea that every value in Rust has a single owner at any given time.

When you create a value in Rust, such as a variable or an object, you become its owner. As an owner, you have full control over that value and its associated memory. Ownership comes with three fundamental rules:

1. Unique Ownership:
- Each value in Rust has a unique owner, and there can only be one owner at a time.
- The owner is responsible for the memory deallocation when the value goes out of scope.
- When the owner goes out of scope, the value is dropped, and its memory is automatically freed.

2. Move Semantics:
- Ownership in Rust follows the move semantics by default.
- When a value is assigned to another variable or passed as a function argument, the ownership is transferred.
- The original owner loses its ownership, and the new owner takes control of the value.
- This prevents multiple variables from accidentally accessing and modifying the same value, reducing bugs and data races.

3. Borrowing and References:
- To allow temporary access to a value without transferring ownership, you can use borrowing and references.
- Borrowing enables you to lend a reference to a value to another part of your code.
- References come in two forms: immutable references (&T) and mutable references (&mut T).
- Borrowing enforces strict rules at compile-time to prevent data races, dangling references, and use-after-free errors.

These ownership rules enable the Rust compiler to perform static analysis and memory management at compile time. It ensures that values are dropped when they are no longer needed and prevents common issues like memory leaks and use-after-free bugs. The ownership system allows Rust to provide memory safety and concurrency guarantees without the need for manual memory management or garbage collection.

Rust's ownership model may seem strict at first, but it promotes clear ownership semantics, eliminates many runtime errors, and guarantees thread safety without sacrificing performance. It enables you to write robust, efficient, and safe code in a systems programming language.

Check docs for detailed explaination <br/>
[docs](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

## Chapter-13
### Borrowing
Borrowing is a fundamental concept related to ownership and memory management. It ensures that multiple parts of a program can access and manipulate data without causing issues like data races or memory leaks.

When you create a variable in Rust, you become its owner, meaning you have exclusive control over that variable and its memory. However, sometimes you need to temporarily share that variable with other parts of your code without transferring ownership. This is where borrowing comes into play.

Borrowing allows you to lend a reference to a variable or data structure to another part of your code, called the borrower, without giving up ownership. There are two types of borrowing in Rust: mutable borrowing and immutable borrowing.

1. Immutable Borrowing (Shared Reference):
- When you borrow a variable with an immutable borrow, you can read its value but cannot modify it.
- Immutable borrows are denoted using the & symbol. For example, &x borrows the variable x immutably.
- Multiple immutable borrows can exist simultaneously, allowing multiple parts of your code to read the data.
- The borrower can hold the reference for as long as it needs, but it cannot outlive the owner's lifetime.

2. Mutable Borrowing (Exclusive Reference):
- When you need to modify a borrowed variable, you use a mutable borrow.
- Mutable borrows are denoted using the &mut symbol. For example, &mut y borrows the variable y mutably.
- Rust enforces strict rules with mutable borrows. Only one mutable borrow can exist at a time for a specific piece of data within a given scope.
- Mutable borrows prevent simultaneous access to the data to avoid data races.
- The borrower can hold the mutable reference for as long as it needs, but it cannot outlive the owner's lifetime.

The Rust compiler statically analyzes your code to ensure that borrows adhere to these rules at compile time. This analysis prevents common issues like use-after-free errors, data races, and dangling references, which are typically found in languages with manual memory management.

Borrowing, combined with ownership, allows Rust to achieve memory safety without relying on a garbage collector or runtime overhead. It enables you to write efficient and safe code by enforcing strict rules around the ownership and usage of variables and data structures.<br/>
Check detailed Doc on borowwing [Doc](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

Example:-
```rs
fn main() {
    // Immutable borrowing example
    let x = 5; // Original value

    // Borrowing x immutably with an immutable reference
    let y = &x;

    println!("x: {}", x);
    println!("y: {}", y);

    // Uncommenting the line below will result in a compilation error
    // x = 10; // Error: Cannot assign to `x` because it is borrowed

    // Mutable borrowing example
    let mut z = 10; // Original mutable value

    // Borrowing z mutably with a mutable reference
    let w = &mut z;

    println!("z: {}", z);
    println!("w: {}", w);

    *w += 5; // Modifying the value through the mutable reference

    println!("Modified z: {}", z);
    println!("Modified w: {}", w);
}
```

## Chapter-14
### String
Rust treats strings differently compared to other languages
- Strings are represented by the String type which is part of the Rust standard library and provides methods for string manipulation and operations.
- Rust strings are growable, meaning their length can change dynamically.
- Strings are encoded as UTF-8, allowing for handling and manipulation of Unicode characters.
- The String type is heap-allocated, meaning the memory for the string content is dynamically allocated on the heap.
- Rust provides seamless conversion between string slices (&str) and owned strings (String).
- String manipulation methods like concatenation, appending, slicing, replacing, and more are available for working with strings.

Creating a new String can be done using the String::new() function, which returns an empty string. You can also create a String from a string literal using the to_string() method or the String::from() function:
```rs
let empty_string = String::new();
let hello = "Hello, ".to_string();
let world = String::from("world!");
```

The String type supports concatenation using the + operator or the String::push_str() method:
```rs
let hello = "Hello".to_string();
let world = "world!";
let hello_world = hello + " " + world;

let mut message = String::from("Hello, ");
let name = "Alice";

message.push_str(name);

println!("{}", message); // Output: Hello, Alice
```

Additionally, you can use the format!() macro to concatenate multiple values into a String:
```rs
let name = "Alice";
let age = 25;
let message = format!("My name is {} and I am {} years old.", name, age);
```

Rust's String type provides many other useful methods for manipulating and working with strings, such as indexing, slicing, replacing, splitting, and more. You can explore these methods in the Rust documentation or by using Rust's built-in documentation tool (rustup doc or cargo doc).

It's important to note that Rust distinguishes between string slices (&str) and owned strings (String). String slices are borrowed references to string data, and they can be used for efficient string manipulation when ownership is not required. Rust provides seamless conversion between string slices and String using the &str and String types' respective as_str() and into() methods.

```rs
fn print_string_length(string: &str) {
    println!("Length of the string: {}", string.len());
}

fn main() {
    // Converting String to &str
    let owned_string = String::from("Hello, Rust!");
    let borrowed_string: &str = owned_string.as_str();

    print_string_length(borrowed_string);

    // Converting &str to String
    let borrowed_string = "Hello, Rust!";
    let owned_string: String = borrowed_string.to_string();

    println!("New owned string: {}", owned_string);
}
```

## Chapter-15
### Collection
In Rust, collections are data structures that allow you to store and manipulate multiple values. Rust's standard library provides several collection types, each with its own characteristics and use cases. 

Here are some of the commonly used collection types in Rust:
- Vec<T> (Vector): A dynamically resizable array-like structure that allows you to store a variable number of elements of the same type. Vectors are efficient for random access and provide methods for adding, removing, and modifying elements.
- VecDeque<T> (Double-Ended Queue): Similar to Vec<T>, but with the ability to efficiently add and remove elements from both ends of the collection. It is useful for scenarios where elements need to be inserted or removed from the front or back frequently.
- HashMap<K, V>: A collection of key-value pairs where each key is unique. It provides fast lookup and insertion based on the keys. The keys and values can be of different types, and the ordering of the elements is not guaranteed.
- BTreeMap<K, V>: Similar to HashMap<K, V>, but it maintains the elements in sorted order based on the keys. It is implemented as a balanced binary search tree, which allows efficient range queries.
- HashSet<T>: A collection of unique elements without any specific ordering. It provides fast lookup and insertion based on the elements. The elements must implement the Eq and Hash traits.
- BTreeSet<T>: Similar to HashSet<T>, but it maintains the elements in sorted order. It is implemented as a balanced binary search tree, allowing efficient range queries.
- LinkedList<T>: A doubly linked list where each element is connected to the previous and next element. It allows efficient insertion and removal at both ends but is slower for random access compared to vector-like structures.
- String: As discussed earlier, String represents an owned, growable, UTF-8 encoded string. It provides methods for manipulating and working with string data.

These collection types provide a range of options to handle different scenarios. Rust's ownership and borrowing system ensures memory safety and efficient resource management when working with collections.

Example of Vec and Hashmap:-
```rs
use std::collections::HashMap;

fn main() {
    // Example with Vec<T>
    let mut numbers: Vec<i32> = Vec::new(); // Creating an empty vector

    numbers.push(10); // Adding elements to the vector
    numbers.push(20);
    numbers.push(30);

    println!("Vector: {:?}", numbers); // Printing the vector

    let first_element = numbers[0]; // Accessing an element by index
    println!("First element: {}", first_element);

    let length = numbers.len(); // Getting the length of the vector
    println!("Length: {}", length);

    // Example with HashMap<K, V>
    let mut scores: HashMap<String, i32> = HashMap::new(); // Creating an empty HashMap

    scores.insert(String::from("Alice"), 100); // Adding key-value pairs to the HashMap
    scores.insert(String::from("Bob"), 85);
    scores.insert(String::from("Charlie"), 92);

    println!("HashMap: {:?}", scores); // Printing the HashMap

    let alice_score = scores.get("Alice"); // Accessing a value by key
    match alice_score {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("Alice's score not found"),
    }

    scores.remove("Bob"); // Removing a key-value pair from the HashMap

    for (name, score) in &scores { // Iterating over the HashMap
        println!("Name: {}, Score: {}", name, score);
    }
}
```
## Chapter-15
### Traits
In Rust, traits are a way to define shared behavior or functionality that types can implement. They provide a mechanism for defining a set of methods or associated constants that can be used across different types, allowing for code reuse and abstraction.

Here are some key points about traits in Rust:

1. Trait Definition: Traits are defined using the trait keyword followed by the trait name and a set of method signatures or associated constants. For example:
```rs
trait MyTrait {
    fn method1(&self);
    fn method2(&mut self);
    const CONSTANT: u32;
}
```

2. Implementing Traits: Types can implement traits by providing implementations for each of the trait's methods and associated constants. This is done using the impl keyword. For example:
```rs
struct MyStruct;

impl MyTrait for MyStruct {
    fn method1(&self) {
        // Implementation for method1
    }

    fn method2(&mut self) {
        // Implementation for method2
    }

    const CONSTANT: u32 = 42;
}
```

3. Default Implementations: Traits can provide default implementations for some or all of their methods. Types implementing the trait can choose to override these methods or use the default implementation. Default implementations are defined using the default keyword.
```rs
trait MyTrait {
    fn method1(&self) {
        // Default implementation for method1
    }

    fn method2(&mut self);
}
```

4. Trait Bounds: Trait bounds are used to specify that a generic type parameter must implement a certain trait. This ensures that the generic type can use the methods and functionality defined by the trait. Trait bounds are specified using the where clause or directly in the generic type declaration.

```rs
fn my_function<T: MyTrait>(value: T) {
    // Function implementation
}

fn my_function<T>(value: T)
where
    T: MyTrait,
{
    // Function implementation
}
```

5. Associated Types: Traits can define associated types, which allow the trait to define types that are associated with the trait but not necessarily specific to each implementation. Associated types are specified using the type keyword.
```rs
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Traits are a fundamental part of Rust's type system and are used extensively to enable code reuse, abstraction, and generic programming. They provide a powerful mechanism for defining shared behavior across different types and promoting code flexibility and modularity.