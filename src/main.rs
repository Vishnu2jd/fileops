mod cli;
mod fs_operations;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {}", e);
    }
}
