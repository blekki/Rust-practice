fn is_prime_number(number: u32) -> bool {

    if number == 0 {
        return false;
    }

    // check is it a prime number
    for divider in 2..number { // "1" we don't need, so skip this iteration
        if number % divider == 0 {
            return false;
        }
    }
    
    return true;
}

// ########## main program ##########
fn main()
{
    for i in 0..30 {
        println!("{i:3}.| {}", is_prime_number(i));
    }
}