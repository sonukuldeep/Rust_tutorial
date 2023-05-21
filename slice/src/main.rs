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



