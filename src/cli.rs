use std::env::current_dir;
use std::io::{self, Write};

use crate::fs_operations;

pub fn run() -> Result<(), io::Error> {
    let current_directory = current_dir()?;
    loop {
        print_prompt(&current_directory);
        let user_input = get_user_input();

        let mut args = user_input.trim().split_whitespace();
        let command = args.next().unwrap_or("");
        let arguments: Vec<&str> = args.collect();

        match command {
            "ls" => {
                let include_hidden = arguments.contains(&"-a");
                fs_operations::list_files(&current_directory, include_hidden);
            }
            "mkdir" => {
                if let Some(dir_name) = arguments.first() {
                    fs_operations::create_directory(dir_name);
                } else {
                    println!("Please provide a directory name.");
                }
            }
            "touch" => {
                if let Some(file_name) = arguments.first() {
                    fs_operations::create_file(file_name);
                } else {
                    println!("Please provide a file name.");
                }
            }
            "rm" => {
                if let Some(item_name) = arguments.first() {
                    fs_operations::remove(item_name);
                } else {
                    println!("Please provide a file or directory name to remove.");
                }
            }
            "exit" => break,
            _ => println!("Invalid command"),
        }
    }
    Ok(())
}

fn print_prompt(current_directory: &std::path::PathBuf) {
    print!("{} $ ", current_directory.display());
    io::stdout().flush().unwrap();
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}
