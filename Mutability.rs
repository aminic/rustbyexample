fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);
    //_immutable_binding += 1;//error

    println!("/////////////////////");
    

    let im_box = Box::new(5u32);
    println!("immutable_box contains {}", im_box);
    let mut mut_box = im_box;
    println!("mutable_box contained {}", mut_box);
    *mut_box = 4;
    println!("mutable_box now contains {}", mut_box);
}