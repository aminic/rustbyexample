struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self){self.0+=1}
    fn print<'a>(&'a self){
        println!("print: {}",self.0 );
    }
}

fn main() {
    let mut o=Owner(18);
    o.add_one();
    o.print();
}