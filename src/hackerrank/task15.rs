use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0; // future result
    
    for a in 0..(s.len() as i32 - m + 1) { // move on "s" array
        // find a sum
        let mut sum = 0;
        for b in 0..m {
            sum += s[(a + b) as usize];
        }

        // check does the sum equel "d"
        if sum == d { count += 1; }
    }
    
    return count;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let s: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let d = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let result = birthday(&s, d, m);

    writeln!(&mut fptr, "{}", result).ok();
}