use std::io::{self, BufRead};

fn miniMaxSum(arr: &[i32]) {
    let mut min = arr[0];
    let mut max = arr[0];
    let mut sum: i64 = arr[0] as i64;

    // calculation
    for i in 1..arr.len() {
        sum += arr[i] as i64; // final sum

        if max < arr[i] { max = arr[i] as i32; } // find the higher number
        if min > arr[i] { min = arr[i] as i32; } // find the lower number
    }

    // print result
    print!("{} ", sum - max as i64); // print min sum
    print!("{}",  sum - min as i64); // print max sums
    print!("\n");
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}