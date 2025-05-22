use std::collections::HashMap;

// print one result row
fn printer(str: String, unique: HashMap<char, u8>) {
    str.chars().for_each(|ch| {
        if unique.get(&ch).is_some() {
            print!("{}", *unique.get(&ch).unwrap());
        } else {
            print!(" ")
        };
    });
    println!();
}

fn finder(array: &[String]) {
    let mut unique: HashMap<char, u8> = HashMap::new();
    let mut num: u8 = 1;

    array.iter().for_each(|s| {
        s.chars().for_each(|ch| {
            if ch != ' ' {
                if !unique.contains_key(&ch) {
                    unique.insert(ch, num);
                    num += 1;
                }
            }
        });
    });

    // > if wanna check hashmap values
    // unique.iter().for_each(|f| {
    //     print!("{} : ", f.0);
    //     print!("{}", f.1);
    //     println!();
    // });

    // print result
    printer(array[0].clone(), unique.clone());
    printer(array[1].clone(), unique.clone());
    for _ in 0..array[0].len() { print!("-"); } println!(); // separater
    printer(array[2].clone(), unique.clone());
}

fn main() {
    
    let test: [String; 3] = [String::from(" muxa"),
                             String::from("x   a"),
                             String::from(" slon")];

    finder(&test);
    // <> must be the same result <>
    //  1234
    // 3   4
    // -----
    //  5678
}