use std::io::{self, BufRead};

fn staircase(n: i32) {
    let mut void  : i32 = n - 1; // count space in stair row
    let mut symbol: i32 = 1; // count symbol in stair row

    // print stair
    for _ in 0..n {
        print!("{: <1$}", "", void as usize);
        println!("{:#<1$}", "", symbol as usize);
        // preparation for next row
        void   -= 1;
        symbol += 1;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}