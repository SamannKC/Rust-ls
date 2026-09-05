use colored::Colorize;
use std::{env, fs};

fn main() {
    let current_directory = env::current_dir()
        .expect("Couldnt determine current directory.");

    let entries = fs::read_dir(&current_directory)
        .expect("Couldnt read current directory.");

    for entry in entries {
        let entry = entry
            .expect("Couldnt read entry");

        let file_type = entry
            .file_type()
            .expect("Couldnt determine filetype.");

        let name = entry.file_name();
        let name = name.to_string_lossy();

        println!("{}", {
            if file_type.is_dir() {
                name.blue().bold()
            } else {
                name.white()
            }
        });
    }

    println!("{}", current_directory.display());
}
