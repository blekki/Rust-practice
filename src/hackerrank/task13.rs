use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0; // future result

    let a_last = a.last().unwrap(); // last a elem
    let b_first = b.first().unwrap(); // first b elem
    
    let mut num = *a_last; // promising number
    while num <= *b_first {
        let mut cond = true;

        // check with all "a" array elems
        for n in a {
            if num % n != 0 {
                cond = false;
                break;
            }
        }

        // check with all "b" array elems
        for n in b {
            if n % num != 0 {
                cond = false;
                break;
            }
        }

        // if number is ok
        if cond { count += 1; }

        num += a_last; // next step
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

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = getTotalX(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}