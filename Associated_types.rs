struct Container(i32, i32);

trait Contains {
    type A;
    type B;
    fn contains(&self, &Self::A, &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;
    fn contains(&self, number: &i32, digit: &i32) -> bool {
        (&self.0 == number) && (&self.1 == digit)
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number = 3;
    let digit = 10;
    let container = Container(number, digit);
    println!("Does container contain {} and {}: {}", &number, &digit,
             container.contains(&number, &digit));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is: {}", difference(&container));
}