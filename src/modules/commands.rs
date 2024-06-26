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
}

const FILE_PATH: &str = "data.txt";

pub fn help() {
    
}

pub fn show() {
    match dataio::read_file(FILE_PATH) {
        Ok(data) => {
            if data != "" {
                print!("+--------------+");
                println!("{}", data);
                println!("+--------------+");
            }else {println!("Data is Empty")}
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}

pub fn add() {
    println!("Create a basic name without white space");
    let mut response: String = String::new();
    std::io::stdin().read_line(&mut response).expect("Error on Reading the Response");
    let data: &str = response.trim();

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

pub fn remove() {
    println!("What is the Data you want to Remove");
    let mut response: String = String::new();
    std::io::stdin().read_line(&mut response).expect("Error on Reading the Response");
    let data: &str = response.trim();



    let mut datas: Vec<String> = Vec::new();
    match dataio::read_file(FILE_PATH) {
        Ok(old_data) => {
            let dt: std::str::Split<char> = old_data.split('\n');
            for item in dt {
                if item == data {continue}
                datas.push(item.to_string());
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }

    match dataio::write_file(FILE_PATH, datas.join("\n").as_str()) {
        Ok(_) => {
            println!("Data Successfully Removed!");
        }
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }   
}