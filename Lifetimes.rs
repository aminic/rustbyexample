fn main() {
    let i = 3;
    {
        let b1 = &i;
        println!("borrow1: {}", b1);
        println!("i: {:?}",i );
    }
    {
        let b2 = &i;
        println!("b2: {}", b2);
    }
}