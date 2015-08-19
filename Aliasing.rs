struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut p = Point { x: 0, y: 0, z: 0 };
    {
        let borrowed_point = &p;
        let another_borrow = &p;
        println!("Point has coordinates: ({:?}, {:?}, {:?})", borrowed_point.x, another_borrow.y,
                 p.z);

    }
    {
        (&mut p).x = 6;
        println!("Point.x: {:?}", p.x);
        ////////////////////////
        let mut_borrow = &mut p;
        println!("mut_borrow.x : {:?}", mut_borrow.x);
        mut_borrow.x = 5;
        println!("mut_borrow.x : {:?}", mut_borrow.x);

    }
    println!("Point now has coordinates: ({}, {}, {})", p.x, p.y, p.z);
}