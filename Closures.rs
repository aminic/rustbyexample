fn main() {
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("{:?}", i);
    println!("function: {}", function(i));
    println!("{:?}", i);
    println!("annotated closure: {}", closure_annotated(i));
    println!("{:?}", i);
    println!("inferred closure: {}", closure_inferred(i));
    println!("{:?}", i);

    let one = || 1;
    println!("closure returning one: {}", one());

    let professor_x = "Charles Xavier";

    let print = || println!("professor X's name is: {}", professor_x);
    print();
}