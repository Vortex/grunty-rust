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

// use std::env;
// // use std::fs::DirEntry;
// // use walkdir::WalkDir;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let path = &args[1];

//     println!("Checking path: {}", path);
//     // So for starters this should do the simplest thing:
 
//     // 1. List all mp4s in a directory
//     // 2. Create subdirs based on the names of those mp4s
//     // 3. Move the mp4s in their own directory
//     // for file in fs::read_dir(path).unwrap() {
//     //     println!("{}", file.unwrap().path().display());
//     // }

//     // for file in WalkDir::new(path)
//     //     .into_iter()
//     //     .filter_entry(|file| is_mp4(file))
//     // {
//     //     if file.metadata().unwrap().is_file() {
//     //         println!("{}", file.path().display());
//     //     }
//     // }

//     // let walker = WalkDir::new(path).into_iter();
//     // for entry in walker.filter_map(|e| e.ok() && is_mp4(e)) {
//     //     println!("{}", entry.path().display());
//     // }

//     // fn is_mp4(entry: &DirEntry) -> Result<bool, bool> {
//     //     entry
//     //         .file_name()
//     //         .to_str()
//     //         .map(|s| {
//     //             println!("{}", s);
//     //             return s.contains("mp4");
//     //         })
//     //         .unwrap_or(Result<Ok(true), Err(false)>)
//     // }
// }
