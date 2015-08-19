use std::ops::Add;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
struct Inch;
#[derive(Debug, Clone, Copy)]
struct Mm;

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;
    fn add(self, rhs: Length<Unit>) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {
    let one_foot: Length < Inch >= Length(12.0, PhantomData); 
    let one_meter: Length < Mm >= Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot; let two_meters =
        one_meter + one_meter;

    println!("one foot + one foot = {:?}", two_feet); 
    println!("one meter + one meter = {:?}",  two_meters);
}