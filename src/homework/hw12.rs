// fast vector printing function
fn vec_print(vec: &Vec<u32>) {
    for i in vec {
        print!("{} ", i);
    }
    print!("\n");
}

// get average vector value
fn count_permutation(ship: &Vec<u32>) -> f32{
    let mut result: f32 = 0.0;
    for a in ship.iter() {
        result += *a as f32;
    }
    return result / ship.len() as f32;
}

// better then default permutation
fn gen_shipments_plus(shipments: &Vec<u32>) -> Vec<u32> {
    let ave = count_permutation(shipments);
    let mut result: Vec<u32> = Vec::new();
    // permutation
    let current: u32 = ave as u32;
    let differ: f32 = ave % 1.0;
    let mut plus: f32 = 0.0;
    for _ in 0..shipments.len() {
        plus += differ;
        result.push(current + plus as u32);
        if !!!(plus < 1.0) { plus -= 1.0; }
    }
    return result;
}

// basic permutation
fn gen_shipments(shipments: &Vec<u32>) -> Vec<u32> {
    let ave = count_permutation(shipments);
    let mut result: Vec<u32> = Vec::new();
    for _ in 0..shipments.len() {
        result.push(ave as u32);
    }
    return result;
}

// ########## main program ##########
fn main() {
    let test: [Vec<u32>; 5] = [
        [1, 3, 3].to_vec(),
        [3, 6, 1, 3, 9, 0, 2].to_vec(),
        [8, 1, 32, 2, 1, 1, 2].to_vec(),
        [8, 2, 3, 2, 9, 5].to_vec(),
        [1, 555].to_vec()
    ];

    test.iter().for_each(|ships| {
        print!("before: ");
        vec_print(ships);
        print!("after basic function: ");
        vec_print(&gen_shipments(&ships));
        print!("after function-plus: ");
        vec_print(&gen_shipments_plus(&ships));
        println!();
    });
}