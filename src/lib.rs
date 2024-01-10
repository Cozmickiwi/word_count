use std::fs;

pub fn word_count(file: String) {
    let mut contents = Vec::new();
    let mut chars = Vec::new();
    let mut chars_no_space = Vec::new();
    let file2 = file.clone();
    let path = std::path::Path::new(&file2);
    let mut name = "";
    if let Some(fname) = path.file_name() {
        name = fname.to_str().unwrap();
    }
    match fs::read_to_string(file) {
        Ok(s) => {
            let s2 = s.trim();
            contents = s2.chars().filter(|x| *x == ' ' || *x == '\n').collect();
            chars_no_space = s.chars().filter(|a| *a != ' ' && *a != '\n').collect();
            chars = s.chars().collect();
        }
        Err(_) => println!("Cannot read file, please try again"),
    }
    let n = contents.len() + 1;
    //println!("{name} contains {n} words and {} chars ({} chars excluding whitespace).", chars.len(), chars_no_space.len());
    println!();
    println!("{name}");
    println!("------------------------------------------------------------------------");
    println!("{n} words");
    println!("{} chars", chars.len());
    println!("{} chars excluding whitespace", chars_no_space.len());
    println!("------------------------------------------------------------------------");
    println!();
}
