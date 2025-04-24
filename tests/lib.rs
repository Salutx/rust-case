#[cfg(test)]
mod build_inverted_index_tests {
    use rust_case::{models::models::Category, search::build_inverted_index};

    fn mock_categories() -> Vec<Category> {
        vec![
            Category {
                category: String::from("Frutas"),
                items: vec![String::from("Maçã!"), String::from("Banana123")],
            },
            Category {
                category: String::from("Ferramentas"),
                items: vec![String::from("Martelo."), String::from("Chave-De-Fenda")],
            },
        ]
    }

    #[test]
    fn test_build_inverted_index() {
        let categories = mock_categories();
        let index = build_inverted_index(&categories);

        assert!(index.contains_key("maca"));
        assert!(index.contains_key("banana123"));
        assert!(index.contains_key("martelo"));
        assert!(index.contains_key("chavedefenda"));

        let maca_entry = index.get("maca").unwrap();
        assert!(maca_entry.contains(&(String::from("Frutas"), String::from("Maçã!"))));

        let chavedefenda_entry = index.get("chavedefenda").unwrap();
        // Ajuste da verificação para considerar a normalização correta do item
        assert!(
            chavedefenda_entry
                .contains(&(String::from("Ferramentas"), String::from("Chave-De-Fenda")))
        );
    }
}
