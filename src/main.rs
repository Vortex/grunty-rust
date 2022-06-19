use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    #[derive(Debug)]
    enum Command {
        Help,
        ToOwnFolder,
        Unrecognized
    }

    let chosen = match command.as_str() {
        "help" => Command::Help,
        "toownfolder" => Command::ToOwnFolder,
        _ => Command::Unrecognized
    };

    println!("You have chosen the command: {:?}", chosen);

    match chosen {
        Command::ToOwnFolder => {
            println!("Implement stuff here")
        }
        Command::Help => {
            println!("List all commands here")
        },
        Command::Unrecognized => {
            println!("Command unrecognized")
        }
    }

}


