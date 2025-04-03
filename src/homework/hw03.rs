const HEIGHT: i32 = 17;
const WIDTH: i32 = 43;


fn main() {
    
    let correlation: f32 = WIDTH as f32 / HEIGHT as f32;
    // let correlation  = WIDTH / HEIGHT;'

    // let mut range = correlation;
    let mut range = 0.0;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            
            let mut current_symbol = ' ';

            let is_hor = x == 0 || x == WIDTH - 1;
            let is_ver = y == 0 || y == HEIGHT - 1;
            if is_hor || is_ver {
                current_symbol = '*';
            }

            
            // let diagonal_one = (x as f32) >= (correlation *  y as f32) &&
            //                          (x as f32) <  (correlation * (y + 1) as f32);
            // let diagonal_two = (y as f32) >= (1.0 / correlation *  x as f32) &&
            //                          (y as f32) <  (1.0 / correlation * (x + 1) as f32);

            // let diagonal_one = (x as f32) >= (correlation * (HEIGHT - y - 1) as f32) &&
            //                          (x as f32) <  (correlation * (HEIGHT - y) as f32);

            // let diagonal_one = (x as f32) < (range) &&
            //                          (x as f32) >= (range - correlation);
            // let diagonal_two = (x as f32) < (WIDTH as f32 - range + correlation) &&
            //                          (x as f32) >= (WIDTH as f32 - range);

            let diagonal_one = (x) == (range as i32);
            let diagonal_two = (x) == (WIDTH - 1 - (range as i32));

            // if diagonal_one {
            if diagonal_one || diagonal_two {
                current_symbol = '*';
            }


            // if diagonal_one || diagonal_two {
            // if diagonal_one {
            //     current_symbol = '*';
            // }
            
            print!("{current_symbol}");
            
        }
        println!();

        range += correlation;
    }

    // let mut str = String::from("");
    // str.chars().nth(2).unwrap() = '2';
    // for _ in 0..4 {
    //     str.push('a');
    // }
    // println!("{str}");

    // let mut sli  = "sdfsdf";
    // sli.chars().;
    // println!("{sli}");
}