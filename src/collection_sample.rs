pub fn sample_array() {
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

    while counter <= length {
        println!("Roll: {}, Name: {}", x[counter].1, x[counter].0);
        counter = counter + 1;
    }
}
