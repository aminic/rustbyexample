fn main() {
    let color = "green";
    let print = || println!("color: {}", color);
    print();
    print();
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    inc();
    
    let movable = Box::new(3);
    println!("{:?}", movable);
    let consume = || {
        println!("`movable`:{:?}", movable);
        drop(movable);
    };
    consume();
    
}