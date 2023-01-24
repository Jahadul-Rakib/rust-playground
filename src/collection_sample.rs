pub fn sample_array() {
    // A stack-allocated array
    let array: [i32; 3] = [1, 2, 3];

    // A heap-allocated array, coerced to a slice
    let boxed_array: Box<[i32]> = Box::new([1, 2, 3]);

    let x: [i32; 5] = [1, 2, 3, 4, 5];
    for i in &x {
        println!("{}", i)
    }
}
pub fn sample_tuple() {
    let tuple1: (&str, i8) = ("rakib", 1);
    let tuple2: (&str, i8) = ("rifat", 2);
    let tuple3: (&str, i8) = ("rony", 3);
    let tuple4: (&str, i8) = ("saleh akram", 4);

    let x: [(&str, i8); 4] = [tuple1, tuple2, tuple3, tuple4];
    let mut counter = 0;
    let length = x.len();

    while counter < length {
        println!("Roll: {}, Name: {}", x[counter].1, x[counter].0);
        counter = counter + 1;
    }

    counter = 0;
    while counter < length {
        println!(" Name: {:#?}  Roll: {:#?} ", x[counter].0, x[counter].1); //for debug output
        counter = counter + 1;
    }
}

pub fn sample_slice() {
    let array_data = [2, 3, 13, 4, 3, 3444, 334, 23, 23];

    //slice example
    let first_slice = &array_data[0..array_data.len()]; //same as (0..)
    println!("{:?}", first_slice);

    let second_slice = &array_data[0..2];
    println!("{:?}", second_slice);
}
