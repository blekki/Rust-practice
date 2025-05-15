// Grey code
fn gray(len: u8) -> Vec<String> {
    let mut result: Vec<String> = vec![]; //result
    
    // prepare 
    let mut str = String::from("");
    let range = std::ops::Range::<usize> {start: 10 - (len as usize), end: 10};
    let mut current: u8 = 0b0;
    for _ in 0..(2 as i32).pow(len.into()) {
        // convert binary value into string
        str = format!("{current:->10b}");
        result.push((str[range.clone()]).to_string().replace("-", "0"));
        // next iter
        current += 1;
    }

    return result;
}

// ###### main program ######
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
                 "1100", "1101", "1110", "1111"))
    ];

    test_data.iter().for_each(|(n, out)| 
                              assert_eq!(gray(*n), *out)
                             );
}