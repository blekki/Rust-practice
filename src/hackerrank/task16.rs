use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn divisibleSumPairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0; // pair count
    
    // try to find the pairs
    for a in 0..(ar.len() - 1) { // first num
        let first = ar[a];
        
        for b in (a + 1)..ar.len() { // second num
            let second = ar[b];
            if (first + second) % k == 0 { count += 1; } // if find, grow up count
        }
    }

    return count;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = divisibleSumPairs(n, k, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}
