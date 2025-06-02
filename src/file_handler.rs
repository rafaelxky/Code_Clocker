use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;

pub fn reset_file(){
    save_to_file(0);
}

pub fn save_to_file(content: i64) {
    let path = get_log_path();
    let mut file = File::create(path).expect("Unable to create log file");
    writeln!(file, "{}", content).expect("Unable to write to log file");
}

pub fn get_log_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Cannot find home directory");
    path.push(".tmr");
    fs::create_dir_all(&path).expect("Failed to create ~/.tmr directory");
    path.push("time_log.txt");
    path
}

pub fn read_from_file() -> String {
    let path = get_log_path();

    if !path.exists() {
        File::create(&path).expect("Unable to create log file");
        return String::from("0");
    }

    let file = File::open(&path).expect("Unable to open log file");
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content).expect("Failed to read log file");
    content.trim().to_string()
}