use std::collections::HashMap;

fn printer(array: &[String], unique: HashMap<char, u8>) {
    array.iter().for_each(|s| {
        s.chars().for_each(|ch| {
            if unique.get(&ch).is_some() {
                print!("{}", *unique.get(&ch).unwrap());
            } else {
                print!("-")
            };
        });
        println!();
    });
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

    unique.iter().for_each(|f| {
        print!("{} : ", f.0);
        print!("{}", f.1);
        println!();
    });

    printer(array, unique);
}

fn main() {
    
    let test: [String; 3] = [String::from(" muxa"),
                             String::from("x   a"),
                             String::from(" slon")];

    finder(&test);
    // .iter().for_each(|f| {
    //     print!("{} : ", f.0);
    //     print!("{}", f.1);
    //     println!();
    // });
}