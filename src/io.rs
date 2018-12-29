use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

const INPUT_DIR: &str = "inputs/";

pub fn load_input(name: &str) -> Vec<String> {
    let path = format!("{}{}.txt", INPUT_DIR, name);
    debug!("Opening path: {}", path);

    let file = File::open(path).unwrap();
    let mut file = BufReader::new(&file);
    let mut result = Vec::new();

    loop {
        let mut line = String::new();
        file.read_line(&mut line).unwrap();

        if line.is_empty() {
            break
        } else {
            result.push(line.trim().to_owned());
        }
    }

    result
}