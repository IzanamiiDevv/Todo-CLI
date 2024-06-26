mod modules;
use modules::commands;
use modules::displays;

fn main() {
    const FILE_PATH: &str = "data.txt";
    displays::print_banner();
    println!("Welcome to my Todo Console Application Created in Rust Programming Language\nTo get started write \"h\" or \"help\" to show all the available command.");

    let mut is_awake: bool = true;
    while is_awake {
        make_request(&mut is_awake);
    }

}

fn make_request(awake: &mut bool) {
    println!("Write Some Command");
    let mut response: String = String::new();
    std::io::stdin().read_line(&mut response).expect("Error on Reading the Response");
    let command: &str = response.trim();

    match command {
        "show" => {commands::show()}
        "h" | "help" => {commands::help()}
        "exit" => {
            *awake = false;
        }
        _=> {println!("Command not Found!")}
    }
}