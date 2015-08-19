fn coerce_fist<'a: 'b, 'b>(x: &'a i32, _: &'b i32) -> &'b i32 {
    x
}

fn main() {
    let x = 800;
    let y = 8;
    let borrow_big = &x;
    {
        let borrow_small = &y;
        let coerced = coerce_fist(borrow_big, borrow_small);
        println!("coerced is {}", coerced);
    }
}