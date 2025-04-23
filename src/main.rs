mod models;
mod utils;

mod search;
use crate::utils::clear_console::clear_console;

const JSON_FILE: &str = "src/data/categories.json";

fn main() {
    let file_path = JSON_FILE;

    // Initialize the console
    clear_console();

    // Check if the JSON file exists
    if !std::path::Path::new(file_path).exists() {
        println!("Arquivo JSON não encontrado: {}", file_path);
        return;
    }

    // Start the search
    search::search(file_path);
}
