use rand::thread_rng;
use rand::Rng;

pub fn password_gen(length: i32, letters: bool, numbers: bool, symbols: bool, all: bool) -> String{

        let chars = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "1", "2", "3", "4", "5", "6", "7", "8", "9", "!", "#", "$", "%", "?", "/", "\\", "*", ",", "."];
        let mut password: Vec<&str> = Vec::new();
        let mut password_string = String::new();
        let mut rng = thread_rng();
        let mut choices: Vec<i32> = Vec::new();

        for _ in 0..length{
            if all == true{
                choices.push(rng.gen_range(0, 52));
                choices.push(rng.gen_range(52, 63));
                choices.push(rng.gen_range(63, chars.len() as i32));
            }else {
                if letters == true {
                    choices.push(rng.gen_range(0, 52));
                }
                if numbers == true {
                    choices.push(rng.gen_range(52, 61));
                }
                if symbols == true {
                    choices.push(rng.gen_range(61, chars.len() as i32));
                }
            }

            password.push(chars[choices[rng.gen_range(0, choices.len())] as usize]);
        }

        for i in 0..password.len(){
            password_string = password_string + password[i];
        }

    return password_string;
}
