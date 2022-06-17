use std::fs::File;
use std::io::{ErrorKind, Error, Read};

pub fn run(){
    let name = "Atri";

    //closures
    let write_message = |slogan: String| -> String {
      String::from(format!("This is name: {}", name))
    };

    let phrase = write_message(String::from("slogan"));
    println!("{}", phrase);

    let filename = "/home/mizuuu/Documents/temp.txt";
    let file_data = read_file(filename);
    match file_data {
        Ok(data) => {
            println!("{}", data);
        }
        Err(_) => {}
    }

}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut file_handle = File::open(filename)?;
    let mut file_data = String::new();
    file_handle.read_to_string(&mut file_data)?;
    Ok(file_data)
}


fn file_errors(){
    let filename = "/home/mizuuu/Documents/temp.txt";
    match File::open(filename){
        Ok(file) => {
            println!("{:#?}", file);
        }
        Err(error) => {
            match error.kind(){
                ErrorKind::NotFound =>{
                    match File::create(filename){
                        Ok(file) => {
                            println!("File created!");
                        }
                        Err(error) => {
                            println!("{:#?}", error);
                        }
                    }
                }
                _ => {
                    println!("{:#?}", error);
                }
            }
        }
    }

}