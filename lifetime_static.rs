static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        let static_string = "In read-only memory";
        println!("static_string holds: {}", static_string);
    }
    println!("but now it's gone.");
    println!("NUM: {} is still around though!", NUM);
    {
        let i = 9;
        let coerced_num = coerce_static(&i);
        println!("coerced_num: {}", coerced_num);
    }
}