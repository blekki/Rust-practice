const HEIGHT: u32 = 6;
const WIDTH: u32 = 10;


fn main() {
    
    
    for y in 1..=HEIGHT {
        for x in 1..=WIDTH {
            let is_hor = x == 1 || x == WIDTH;
            let is_ver = y == 1 || y == HEIGHT;
            
            let mut current_symbol = ' ';
            if is_hor || is_ver {
                current_symbol = '*';
            }

            print!("{current_symbol}");
            
        }
        println!();
    }
}