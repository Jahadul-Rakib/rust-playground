struct Color(String);
struct Machine(String, i8);

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

    let color = Color(String::from("RED"));
    println!("{:?}", color.0);

    let machine = Machine(String::from("BMW"), 10);
    println!("{} , {}", machine.0, machine.1);
}
