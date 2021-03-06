use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x.add(rhs.x),
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub<Vec2<T>> for Vec2<T>
    where T: Sub<T, Output = T>
{
    type Output = Vec2<T>;
    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Vec2<T>
    where T: Add<T, Output = T> + Mul<T, Output = T>
{
    fn dot(self, rhs: Vec2<T>) -> T {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

fn main() {
    let v1 = Vec2 { x: 1.2_f32, y: 3.4 };
    let v2 = Vec2 { x: 5.6_f32, y: 7.8 };
    println!("{:?}+{:?}={:?}", v1, v2, v1 + v2);
    println!("{:?}-{:?}={:?}", v1, v2, v1 - v2);
    println!("{:?}*{:?}={:?}", v1, v2, v1.dot(v2));
}