use std::env;

fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = env::args().collect();

    let command_arg: String = args[1].clone();

    // Generate the commands here
    let mut commands: Vec<Command> = vec![];

    commands.push(Command::new("help", "Help"));
    commands.push(Command::new("toownfolder", "To own folder"));
    commands.push(Command::new("version", "Version"));

    match command_arg.as_str() {
        "help" => {
            println!("List of available commands:");
            for command in commands {
                println!("{}", command.display_description());
            }
        }
        "version" => {
            println!("Grunty-Rust v{}", VERSION)
        }
        _ => {
            let commands = get_command_by_name(commands, command_arg);
            for cmd in commands {
                let result = cmd.run();
                println!("Running command: {}", result);
            }
        }
    };
}

#[derive(PartialEq, Debug)]
struct Command {
    name: String,
    description: String,
}

impl Command {
    fn new(name: &str, description: &str) -> Command {
        Command {
            name: String::from(name),
            description: String::from(description),
        }
    }

    fn display_description(&self) -> String {
        String::from(&self.description)
    }

    fn run(&self) -> String {
        return String::from(&self.name);
    }
}

fn get_command_by_name(commands: Vec<Command>, command_arg: String) -> Vec<Command> {
    return commands
        .into_iter()
        .filter(|item| item.name == command_arg)
        .collect::<Vec<Command>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_by_name() {
        let cmd1 = Command::new("one", "First");
        let cmd2 = Command::new("two", "Second");
        let cmd3 = Command::new("three", "Third");

        let cmds = vec![cmd1, cmd2, cmd3];

        assert_eq!(
            vec![Command::new("two", "Second")],
            get_command_by_name(cmds, String::from("two"))
        );
    }
}
