extern crate rand;
use rand::Rng;
use colored::Colorize;

// find min sum 
fn min_adjacent_sum(vec: &Vec<u32>) -> usize {
    // fro calculation
    let mut num_1: u32;
    let mut num_2: u32;
    // for results
    let mut min_sum: u32 = u32::MAX;
    let mut index: usize = 0;

    // find lower pair
    for i in 0..(vec.len()-1) {
        num_1 = vec[i];
        num_2 = vec[i + 1];
        if min_sum > num_1 + num_2 {
            min_sum = num_1 + num_2;
            index = i as usize;
        }
    } 
    return index;
}
// generate random values
fn gen_random_vector(n: usize) -> Vec<u32> {
    let mut vec:Vec<u32> = Vec::new();
    let mut rnd = rand::rng(); 
    for _ in 0..n {
        vec.push(rnd.random_range(10..99));
    }
    return vec;
}

// ########## main program ##########
fn main() {
    const VECTOR_SIZE: usize = 20;
    let vec = gen_random_vector(VECTOR_SIZE); // our vector

    println!("{}", "<><> Vector values: <><>".yellow());
    // get value on screan
    print!("indexes: ");
    for i in 0..(vec.len()) {
        print!("{:2}, ", i);
    }
    print!("\nvalues:  ");
    for i in &vec {
        print!("{:2}, ", i);
    }
    // results:
    print!("{}", "\npair:    ".green());
    let index = min_adjacent_sum(&vec);
    for _ in 0..(index*4) {
        print!(" ");
    }
    print!("{}", "^^--^^".green());
    println!("\nresult: min_sum = {} at indexes {} and {}", (vec[index] + vec[index + 1]), index, index + 1);
}