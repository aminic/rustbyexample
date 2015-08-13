#![allow(dead_code)]
enum Person {
    Skinny,
    Fat,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 }
}

fn inspect(p: Person) {
    match p {
        Person::Skinny => println!("Is skinny!"),
        Person::Fat => println!("Is fat!"),
        Person::Height(i) => println!("Has a height of {:?}", i),
        Person::Weight(i) => println!("Has a weight of {:?}", i),
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn main() {
    let person = Person::Height(18);
    let dav = Person::Info { name: "Dave".to_owned(), height: 72 };
    inspect(person);
    inspect(dav);
}