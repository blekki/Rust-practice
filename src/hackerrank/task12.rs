use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    let mut result = false; // default result

    // try to find the same out the way for the kangaroos
    let mut pos1 = x1;
    let mut pos2 = x2;
    while pos1 <= pos2 { // while the first kangaroos behind the second
        if pos1 == pos2 { // if find
            result = true;
            break;
        }
        pos1 += v1;
        pos2 += v2;
    }

    // return result in a comfortable form
    return if result {String::from("YES")} else {String::from("NO")};
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let x1 = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let v1 = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let x2 = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let v2 = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = kangaroo(x1, v1, x2, v2);

    writeln!(&mut fptr, "{}", result).ok();
}