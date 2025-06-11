use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut socks: HashMap<i32, i32> = HashMap::new(); // HashMap<sock_id, count>

    // find all socks
    for i in ar {
        // check if the current sock is a new one
        if socks.get(i).is_none() {
            socks.insert(*i, 1); // add this to HashMap
        }
        else {
            *socks.get_mut(i).unwrap() += 1; // grow up a count by one
        }
    }


    // find all pairs
    let mut pairs = 0;
    for sock in socks {
        pairs += (sock.1 / 2) as i32;
    }

    return pairs;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sockMerchant(n, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}