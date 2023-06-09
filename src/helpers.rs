use chrono;
use std::fs::File;
use std::io::prelude::*;

#[cfg(not(debug_assertions))]
static LOG_PATH: &str = "/var/log/baby_bottle/";

#[cfg(debug_assertions)]
static LOG_PATH: &str = "logs/";

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
    format!("{}{}{}",LOG_PATH, local_time.format("%Y-%m-%d"), extension)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_file_name_with_now_time() {
        let file_name = generate_file_name_with_now_time(".log".to_string());
        let current_time = chrono::offset::Local::now();
        assert_eq!(file_name.contains(".log"), true);
        assert_eq!(
            file_name.contains(&current_time.format("%Y-%m-%d").to_string()),
            true
        );
    }
}
