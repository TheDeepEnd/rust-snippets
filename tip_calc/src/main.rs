use std::io;
// use std::ops::Div;
// use std::f64;

fn main() {
    println!("I am a tip calculator");
    println!("i will calculate your tip from your total");
    println!("So lets get started");
    println!("Your total is:$ ");

    let mut total = String::new();
    let mut percantage = String::new();

    io::stdin().read_line(&mut total).expect("Failed to read line");
    let total: f64 = total.trim().parse()
        .expect("Please type a number!");


    println!("The total you entered is(in dollars): {}", total);

    println!("The percantage that you want to give (in decimals): ");

    io::stdin().read_line(&mut percantage).expect("Failed to read line");
    let percantage: f64 = percantage.trim().parse()
        .expect("Pleasz type a number!");

    let cent = total * percantage;

    let new_total = total + cent;

    println!("The percantage that you want to give is: {} %", percantage);

    println!("The recommended tip that you want to give is (in dollars): {} ", cent);

    println!("your new total is:$ {} ", new_total);
}
