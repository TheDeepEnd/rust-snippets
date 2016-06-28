use std::io;

fn main() {

    let mut truth           = true;
    let mut input  = String::new();
    let mut counter            = 0;

    while truth{

        println!("1 is the world hello and 2 to exit");

        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim_right();
        counter = input.trim().parse()
            .expect("Please type a number!");

        match counter {
            1 => lets_print(),
            2 => truth = false,
            _ => println!("bad input")
        }
        input = String::new();
    }
}


fn lets_print(){
    println!("wuba luba dub dub");

    
}
