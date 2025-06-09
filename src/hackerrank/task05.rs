use std::io::{self, BufRead};

fn plusMinus(arr: &[i32]) {
    let div = arr.len() as i32;
    let mut pos = 0;
    let mut neg = 0;
    let mut zero = 0;

    // find positive, negative and zero number counts
    for i in arr {
        if *i > 0 { 
            pos += 1;
            continue;
        }

        if *i < 0 { 
            neg += 1;
            continue;
        }

        zero += 1;
    }
    
    // print result
    println!("{:4}", (pos  as f32 / div as f32));
    println!("{:4}", (neg  as f32 / div as f32));
    println!("{:4}", (zero as f32 / div as f32));
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}