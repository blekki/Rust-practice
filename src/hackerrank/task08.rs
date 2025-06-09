use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let mut higher = candles[0];
    let mut count = 1;

    for i in 1..candles.len() {
        if higher == candles[i] { count += 1; }

        if higher < candles[i] { 
            higher = candles[i];
            count = 1;
        }
    }

    return count;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let candles: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthdayCakeCandles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}