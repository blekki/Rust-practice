use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    // result parts
    let mut aRes = 0; 
    let mut bRes = 0; 

    // checking
    for i in 0..a.len() { 
        if a[i] > b[i] {
            aRes += 1;
            continue;
        }

        if a[i] < b[i] {
            bRes += 1;
            continue;
        }
    }

    // return result in comfortable form
    let result: Vec<i32> = vec![aRes, bRes];
    return result;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = compareTriplets(&a, &b);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
