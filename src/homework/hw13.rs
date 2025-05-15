// ### structures ###
struct Point {
    x: i32,
    y: i32,
}
struct Rectangle {
    a: Point,
    b: Point,
}

// ### functions ###
// calulate simple area
fn calc_area(rectangle: &Rectangle) -> i32{
    return (rectangle.b.x - rectangle.a.x).abs() * (rectangle.b.y - rectangle.a.y).abs();
}
// find deep areas collision
fn collision_finder(cur: &Rectangle, pre: &Rectangle) -> Rectangle{
    let mut result: Rectangle = Rectangle{a: Point{x: 0, y: 0},
                                          b: Point{x: 0, y: 0}};
    // possible collision
    result.a.x = if cur.a.x < pre.a.x {pre.a.x} else {cur.a.x}; // a point (X)
    result.a.y = if cur.a.y > pre.a.y {pre.a.y} else {cur.a.y}; // a point (Y)
    result.b.x = if cur.b.x > pre.b.x {pre.b.x} else {cur.b.x}; // b point (X)
    result.b.y = if cur.b.y < pre.b.y {pre.b.y} else {cur.b.y}; // b point (Y)
    // chech is collision right
    if result.a.x > result.b.x || result.a.y < result.b.y {
        return Rectangle{a: Point{x: 0, y: 0},
                         b: Point{x: 0, y: 0}};
    }
    // result
    return result;
}

fn area_occupied(vec: Vec<Rectangle>) -> i32 {
    let mut area: i32 = 0; // future result
    // calculate every areas
    for cur in 0..vec.len() {
        area += calc_area(&vec[cur]);
        println!("area: {}", calc_area(&vec[cur]));
        
        for pre in 0..cur { // check for collision
            let coll = collision_finder(&vec[cur], &vec[pre]);
            // debug
            // print!("collision: [x:{} y:{} | ", coll.a.x, coll.a.y);
            // print!("x:{} y:{}]\n", coll.b.x, coll.b.y);
            area -= calc_area(&coll);
        }
    }
    return area;
}
// other
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        }
    ]
}

// ####### main program #######
fn main() {
    let full_area = area_occupied(test_data());
    println!("{}", full_area);
}