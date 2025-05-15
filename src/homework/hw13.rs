struct Point {
    x: i32,
    y: i32,
}
struct Rectangle {
    a: Point,
    b: Point,
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 }, // 3
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 3, y: 8 }, // 7
            b: Point { x: 10, y: 6 },
        },
        Rectangle {
            a: Point { x: 1, y: 2 },
            b: Point { x: 20, y: 2 },
        },
    ]
}

// fn calc_area(vec: Vec<Rectangle>) -> i32{
//     return 2;
// }

fn collision(cur: &Rectangle, pre: &Rectangle) -> Rectangle {
    let mut result: Rectangle = Rectangle{a: Point{x: 0, y: 0}, 
                                          b: Point{x: 0, y: 0}};
                                     
    if (pre.a.x..=pre.b.x).contains(&cur.a.x) {
        if (pre.a.x..=pre.b.x).contains(&cur.b.x) {
            result.a.x = cur.a.x;
            result.b.x = cur.b.x;
        }
        else {
            result.a.x = cur.a.x;
            result.b.x = pre.b.x;
        }
    }
    else {
        if (pre.a.x..=pre.b.x).contains(&cur.b.x) {
            result.a.x = pre.a.x;
            result.b.x = cur.b.x;
        }
        else {
            if cur.a.x < pre.a.x && cur.b.x > pre.b.x {
                result.a.x = pre.a.x;
                result.b.x = pre.b.x;
            }
            // break;
        }
    }
    return result;
}

fn area_occupied(vec: Vec<Rectangle>) -> i32 {
    let mut coll: Vec<Rectangle> = vec![];
    let mut area: i32 = 0;
    for cur in 0..vec.len() {
        area += (vec[cur].b.x - vec[cur].a.x).abs();
        
        for pre in 0..cur {
            // let curr = [vec[cur].a.x, vec[cur].b.x];
            // let prev = [vec[pre].a.x, vec[pre].b.x];
            // let mut un = [0, 0];
            let space = collision(&vec[cur], &vec[pre])
            area -= (&space.b.x - space.a.x).abs();
            coll.push(space);
            

            // if (prev[0]..=prev[1]).contains(&curr[0]) {
            //     if (prev[0]..=prev[1]).contains(&curr[1]) {
            //         un[0] = curr[0];
            //         un[1] = curr[1];
            //     }
            //     else {
            //         un[0] = curr[0];
            //         un[1] = prev[1];
            //     }
            // }
            // else {
            //     if (prev[0]..=prev[1]).contains(&curr[1]) {
            //         un[0] = prev[0];
            //         un[1] = curr[1];
            //     }
            //     else {
            //         if curr[0] < prev[0] && curr[1] > prev[1] {
            //             un[0] = prev[0];
            //             un[1] = prev[1];
            //         }
            //         // break;
            //     }
            // }
            // area -= (un[1] - un[0]).abs();
        }
        // area += (vec[j].b.x - vec[j].a.x).abs() * (vec[j].b.y - vec[j].a.y).abs();
    }
    return area;
}

fn main() {
    let area = area_occupied(test_data());
    println!("{}", area);
}