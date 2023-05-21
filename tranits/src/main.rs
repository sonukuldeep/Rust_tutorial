struct Film {
    title: String,
    director: String,
    studio: String
}

struct Book {
    title: String,
    author: String,
    publisher: String
}

trait Catalog {
    fn describe(&self);

    // this return default msg
    fn popularity(&self) {  
        println!("We need more info about this")
    }
}

impl Catalog for Film {
    fn describe(&self) {
        println!("{} was directed by {} through {} studios", 
        self.title, 
        self.director, 
        self.studio)
    }
}

impl Catalog for Book {
    fn describe(&self) {
        println!("{} was written by {} and published by {}", 
        self.title, 
        self.author, 
        self.publisher)
    }
}

fn main() {
    let capt_marvel = Film {
        title: String::from("Captain Marvel"),
        director: String::from("Anna Boden and Ryan Fleck"),
        studio: String::from("Marvel")
    };

    capt_marvel.describe();

    let elantris = Book {
        title: String::from("Elantris"),
        author: String::from("Brandon Sanderson"),
        publisher: String::from("Tor Books")
    };

    elantris.describe();
}
