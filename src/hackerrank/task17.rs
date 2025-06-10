use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new(); // HashMap<bird_id, count>
    
    // check all birds array
    for bird in arr {
        // check is current bird a new one
        if map.get(bird).is_none() {
            map.insert(*bird, 1); // add this bird
        }
        else {
            *map.get_mut(bird).unwrap() += 1; // grow up a count
        }
    }

    // find lower bird_id of the most common
    let mut min_of_max: (i32, i32) = (0, 0); // (bird_id, count)
    for (bird, count) in map.iter() {
        // found more common bird
        if min_of_max.1 < *count {
            min_of_max.0 = *bird;
            min_of_max.1 = *count;
            continue;
        }

        // the same common but with lower id 
        if min_of_max.0 > *bird && min_of_max.1 == *count {
            min_of_max.0 = *bird;
        }
    }

    // return a bird type
    return min_of_max.0;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}