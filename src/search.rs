use crate::models::models::Category;
use crate::utils::clear_console::clear_console;
use crate::utils::normalize::normalize;

use std::{collections::HashMap, fs};

/**
* Function to read user input from the console.
* @param prompt: The prompt message to display to the user.
* @return: The user input as a String.
*/
pub fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    std::io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a entrada");
    input.trim().to_string()
}

/*
* Function to build an inverted index from the categories.
* @param categories: A slice of Category structs.
* @return: A HashMap where the keys are normalized words and the values are vectors of tuples containing the category and item.
* This function normalizes the items and splits them into words, creating an index for efficient searching.
*/
pub fn build_inverted_index(categories: &[Category]) -> HashMap<String, Vec<(String, String)>> {
    let mut index: HashMap<String, Vec<(String, String)>> = HashMap::new();

    // For each category, iterate through its items
    // and split them into words
    for cat in categories {
        for item in &cat.items {
            // Normalize the item and split it into words
            let normalized_item = normalize(item);
            let words = normalized_item.split_whitespace().map(|s| s.to_string());

            for word in words {
                index
                    .entry(word)
                    .or_insert_with(Vec::new)
                    .push((cat.category.clone(), item.clone()));
            }
        }
    }

    // Normalize the keys of the index
    index
}

/**
* Function to search the index for a given key.
* @param index: A reference to the inverted index HashMap.
* @param key: The search term.
* @return: A vector of tuples containing the category and item for each match.
*/
pub fn search_index(
    index: &HashMap<String, Vec<(String, String)>>,
    key: &str,
) -> Vec<(String, String)> {
    index
        .iter()
        .filter(|(term, _)| term.contains(key))
        .flat_map(|(_, items)| items.clone())
        .collect()
}

/**
* Function to display the search results.
* @param results: A reference to a vector of tuples containing the category and item.
* This function groups the results by category and prints them in a formatted manner.
* It uses a HashMap to group the items by category and then iterates through the map to display the results.
* Each category is printed with its items listed below it.
* The function also handles the case where no results are found for a given search term.
*/
fn display_results(results: &[(String, String)]) {
    let mut grouped: HashMap<String, Vec<String>> = HashMap::new();

    // Group the results by category
    // and store the items in a vector
    for (category, item) in results {
        grouped
            .entry(category.clone())
            .or_insert_with(Vec::new)
            .push(item.clone());
    }

    // Print the results
    // If no results are found, print a message indicating that
    for (category, items) in grouped {
        println!("\nCategoria: {}", category);
        for item in items {
            println!("- {}", item);
        }
    }

    // Print a separator line
    println!();
}

/**
 * Function to search for items in a JSON file.
 * @param file_path: The path to the JSON file.
 * This function reads the JSON file, builds an inverted index, and allows the user to search for items.
 * It caches the results for faster subsequent searches.
 */
pub fn search(file_path: &str) {
    clear_console();

    // Double-check if the file path is empty
    if file_path.trim().is_empty() {
        println!("Caminho do arquivo não pode ser vazio.");
        return;
    }

    // Check if the JSON file exists
    let file_content = fs::read_to_string(file_path).expect("Erro ao ler o arquivo JSON.");
    let categories: Vec<Category> =
        serde_json::from_str(&file_content).expect("Erro ao parsear o JSON.");

    // Initialize the inverted index and cache
    // The inverted index is built from the categories
    // The cache is a HashMap that stores the search results
    // for faster subsequent searches
    let index = build_inverted_index(&categories);
    let mut cache: HashMap<String, Vec<(String, String)>> = HashMap::new();

    loop {
        let search_term = read_input(
            "Digite o termo a ser pesquisado (ou 'sair' para encerrar / 'limpar' para limpar):",
        );

        // Check for exit or clear commands
        // If the user types "sair", the loop breaks and the program ends
        // If the user types "limpar", the console is cleared
        // and the loop continues to prompt for a new search term
        match search_term.to_lowercase().as_str() {
            "sair" => break,
            "limpar" => {
                clear_console();
                continue;
            }
            _ => {}
        }

        let key = normalize(&search_term);
        println!();

        // Check if the search term is empty
        // If it is, prompt the user to enter a valid search term
        let results = if let Some(cached) = cache.get(&key) {
            println!("Resultados em cache para '{}':", search_term);

            // If the search term is found in the cache, use the cached results
            cached.clone()
        } else {
            let results = search_index(&index, &key);
            cache.insert(key.clone(), results.clone());
            if results.is_empty() {
                println!("Nenhum resultado encontrado para '{}'\n", search_term);
                continue;
            } else {
                println!("Resultados para '{}':", search_term);
            }
            results
        };

        // Display the results
        display_results(&results);
    }
}
