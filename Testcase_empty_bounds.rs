struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {
    // Add code here
}
trait Blue {
    // Add code here
}
impl Red for Cardinal {

}

impl Blue for BlueJay {

}
fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn main() {
    let c = Cardinal;
    let b = BlueJay;
    let t = Turkey;

    println!("a cardinal is {}", red(&c));
    println!("a blue jay is {}", blue(&b));
    //println!("a turkey is {}", red(&t));
}