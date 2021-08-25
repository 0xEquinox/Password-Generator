use std::io::stdin;

pub fn get_input() -> String{

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Did not enter a correct string");

    return input;

}

pub fn init(){

    println!("Windows Password Generator");
    println!("Copyright (C) Equinox Corporation. All rights reserved.");
    println!();

}