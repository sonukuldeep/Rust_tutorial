enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x: i64, y: i64},

}

// Optional enum
enum Option<T> { //T is generic type here
    Some(T),
    None,
}

fn main() {
    let quit = WebEvent::KeyPress('q');

    let something = Some(1);

}
