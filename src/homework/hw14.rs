use std::{cmp::Reverse, fmt::format};


fn gray(len: u8) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    // let mut str: String = String::from("");
    // let str = (0..len).map(|_| "0").collect::<String>();
    // println!("{}", str);

    let a = 0b00111;
    // println!("a {}", 0b010101[0..2]);
    // let s = a.to_string();
    let s = format!("{a:b}");
    // let g = 4;
    println!("a {:0<3b}", a);
    println!("s {:0>5}", &s);

    println!();

    // for _ in 0..len {
    //     let copy = str.clone();

    //     let cur_char: char = '-';
    //     let pos = copy.len();
    //     while (cur_char != '0' && pos != 0) {
    //     // for c in 0..copy.len() {
    //         if '0' == copy.chars().nth(2).unwrap() {
    //             pos -= 1 as usize;
    //         }
    //     }
    // }

    // let g: GrayCode8;
    
    // println!("B {}", 0b01001[0..2]);
    let mut str = String::from("");
    let range = std::ops::Range::<usize> {start: 10 - (len as usize), end: 10};
    let mut current: u8 = 0b0;
    for _ in 0..(2 as i32).pow(len.into()) {
        // str = format!("{current:->10b}");
        // println!("{:4b} ({})", current, str);
        // result.push((str[0..(len as usize)]).to_string());
        // result.push((str[range.clone()]).to_string().replace("-", "0"));
        // result.push((str[0..(len as usize)]).chars().rev().collect::<String>());
        // result.push(format!("{current:#010b}").replace("0b1", ""));

        // current = current | 0b1;

        // result.push(format!("{current:#010b}"));


        str = format!("{current:->10b}");
        println!("{:4b} ({})", current, str);
        result.push((str[range.clone()]).to_string().replace("-", "0"));
        // result.push((str[0..(len as usize)]).chars().rev().collect::<String>());

        // current = current << 1;
        current += 1;
    }

    return result;
}

fn main() {
    let test_data = [
        (0, vec!("")),
        (1, vec!("0", "1")),
        (2, vec!("00", "01", "10", "11")),
        (3, vec!("000", "001", "010", "011",
                 "100", "101", "110", "111")),
        (4, vec!("0000", "0001", "0010", "0011", 
                 "0100", "0101", "0110", "0111", 
                 "1000", "1001", "1010", "1011",
                 "1100", "1101", "1110", "1111")),
    ];


    test_data.iter().for_each(|(n, out)| 
            assert_eq!(gray(*n), *out)
        );

    
    // let vec = gray(4);
    // for i in vec {
    //     print!("{i} : ");
    // }
    // println!();
}