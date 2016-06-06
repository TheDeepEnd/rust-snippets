// use std::mem;
// use str::is_char_boundary
// use std::slice;

#![feature(str_char)]
use std::io;
use std::str;
use std::string::String;
use std::usize;
use std::io::Write;
fn main() {

    println!("Why, hello there. i don't think we met.");
    println!("would you kindly please tell me your name?");
    let mut name_input = String::new();

    io::stdin()
    .read_line(&mut name_input)
    .expect("Failed to read line");

    let name_counter = space_counter(&name_input);

    println!("ah, yes. master {}", name_input.trim());
    println!("i am curious do you have {} letters in your whole name?", name_counter);
    println!("well, best if we be off. so shall we get started then?");

    let mut intake_noun = String::new();
    let mut intake_adjetive  = String::new();
    let mut intake_adverb = String::new();
    let mut intake_verb = String::new();
    let mut mad_lib = Vec::with_capacity(10);


        println!("Please enter a Noun");
        io::stdin().read_line(&mut intake_noun).expect("Failed to read line");

        mad_lib.push(&intake_noun);


        println!("Please enter an Adjective");
        io::stdin().read_line(&mut intake_adjetive).expect("Failed to read line");
        mad_lib.push(&intake_adjetive);

        println!("Please enter an Adverb");
        io::stdin().read_line(&mut intake_adverb).expect("Failed to read line");
        mad_lib.push(&intake_adverb);

        println!("Please enter a Verb");
        io::stdin().read_line(&mut intake_verb).expect("Failed to read line");
        mad_lib.push(&intake_verb);

        //i still don't know how to strip the end of the new line
        print!("the {} {} {} {}", mad_lib[1].trim(), mad_lib[0].trim()
        , mad_lib[2].trim(), mad_lib[3]);
        io::stdout().flush().unwrap();

}


fn space_counter(stringy: &std::string::String) ->usize{
    #![allow(deprecated)]
    let len = stringy.len();
    let spaces = ' ';
    let mut c = stringy.char_at(0);
    let mut count = 0;

    for b in 0..len{

        if c == spaces{
            count += 1;

        }


        c = stringy.char_at(b);
    }
    //for the stupid new line :/ fucking bull shit
    count +=1;
    return len - count;
}
