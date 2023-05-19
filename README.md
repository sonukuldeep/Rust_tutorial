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