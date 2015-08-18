fn function(){
    println!("called `function()`");
}

mod my{
    #[allow(dead_code)]
    fn function(){
        println!("called `my::function()`");
    }

    mod nested{
        #[allow(dead_code)]
        fn function(){
            println!("call `my::nested::function()`");
        }
    }
}

fn main() {
    function();
    //my::function();
    my::nested::function();

}