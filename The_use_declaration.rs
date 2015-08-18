fn function() {
    println!("called `function()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    use deeply::nested::function as ohter_function;
    ohter_function();
    println!("Entering block");
    {
        use deeply::nested::function;
        function();
        println!("Leaving block");
    }
    function();
}