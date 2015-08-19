fn create_box() {
    let _function_box = Box::new(3i32);
    println!("created function box");
}

fn main() {
    let _boxed_int = Box::new(5i32);
    {
        let _short_lived_box = Box::new(4i32);
    }
    for _ in 0u32..1_000 {
        create_box();
    }
}