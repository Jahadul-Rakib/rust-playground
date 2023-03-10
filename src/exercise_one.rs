pub fn exercise_one() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_matrix: [[i32; 3]; 3] = Default::default();

    let mut colum_a: [i32; 3] = Default::default();
    let mut colum_b: [i32; 3] = Default::default();
    let mut colum_c: [i32; 3] = Default::default();

    let mut counter = 0;
    matrix.into_iter().for_each(|mini_list| {
        colum_a[counter] = mini_list[0];
        colum_b[counter] = mini_list[1];
        colum_c[counter] = mini_list[2];
        counter += 1;
    });

    new_matrix[0] = colum_a;
    new_matrix[1] = colum_b;
    new_matrix[2] = colum_c;

    return new_matrix;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for data in matrix.into_iter() {
        println!("{:?}", data);
    }
}
