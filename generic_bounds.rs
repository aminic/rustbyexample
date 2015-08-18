use std::fmt::Debug;

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rect = Rectangle { length: 3.0, height: 4.0 };
    let _trian = Triangle { length: 3.0, height: 4.0 };
    print_debug(&rect);
    println!("Area: {:?}", area(&rect));
}