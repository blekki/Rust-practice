
fn gcd(first_value: u32, second_value: u32) -> u32
{
    // try to find half of higher value
    let mut lower = first_value;
    let mut higher= second_value;
    if second_value < first_value {
        higher = first_value;
        lower = second_value;
    }

    // pre-checking
    if lower == higher {
        return lower;
    }
    if lower == 0 {
        return 0;
    }
    if higher % lower == 0 {
        return lower;
    }

    // check everything number lower then "half of lower value"
    let start_point: u32 = lower / 2; // isn't so necessary line (can be replaced in the next operator)
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

//#[test #1]
fn test() {
    println!("<><><> test results <><><>");
    println!("GCD of {:4} and {:4} = {:4}", 12, 17, gcd(12, 17));
    println!("GCD of {:4} and {:4} = {:4}", 3, 27,gcd(3, 27));
    println!("GCD of {:4} and {:4} = {:4}", 44, 57, gcd(44, 57));
    println!("GCD of {:4} and {:4} = {:4}", 9, 3, gcd(9, 3));
    println!("GCD of {:4} and {:4} = {:4}", 56, 56, gcd(56, 56));
    println!("GCD of {:4} and {:4} = {:4}", 6790, 6797, gcd(6790, 6797));


    println!("= {:4}", gcd(10, 2));
    println!("= {:4}", gcd(1000000, 12312));
    println!("= {:4}", gcd(2, 10));
    println!("= {:4}", gcd(5, 10));
    println!("= {:4}", gcd(15, 20));
}

//########## main program ##########
fn main()
{
    const FIRST_NUMBER: u32 = 62;
    const SECOND_NUMBER: u32 = 64;

    println!("GCD of {FIRST_NUMBER} and {SECOND_NUMBER} = {}", gcd(FIRST_NUMBER, SECOND_NUMBER));

    test();
}