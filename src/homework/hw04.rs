
fn drawRhomb(width: i32, height: i32) -> ()
{
    let correlation: f32 = (width as f32) / (height as f32);

    // check every line and column
    for a in 0..height {
        for b in 0..width {
            let mut current_symbol: char = ' ';
            
            // positions depend on a rhomb 
            let mut y = (a - (height / 2)).abs();
            let mut x = (b - (width  / 2)).abs();
                    x = (x as f32 / correlation) as i32; // add a correlation to the "x" position

            // check does position in the rhomb space
            let inside_rhomb = (x + y) < (height as f32 / 2.0).round() as i32;
            if inside_rhomb {
                current_symbol = '*';
            }

            // print result
            print!("{current_symbol}");
        }
        println!();
    }
}

fn main() {
    const HEIGHT: i32 = 11;
    const WIDTH:  i32 = 11;

    drawRhomb(WIDTH, HEIGHT);
}