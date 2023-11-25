use std::{collections::HashMap, fs::File, io::Read};

pub struct Braille {
    braille_map: HashMap<char, String>,
}

impl Braille {
    pub fn new() -> Self {
        Self {
            braille_map: Default::default(),
        }
    }

    pub fn initialize(&mut self) {
        let mut file = File::open("src/braille_language/characters.json")
            .expect("No se pudo abrir el archivo");

        // Leer el contenido del archivo en una cadena
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)
            .expect("Error al leer el archivo");

        // Deserializamos el JSON a un HashMap
        self.braille_map = serde_json::from_str(&json_data).expect("Error al deserializar JSON");
    }

    fn _add_mapping(&mut self, character: char, braille_representation: &str) {
        self.braille_map
            .insert(character, braille_representation.to_string());
    }

    pub fn encode(&self, text: String) -> String {
        let mut encoded_text = String::new();

        // Dividir la cadena en palabras
        let words: Vec<&str> = text.split_whitespace().collect();

        // Obtener la cantidad total de palabras
        let total_words = words.len();

        // Iterar sobre las palabras y determinar si es la última
        let mut word_index = 0;

        let mut last_char = ' ';

        while let Some(word) = words.get(word_index) {
            let mut copy_word = word.to_string();
            let mut char_index = 0;

            if last_char.is_lowercase() && self.is_in_uppercase(word.to_string()) {
                encoded_text.push('⠔');
                last_char = ' ';
                copy_word = copy_word.to_lowercase();
            }

            while let Some(character) = copy_word.chars().nth(char_index) {
                let next_char = word.chars().nth(char_index + 1).unwrap_or_default();

                if character == '.' && next_char.is_uppercase() {
                    // println!("Character: {} -> Next: {}", character, next_char);
                    encoded_text.push_str(&self.find_character(next_char));
                    char_index += 1;
                } else {
                    encoded_text.push_str(&self.find_character(character));
                }
                last_char = character;
                char_index += 1;
            }

            if word_index != total_words - 1 {
                encoded_text.push(' ');
            }

            word_index += 1;
        }

        encoded_text
    }

    fn _print_characters(&self) {
        for (letter, braille) in &self.braille_map {
            println!("{}: {}", letter, braille);
        }
    }

    fn find_character(&self, character: char) -> String {
        if let Some(braille) = self.braille_map.get(&character) {
            return braille.to_string();
        }
        // Si el carácter no tiene un mapeo lo dejamos sin cambios
        return character.to_string();
    }

    fn is_in_uppercase(&self, word: String) -> bool {
        word.chars().all(|char| char.is_uppercase())
    }
}
