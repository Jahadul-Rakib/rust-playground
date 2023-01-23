pub fn sample_variable() {
    const MY_CONSTANT_CONVENTION: &str = "Hello ..... Dunia"; //immutable global variable
    println!("{}", MY_CONSTANT_CONVENTION);

    static BANNER: &str = "Welcome to STATIC"; //mutable global variable
    println!("{}", BANNER);

    let immutable_variable: u32 = 2330;
    println!("{immutable_variable}");

    let mut mutable_variable: i32 = 10;
    println!("{}", mutable_variable);

    while mutable_variable < 100 {
        if mutable_variable > 10 {
            println!("nothing here");
        } else if mutable_variable.eq(&10) {
            println!("in equal condition")
        } else {
            println!("in else condition")
        }
        mutable_variable += 20;
    }

    let open_positive: usize = 120;
    println!("{}", open_positive);

    let open_negative: isize = -120;
    println!("{}", open_negative);
}
