use colored::Colorize;
use std::{env, fs};
use terminal_size::{terminal_size, Width};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args{
    // shows hidden files
    #[arg(short)]
    a: bool, 

    #[arg(short)]
    grep: Option<String>,
}

// struct for items in directory 
struct Items{
    name: String,
    is_dir: bool,
}

fn main() {
    let args = Args::parse();

    let current_directory = env::current_dir()
        .expect("Couldnt determine current directory.");

    let entries = fs::read_dir(&current_directory)
        .expect("Couldnt read current directory.");

    let mut names: Vec<Items> = Vec::new();

    // Populate the struct
    populate(&mut names, entries, &args);

    // divide into columns for better readability
    let max_width = names.iter()
        .filter(|item|{
            match &args.grep {
                Some(grep) => item.name.to_lowercase().contains(&grep.to_lowercase()),
                None => true,
            }
        })
        .map(|item| item.name.len())
        .max()
        .unwrap_or(0);

    let terminal_width = match terminal_size() {
        Some((Width(width), _)) => width as usize,
        None => 80,
    };

    let column_width = max_width + 3;
    let columns = (terminal_width/column_width).max(1);

    // display the items
    display(&names, columns, column_width, &args);

}

fn populate(names: &mut Vec<Items>, entries: fs::ReadDir, args: &Args){

    for entry in entries {
        let entry = entry
            .expect("Couldnt read entry");

        let file_type = entry
            .file_type()
            .expect("Couldnt determine filetype.");

        let name = entry.file_name();
        let name = name.to_string_lossy().to_string();
        if !args.a {
            if name.starts_with("."){
                continue;
            }
        }

        names.push(Items{
            name: name,
            is_dir: file_type.is_dir(),
        })
    }

}


fn display(names: &[Items], columns: usize, column_width: usize, args: &Args){

    for (i, item) in names.iter()
        .filter(|item|{match &args.grep {
                Some(grep) => item.name.to_lowercase().contains(&grep.to_lowercase()),
                None => true,
            }
        }).enumerate(){

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
