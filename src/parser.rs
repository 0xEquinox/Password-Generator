use crate::executioner;

pub fn parse(command: String) -> String{



    if command.contains("passgen"){

        if command.contains("-h"){
            println!("-a: all arguments (numbers, letters, symbols)");
            println!("-l: Length of password");
            println!("-n: Numbers in password generation toggle");
            println!("-L: Letters in password generation toggle");
            println!("-s: Symbols in password generation toggle");
            println!("-h: Help flag (prints all possible arguments)");
            return "Help concluded".to_string();
        }

        if letter_flag(&command) == false && number_flag(&command) == false && symbol_flag(&command) == false && all_flag(&command) == false && help_flag(&command) == false {
            return "Error: No arguments given".to_string();
        }

        return executioner::password_gen(find_length(&command), letter_flag(&command), number_flag(&command), symbol_flag(&command), all_flag(&command));
    }

    return "Error: Unknown Command".to_string();

}

fn find_length(command: &String) -> i32{

    let split = command.split(" ");

    let commands:Vec<&str> = split.collect();
    let mut length = 10;

    for i in 0..commands.len(){
        if commands[i].contains("-l"){
            length = commands[i + 1][0..commands[i + 1].len() - 1].parse().unwrap();
            break;
        }
    }
    return length;
}

fn number_flag(command: &String) -> bool{
    return if command.contains("-n") {
        true
    } else {
        false
    }
}

fn symbol_flag(command: &String) -> bool{
    return if command.contains("-s") {
        true
    } else {
        false
    }
}

fn letter_flag(command: &String) -> bool{
    return if command.contains("-L") {
        true
    } else {
        false
    }
}

fn all_flag(command: &String) -> bool{
    return if command.contains("-a") {
        true
    } else {
        false
    }
}

fn help_flag(command: &String) -> bool{
    return if command.contains("-h"){
        true
    }else {
        false
    }
}