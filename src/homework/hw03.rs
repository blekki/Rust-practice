
fn draw_envelope(height: i32, width: i32) {
    let correlation: f32 = width as f32 / height as f32;

    let mut range = 0.0;

    for y in 0..height {
        for x in 0..width {
            let mut current_symbol = ' ';
            
            // border drawing
            let is_hor = x == 0 || x == width - 1;  // horizontal borders 
            let is_ver = y == 0 || y == height - 1; // vertical borders
            if is_hor || is_ver {
                current_symbol = '*';
            }

            // diagonale drawing
            let diagonal_one = (x) == (range as i32);               // left to right
            let diagonal_two = (x) == (width - 1 - (range as i32)); // right to left
            if diagonal_one || diagonal_two {
                current_symbol = '*';
            }
            
            // draw result
            print!("{current_symbol}");
        }
        println!();
        range += correlation;
    }
}

//########## MAIN FUNCTION ##########
fn main() {
    const HEIGHT: i32 = 13;
    const WIDTH:  i32 = 30;

    draw_envelope(HEIGHT, WIDTH);
}