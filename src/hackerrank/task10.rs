use std::env;
use std::fs::File;
use std::intrinsics::ceilf32;
use std::io::{self, BufRead, Write};

fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = vec![]; // prepare a variable for result

    // calculation
    for i in grades {
        let potential = (*i as f32 / 5.0).ceil() as i32 * 5; // find a potential student grade
        if potential - *i < 3 && potential >= 40 { // if a student can pass, up a grade
            result.push(potential);
        }
        else {
            result.push(*i);
        }
    }

    return result;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = gradingStudents(&grades);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}