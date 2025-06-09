use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn timeConversion(s: &str) -> String {
    let morning   = s.chars().nth(s.len() - 2).unwrap() == 'A'; // check is it a morning time
    let min_sec = s[2..(s.len() - 2)].to_string(); // get minutes and seconds
    let mut hour   = s[0..2].to_string().parse::<i32>().ok().unwrap(); // get hour

    if hour >= 12 { hour -= 12; } // check on overflow up 12 hour
    if !morning   { hour += 12; } // change 12-hour into 24-hour system

    //result
    return format!("{:0>2}", hour.to_string()) + &min_sec;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}