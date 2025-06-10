use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let mut higher = scores[0];
    let mut lower  = scores[0];

    let mut h_count = 0; // higher number count
    let mut l_count = 0; // lower  number count

    // check scores
    for i in 1..scores.len() {
        if scores[i] > higher {
            h_count += 1;
            higher = scores[i];
            continue;
        }

        if scores[i] < lower {
            l_count += 1;
            lower = scores[i];
        }
    }
    
    return vec![h_count, l_count];
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breakingRecords(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}