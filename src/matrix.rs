fn print_matrix(array: &[i32], n: usize, m: usize) {
    for i in 0..n {
        for j in 0..m {
            print!("{}", { array[i * n + j] });
        }
        println!();
    }
}

fn transpose(array: &mut [i32], n: usize, m: usize) {
    for i in 0..n {
        for j in 0..m {
            if i != j && i + j > 2 * i {
                array.swap(i * n + j, n * j + i);
            }
        }
    }
}

fn main() {
    let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    print_matrix(&array, 3, 3);
    transpose(&mut array, 3, 3);
    println!();

    print_matrix(&array, 3, 3);
}
