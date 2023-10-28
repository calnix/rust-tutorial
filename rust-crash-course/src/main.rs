#![deny(clippy::all)]

struct Person {
    name: String,
    age: u8,
}


fn main() {
 
    let person = Person {
        age: 30,
        name: "John".to_string(),
    };

    println!("{}", person.age)
}

