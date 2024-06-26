use std::fs::File;
use std::io::{self, Read, Write};

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() {
    const FILE_PATH: &str = "output.txt";
    let mut response: String = String::new();

    std::io::stdin().read_line(&mut response).expect("Error");

    match write_file(FILE_PATH, &response) {
        Ok(_) => println!("Successfully wrote to {}", FILE_PATH),
        Err(err) => eprintln!("Error writing to {}: {}", FILE_PATH, err),
    }

    print!("Hello World");
}

