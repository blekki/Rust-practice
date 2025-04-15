
fn draw_levels(mut level: u32, max_level: u32) {
    if level >= max_level {
        return;
    }

    level += 1;
    
    for a in 1..=level {
        // draw space
        for _ in 0..(max_level - a) {
            print!(" ");
        }
        // draw tree elements
        for _ in 0..(a * 2 - 1) {
            print!("*");
        }
        // draw space again (can be commented)
        for _ in 0..(max_level - a) {
            print!(" ");
        }
        
        println!(); // jump to the next line
    }
    // come draw next tree level
    draw_levels(level, max_level);
}

fn draw_tree(max_level: u32) {
    const ZERO_LEVEL: u32 = 0;
    draw_levels(ZERO_LEVEL, max_level);
}

// <><><><> MAIN PROGRAM <><><><>
fn main()
{
    const MAX_LEVEL: u32 = 4;
    draw_tree(MAX_LEVEL);
}