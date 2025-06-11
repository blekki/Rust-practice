use std::io::{self, BufRead};

fn bonAppetit(bill: &[i32], k: i32, b: i32) {

    // find shared bills
    let mut shared = 0;
    for i in bill {
        shared += i;
    }
    shared -= bill[k as usize];

    // print result
    if b - (shared / 2) > 0 { // If Brian did overcharge Anna
        println!("{}", b - (shared / 2));
    }
    else {
        println!("Bon Appetit");
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    bonAppetit(&bill, k, b);
}