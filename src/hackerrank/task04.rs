use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let len = arr.len(); // matrix size length
    // diagonals
    let mut a_diag = 0;
    let mut b_diag = 0;

    // calculations
    for i in 0..len {
        a_diag += arr[i][i];
    }

    for i in 0..len {
        b_diag += arr[i][(len - 1) - i];
    }
    
    // get result
    let result = (a_diag - b_diag).abs();
    return result;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}