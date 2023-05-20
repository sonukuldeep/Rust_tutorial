fn main() {
    if 1 == 2 {
        println!("Rust is broken");
    } else {
        println!("Cool!");
    }

    let formal = true;
    let _greeting = if formal {
        println!("Good evening");
    } else {
        println!("Whats up");
    };

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

}
