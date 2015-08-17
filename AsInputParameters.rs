fn apply<F>(f: F)
    where F: FnOnce()
{
    f()
}

fn apply_to_3<F>(f: F) -> i32
    where F: Fn(i32) -> i32
{
    f(3)
}

fn main() {
    let greeting = "hello";
    let mut farwell = "goodbye".to_owned();
    let diary = || {
        println!("I said {}.", greeting);
        farwell.push_str("!!!");
        println!("Then I screamed {}.", farwell);
        println!("Now I can sleep. zzzzz");
        drop(farwell);
    };
    apply(diary);
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}