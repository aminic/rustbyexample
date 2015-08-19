#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let c = 'y';
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);
    let p = Point { x: 0, y: 0 };
    let _copy_of_x = {
        let Point { x: ref ref_to_x, y: _ } = p;
        *ref_to_x
    };
    let mut mut_p = p;
    {
        let Point { x: _, y: ref mut mut_ref_to_y } = mut_p;
        *mut_ref_to_y = 1;
    };
    println!("point is ({},{})", p.x, p.y);
    println!("mutable_point is ({},{})", mut_p.x, mut_p.y);
    println!("point is ({},{})", p.x, p.y);
    let tuple = (Box::new(5u32), 3u32);
    println!("tuple is {:?}", tuple);
}