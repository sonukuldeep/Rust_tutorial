fn main() {
    let x: i32 = 5;
    println!("Value of x is {}", x);
    let x: &str = "This is new";
    println!("Value of x is {}", x);

    let (x, y) = (1, 2);
    println!("Value of x is {} and y is {}", x, y);

    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [2, 1];
    assert_eq!((x, y), (3, 1));
    println!("Success");

    let v: u16 = 32_u8 as u16;
    println!("v is {}", v);

    let c = 120_u8;
    println!("c is {}", c);

    assert_eq!("u8".to_string(), type_of(&c));

    println!("i8 max {}", i8::MAX);
    println!("u8 max {}", u8::MAX);

    let a = 250_000 + 0xff + 0o77 + 0b1111_1111;
    println!("{}", a);

    assert!(0.1_f32 + 0.2_f32==0.3_f32);
    assert!(0.1 as f32 + 0.2 as f32 == 0.3_f32);
    // sum();
    print_character();
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn sum() {
    let mut sum = 0_i32;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5);
    println!("Success");
}

fn print_character() {
    for i in 'a'..='z' {
        println!("{}",i as u8);
    }
}
