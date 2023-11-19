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

        for character in text.chars() {
            if let Some(braille) = self.braille_map.get(&character) {
                encoded_text.push_str(braille);
            } else {
                // Si el carácter no tiene un mapeo lo dejamos sin cambios
                encoded_text.push(character);
            }
        }

        encoded_text
    }

    fn _print_characters(&self) {
        for (letter, braille) in &self.braille_map {
            println!("{}: {}", letter, braille);
        }
    }
}
