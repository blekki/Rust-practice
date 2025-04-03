const HEIGHT: i32 = 10;
const WIDTH: i32 = 40;


fn main() {
    
    let correlation: f32 = WIDTH as f32 / HEIGHT as f32;
    // let correlation  = WIDTH / HEIGHT;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            
            let mut current_symbol = ' ';

            let is_hor = x == 0 || x == WIDTH - 1;
            let is_ver = y == 0 || y == HEIGHT - 1;
            if is_hor || is_ver {
                current_symbol = '*';
            }

            
            let diagonal_one = (x as f32) >= correlation * (y as f32) && (x as f32) < correlation * ((y + 1) as f32);
            if diagonal_one {
                current_symbol = '*';
            }
            
            print!("{current_symbol}");
            
        }
        println!();
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