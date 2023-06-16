fn main() {
    let x: i32 = 5;
    println!("Value of x is {}", x);
    let x: &str = "This is new";
    println!("Value of x is {}", x);

    let (x, y) = (1, 2);
    println!("Value of x is {} and y is {}", x, y);

    let (x,y);
    (x,..) =(3,4);
    [..,y] = [2,1];
    assert_eq!((x,y),(3,1));
    println!("Success")
}
