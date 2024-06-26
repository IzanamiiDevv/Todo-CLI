mod dataio {
    use std::fs::File;
    use std::io::{self, Read, Write};
    pub fn read_file(file_path: &str) -> io::Result<String> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
    
    pub fn write_file(file_path: &str, content: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn addData(data: &str) {
        
    }

    fn deleteData(data: &str) {

    }
}

const FILE_PATH: &str = "data.txt";

pub fn help() {
    
}

pub fn show() {
    match dataio::read_file(FILE_PATH) {
        Ok(data) => {
            println!("+--------------+");
            println!("{}", data);
            println!("+--------------+");
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}

pub fn add(data: &str) {
    let mut datas: Vec<String> = Vec::new();
    match dataio::read_file(FILE_PATH) {
        Ok(old_data) => {
            let dt: std::str::Split<char> = old_data.split('\n');
            for item in dt {
                datas.push(item.to_string());
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }

    datas.push(data.to_string());

    match dataio::write_file(FILE_PATH, datas.join("\n").as_str()) {
        Ok(_) => {
            println!("Data Successfully Added!");
        }
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }
}

pub fn done() {

}