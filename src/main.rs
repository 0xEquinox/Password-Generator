
mod interface;
mod parser;
mod executioner;

fn main() {

    interface::init();

    loop{
        let password = parser::parse(interface::get_input());
        println!("{}", password);
    }
}
