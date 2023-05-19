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