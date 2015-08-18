fn function(){
    println!("called `function()`");
}

mod my{
    //a public function
    pub fn function(){
        println!("called `my::function()`");
    }
    //a private function
    fn private_function(){
        println!("called `my::private_function()`");
    }
    //items can access other items in the same module
    pub fn indirect_access(){
        print!("called `my::indirect_access()`");
        private_function();
    }
    //a public module
    pub mod nested{
        pub fn function(){
            println!("called `my::nested::function()`");
        }
        #[allow(dead_code)]
        fn private_function(){
            println!("called `my::nested::private_function()`");
        }
    }
    //a inaccessible
    mod inaccessible{
        #[allow(dead_code)]
        pub fn public_function(){
            println!("called `my::inaccessible::public_function()`");
        }
    }
}


fn main() {
    my::function();
    function();
    my::indirect_access();
    my::nested::function();
    
}

