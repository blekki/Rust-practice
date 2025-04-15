
fn gcd(first_value: u32, second_value: u32) -> u32
{
    // pre-checking 
    if first_value == second_value {
        return first_value;
    }
    if first_value == 0 || second_value == 0 {
        return 0;
    }

    // try to find half of higher value
    let mut higher = first_value;
    if second_value > first_value {
        higher = second_value;
    }

    // check everything number lower then "half of higher value" 
    let start_point: u32 = higher / 2; // isn't so necessary line (can be replaced in next operator)
    for current_value in (1..start_point).rev() {
        let first_checking:  bool = first_value  % current_value == 0;
        let second_checking: bool = second_value % current_value == 0;

        if first_checking && second_checking{
            return current_value;
        }
    }
    // default return
    return 1;
}

//#[test]
fn test() {
    println!("<><><> test results <><><>");
    println!("GCB of {:4} and {:4} = {:4}", 12, 17, gcd(12, 17));
    println!("GCB of {:4} and {:4} = {:4}", 3, 27,gcd(3, 27));
    println!("GCB of {:4} and {:4} = {:4}", 44, 57, gcd(44, 57));
    println!("GCB of {:4} and {:4} = {:4}", 9, 3, gcd(9, 3));
    println!("GCB of {:4} and {:4} = {:4}", 56, 56, gcd(56, 56));
    println!("GCB of {:4} and {:4} = {:4}", 6790, 6797, gcd(6790, 6797));
}

//########## main program ##########
fn main()
{
    const FIRST_NUMBER: u32 = 62;
    const SECOND_NUMBER: u32 = 64;

    println!("GCD of {FIRST_NUMBER} and {SECOND_NUMBER} = {}", gcd(FIRST_NUMBER, SECOND_NUMBER));

    // test();
}