use colored::Colorize;
use std::{env, fs};
use terminal_size::{terminal_size, Width};

// struct for items in directory 
struct Items{
    name: String,
    is_dir: bool,
}

fn main() {
    let current_directory = env::current_dir()
        .expect("Couldnt determine current directory.");

    let entries = fs::read_dir(&current_directory)
        .expect("Couldnt read current directory.");

    let mut names: Vec<Items> = Vec::new();

    // Populate the struct
    populate(&mut names, entries);

    // divide into columns for better readability
    let max_width = names.iter()
        .map(|item| item.name.len())
        .max()
        .unwrap_or(0);

    let terminal_width = match terminal_size() {
        Some((Width(width), _)) => width as usize,
        None => 80,
    };

    let column_width = max_width + 3;
    let columns = terminal_width/column_width;

    // display the items
    display(names, &columns, &column_width);

}

fn populate(names: &mut Vec<Items>, entries: fs::ReadDir){
    for entry in entries {
        let entry = entry
            .expect("Couldnt read entry");

        let file_type = entry
            .file_type()
            .expect("Couldnt determine filetype.");

        let name = entry.file_name();
        let name = name.to_string_lossy().to_string();
        if name.starts_with("."){
            continue;
        }

        names.push(Items{
            name: name,
            is_dir: file_type.is_dir(),
        })
    }

}


fn display(names: Vec<Items>, columns: &usize, column_width: &usize){
    for (i, item) in names.iter().enumerate(){
        if item.is_dir{
            print!(
                "{:<width$}",format!("{}/", item.name).blue().bold(),
                width = column_width
            );
        } else {
            print!(
                "{:<width$}", format!("{}", item.name).white(),
                width = column_width
            );
        }

        if (i + 1) % columns == 0 {
            println!();
        }
    }

}
