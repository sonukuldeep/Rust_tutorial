//classic struct
struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

// tuple struct
struct Point2D(u32, u32);

fn main() {
    let person = Person {
        name: String::from("Adam"),
        likes_oranges: true,
        age: 25,
    };

    println!("Person name is: {}", person.name);

    let origin = Point2D(100, 200);
    println!("Point location is: {:?} {:?}", origin.0, origin.1);

    let Point2D(x, y) = origin; // destructuring
    println!("Point location is: {:?} {:?}", x, y);

    
}
