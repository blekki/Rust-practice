fn moving(str: String, mut shift: isize) -> String {
    let mut new_str: String = String::new();

    if shift.abs() as usize > str.len() {
        shift = shift % str.len() as isize;
    }
    // if (shift * -1) as usize > str.len() {
    //     shift = shift % str.len() as isize;
    // }

    let (first, second);
    if shift > 0 {
        let new_shift: usize = str.len() - shift as usize;
        (first, second) = str.split_at(new_shift);
    }
    else {
        let new_shift: usize = (shift * -1) as usize;
        (first, second) = str.split_at(new_shift);
    }

    new_str += second;
    new_str += first;
    return new_str;

    // for i in 0..str.len() {
    //     print!("{i}");
    //     let mut pos = i + shift;
    //     if pos >= str.len() {
    //         pos -= str.len();
    //     }
    //     if pos < 0 {
    //         pos += str.len();
    //     }
    //     let ch = str.chars().nth(pos).unwrap();
    //     new_str.push(ch);
    // }
    // print!("\n");

}

fn main() 
{   
    let basic: String = String::from("abcdefgh");
    let shifts: [(isize, &str); 9] = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts.iter().for_each(|(sh, _str)|
        println!("{sh:3}: {}", moving(basic.to_string(), *sh))
    );

    // println!(" 2: {}", moving("12345678".to_string(), 2));
    // println!("-2: {}", moving("12345678".to_string(), -2));
}