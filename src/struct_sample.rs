enum Type {
    CAR,
    SHIP,
    MODEL(String),
    SERIAL(u32),
    ACTION { name: String, time: String },
    OIL(u32),
}

// tuple type struct
struct Color(String);
struct Machine(String, i8);

// attribute type struct
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
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

    let color = Color(String::from("RED"));
    println!("{:?}", color.0);

    let machine = Machine(String::from("BMW"), 10);
    println!("{} , {}", machine.0, machine.1);

    let rakib = Person::new(String::from("rakib"), 12);
}
