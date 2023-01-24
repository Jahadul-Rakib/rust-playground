pub fn test() {
    block_test();
}

fn block_test() {
    let x = {
        let a = 10;
        let b = { 2 * 3 };
        a * b
    };
    println!("{}", x);
}
