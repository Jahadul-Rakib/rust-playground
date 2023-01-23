struct Person {
    name: String,
    age: u8,
}

pub fn test() {
    let person = Person {
        name: String::from("rakib"),
        age: 21,
    };
    println!("{:?}", person.name);
    println!("{:?}", person.age);

    let person2 = Person {
        name: String::from("rifat"),
        ..person
    };
    println!("{:?}", person2.name);
    println!("{:?}", person2.age);
}
