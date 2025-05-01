fn moving(str: String, mut shift: isize) -> String {
    let mut result: String = String::new();

    // checking on overflow
    if shift.abs() as usize > str.len() {
        shift = shift % str.len() as isize;
    }

    // shift string
    let (first, second);
    if shift > 0 {
        let new_shift: usize = str.len() - shift as usize;
        (first, second) = str.split_at(new_shift);
    }
    else {
        let new_shift: usize = (shift * -1) as usize;
        (first, second) = str.split_at(new_shift);
    }

    // result
    result += second;
    result += first;
    return result;
}

fn main() 
{
    // #tests   
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
}