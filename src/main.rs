use std::{
    fs::File,
    io::{self, BufRead},
};

mod braille_language;
use braille_language::braille::Braille;

fn main() {
    // Lee el archivo de texto
    let file = File::open("inputs/input.txt").expect("No se pudo abrir el archivo");
    let reader = io::BufReader::new(file);

    let mut braille = Braille::new();
    braille.initialize();

    for line in reader.lines() {
        let text_line = line.expect("Lecutra de línea");
        let reuslt = braille.encode(text_line.clone());

        if text_line.len() != 0 {
            // Imprime la línea Braille y el texto
            println!("{} -> {}", reuslt, text_line);
        } else {
            println!("");
        }
    }
}
