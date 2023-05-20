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
Ownership
Ownership is a set of rules that govern how a Rust program manages memory.
Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.
- Related to how your program manages memory
- Rust stores data in two differently structured parts of memory
    - Stack
    - Heap


### Ownership Rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

Check docs for detailed explaination <br/>
[docs](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)