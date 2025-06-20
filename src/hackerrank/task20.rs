use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn pageCount(n: i32, p: i32) -> i32 {
    // pages with a final one (if exists)
    let mut pages = n;
    if pages % 2 == 0 { pages += 1; }

    // how to better turn pages - start at the beginning or the end
    let mut turn = p;
    if p * 2 - pages > 0 {
        turn = pages - turn;
    }

    // how much action need to do (result)
    return (turn / 2) as i32;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = pageCount(n, p);

    writeln!(&mut fptr, "{}", result).ok();
}