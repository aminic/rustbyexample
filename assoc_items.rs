struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
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

fn difference<A, B, C>(container: &C) -> i32
    where C: Contains<A, B>
{
    container.last() - container.first()
}

fn main() {
    let number = 3;
    let digit = 10;
    let container = Container(number, digit);
    println!("Does container contain {} and {}: {}", &number, &digit,
             container.contains(&number, &digit));
    println!("Fist number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is: {}", difference(&container));
}