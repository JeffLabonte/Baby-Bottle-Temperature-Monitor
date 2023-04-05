use chrono;
use std::fs::File;
use std::io::prelude::*;

pub fn write_to_file(file_path: String, message: String) {
    let mut file: File = match File::options().append(true).open(&file_path) {
        Ok(f) => f,
        Err(_) => File::create(&file_path).unwrap(),
    };
    let result = file.write_all(message.as_bytes());
    match result {
        Ok(_) => (),
        Err(err) => {
            panic!("Unable to write file: {}\n\n{}", &file_path, err);
        }
    }
}

pub fn generate_file_name_with_now_time(extension: String) -> String {
    let local_time = chrono::offset::Local::now();
    format!("{}{}", local_time.format("%Y-%m-%dT%H:XX"), extension)
}
