use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!("usage:
        match_args <strng>
        Check whether given string is the answer.
        match_args {{increase|decrease}} <integer>
        Increase or decrease give integer by one.");
}

fn main() {
    let args: Vec < String >= env::args().collect(); match args.len() {
        1 => {
            println!("My name is 'match_args'. Try passing some arguments!");
}
, 2 => {
    if &42 == &args[1].parse().unwrap() {
        println!("This is the answer!");
    } else {
        println!("This is not the answer.");
    }
},
       3 => {
    let cmd = &args[1];
    let num = &args[2];
    let number: i32 = match num.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            println!("error: second argument not an integer");
            help();
            return;
        },

    };
    match &cmd[..] {
        "increase" => increase(number),
        "decrease" => decrease(number),
        _ => {
            println!("error: invalid command");
            help();
        },
    }
},
            _ => {
    help();
}
}
}