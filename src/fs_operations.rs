use std::fs;
use std::path::PathBuf;

pub fn list_files(directory: &PathBuf, include_hidden: bool) {
    match fs::read_dir(directory) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                if let Some(file_name) = entry.file_name().to_str() {
                    if include_hidden || !file_name.starts_with('.') {
                        print!("{} ", file_name);
                    }
                }
            }
            println!();
        }
        Err(e) => print_error(&format!("Error reading directory: {}", e)),
    }
}

pub fn create_directory(directory_name: &str) {
    match fs::create_dir(directory_name) {
        Ok(_) => println!("Directory '{}' created successfully.", directory_name),
        Err(e) => print_error(&format!(
            "Failed to create directory '{}': {}",
            directory_name, e
        )),
    }
}

pub fn create_file(file_name: &str) {
    match fs::File::create(file_name) {
        Ok(_) => println!("File '{}' created successfully.", file_name),
        Err(e) => print_error(&format!("Error creating file '{}': {}", file_name, e)),
    }
}

pub fn remove(item_name: &str) {
    match fs::remove_file(item_name) {
        Ok(_) => println!("File '{}' deleted successfully.", item_name),
        Err(_) => {
            if let Err(e) = fs::remove_dir_all(item_name) {
                print_error(&format!("Error deleting '{}': {}", item_name, e));
            } else {
                println!("Directory '{}' deleted successfully.", item_name);
            }
        }
    }
}

fn print_error(message: &str) {
    eprintln!("{}", message);
}
