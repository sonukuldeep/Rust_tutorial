fn main() {
    let text: &str = "Hello\nWorld\n!";
    println!("First line is: {}", first_line(text));
}

fn first_line(string: &str) -> &str {
    string.lines().next().unwrap()
}
