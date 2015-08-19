fn main() {
    let mut i = 5i32;
    {
        let ref_i = &i;
        //i = 4;//freezing
    }
    i = 4;
}