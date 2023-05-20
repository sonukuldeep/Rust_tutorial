use std::fs::File;

fn main() {
    // panic!("Farewell");

    // let v = vec![0, 1, 2, 3];
    // println!("{}", v[6]); // This will cause a panic

    // let fruits = vec!["banana", "apple", "coconut"];

    // let first = fruits.get(0);
    // println!("{:?}", first);
    // let non_existent = fruits.get(99);
    // println!("{:?}", non_existent);

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Can't open the file {:?}", error),
    // };

    // let f = File::open("hello.txt").unwrap(); // same as above
    
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // same as above
}
