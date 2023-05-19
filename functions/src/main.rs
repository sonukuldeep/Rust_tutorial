fn main() {
    println!("Hello, world!");

    let get_last = last_char(String::from("Hello"));
    println!("Last character is: {}", get_last);
}

fn last_char(string: String) -> char {
    if string.is_empty() {
        return 'c';
    }

    string.chars().next_back().unwrap()
}