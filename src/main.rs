use std::env;

fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    #[derive(Debug)]
    enum Command {
        Help,
        ToOwnFolder,
        Version,
        Unrecognized
    }

    let chosen = match command.as_str() {
        "help" => Command::Help,
        "toownfolder" => Command::ToOwnFolder,
        "version" => Command::Version,
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
        Command::Version => {
            println!("Grunty-Rust v{}", VERSION)
        }
        Command::Unrecognized => {
            println!("Command unrecognized")
        }
    }

}


