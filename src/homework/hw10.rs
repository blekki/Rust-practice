fn is_palindrome(x: u32) -> bool {
    let mut vec: Vec<u32> = Vec::new();
    let mut x_copy = x;
    while x_copy > 0 {
        vec.push(x_copy % 10);
        x_copy /= 10;
    }

    // checking
    for i in 0..=vec.len()/2 {
        let first = vec[i];
        let last = vec[vec.len() - 1 - i];
        if first != last { 
            return false;
        }
    }
    return true;
}

// ########## main program ##########
fn main()
{
    let tests = [
        (123,    false),
        (121,    true),
        (1231,   false),
        (19391,  true),
        (1,      true),
        (785587, true),
        (374,    false),
        (77,     true),
        (300103, false),
    ];

    tests.iter().for_each(|(n, exp)|{
        assert_eq!(is_palindrome(*n), *exp);
        println!("{n:6}: {}", is_palindrome(*n));
    });
}