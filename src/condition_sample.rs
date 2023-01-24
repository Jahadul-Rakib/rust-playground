pub fn test() {
    block_test();
    if_check();
    if_let_check();
    while_let_checking();
}

fn block_test() {
    let x = {
        let a = 10;
        let b = { 2 * 3 };
        a * b
    };
    println!("{}", x);
}
fn if_check() {
    let mut x = 15;
    let i = if x % 2 == 0 && x < 10 {
        x
    } else if x % 2 == 0 && x > 10 {
        x + 1
    } else {
        x * x
    };
    println!("{}", i)
}

fn if_let_check() {
    let arg = std::env::args().next();
    if let Some(x) = arg {
        println!("{}", x)
    } else {
        println!("not found")
    }
}

fn while_let_checking() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
}
