struct Book{
    title: String,
    author: String,
    publication_year: u32,
}


fn main() {

    let alice = Book {
        title : String::from("ALHPA"),
        author : String::from("ALIX"),
        publication_year : 2000
    };

    println!("Title : {}", alice.title);
    println!("Author : {}", alice.author);
    println!("Publication Year : {}", alice.publication_year);
}

