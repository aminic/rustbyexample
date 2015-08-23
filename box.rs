use std::mem;

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    let p: Point = origin();
    let rect: Rectangle = Rectangle { p1: origin(), p2: Point { x: 3.0, y: 4.0 } };
    let boxed_rect: Box<Rectangle> = Box::new(Rectangle { p1: origin(), p2: origin() });
    let boxed_p: Box<Point> = Box::new(origin());
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());
    println!("Point occupies {} bytes in the stack", mem::size_of_val(&p)); //16
    println!("Rectangle occupies {} bytes in the stack", mem::size_of_val(&rect)); //32
    println!("Boxed point occupies {} bytes in the stack", mem::size_of_val(&boxed_p)); //8
    println!("Boxed rectangle occupies {} bytes in the stack", mem::size_of_val(&boxed_rect)); //8
    println!("Boxed box occupies {} bytes in the stack", mem::size_of_val(&box_in_a_box)); //8

    let unboxed_p: Point = *boxed_p;
    println!("Unboxed point occupies {} bytes in the stack", mem::size_of_val(&unboxed_p)); //16
}
