use std::sync::atomic::{AtomicBool, Ordering};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::sync::Arc;
use std::{thread, time::Duration};
use chrono::Utc;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if  args.len() > 1 && args[1] == "reset" {
        save_to_file(0);

        print!("Count has ben reset");
        return Ok(());
    }

    let timestamp: i64 = get_current_time();

    // Create cleanup struct which runs code when program ends
    let _cleanup = Cleanup {
        start_time: timestamp,
    };

    // Shared flag for running status, safe across threads
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Setup Ctrl+C handler to stop the program cleanly
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");

    println!("Started counting... Press Ctrl+C to stop.");

    // Loop until Ctrl+C is pressed
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100)); // avoid busy waiting
        // Print and save elapsed time hourly
        get_time_hourly(&timestamp, 1 * 60 * 60);
    }

    Ok(())
}

// print and save elapsed time every hour
// interval is the time between each save
fn get_time_hourly(timestamp: &i64, interval: i64) {
    if get_time_today(&timestamp) >= interval * 1000 {
        let elapsed = get_time_today(&timestamp);
        println!(
            "Elapsed time : {} seconds",
            convert_time(elapsed)
        );
        print_quote();
        let total = get_total_time(&timestamp);
        save_to_file(total);
        let total_str: String = convert_time(total);
        println!("Total time (all sessions):{} {}", total_str, get_badge(&total));
    }
}

fn print_quote(){
    let quotes = vec![
        "Remember to hidrate",
        "Remember to take a break",
        "Watch your posture, you will regret it later",
        "Take a deep breat",
        "Don't sit for too long. Stand up and take a walk",
        "Go touch grass",
        "Enjoy the next 24 hours",
        "Hi :)",
        "There are tinny people inside you computer"
        ];
        let mut rng: ThreadRng = rand::thread_rng();

    println!("{}", quotes[rng.gen_range(0..quotes.len())]);
}

// Calculate total time = previous time + elapsed since start
fn get_total_time(time: &i64) -> i64 {
    let previous: i64 = read_from_file().trim().parse().unwrap_or(0);
    get_current_time() - *time + previous
}

// Calculate time elapsed today (since program started)
fn get_time_today(time: &i64) -> i64 {
    get_current_time() - *time
}

// Get full path to ~/.tmr/time_log.txt, ensure directory exists
fn get_log_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Cannot find home directory");
    path.push(".tmr");
    fs::create_dir_all(&path).expect("Failed to create ~/.tmr directory");
    path.push("time_log.txt");
    path
}

// Read time log from file, create if missing
fn read_from_file() -> String {
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

// Save total time back to file
fn save_to_file(content: i64) {
    let path = get_log_path();
    let mut file = File::create(path).expect("Unable to create log file");
    writeln!(file, "{}", content).expect("Unable to write to log file");
}

fn get_current_time() -> i64 {
    Utc::now().timestamp_millis()
}

// Stop logging and print the total time
fn stop_logging(start_time: &i64) {
    println!(
        "Stopped the count, time programmed today: {} seconds",
        convert_time(get_time_today(start_time))
    );
    let total = get_total_time(start_time);
    save_to_file(total);
    let totalStr: String = convert_time(total);
    println!("Total time (all sessions):{} {}", totalStr, get_badge(&total));
}


// Convert milliseconds to a human-readable format
fn convert_time(time: i64) -> String {
    let mut ms = time;
    let day_ms = 1000 * 60 * 60 * 24;
    let hour_ms = 1000 * 60 * 60;
    let min_ms = 1000 * 60;
    let sec_ms = 1000;

    let days = ms / day_ms;
    ms %= day_ms;
    let hours = ms / hour_ms;
    ms %= hour_ms;
    let minutes = ms / min_ms;
    ms %= min_ms;
    let seconds = ms / sec_ms;

    let mut parts = Vec::new();
    if days > 0 { parts.push(format!("{} days", days)); }
    if hours > 0 { parts.push(format!("{} hours", hours)); }
    if minutes > 0 { parts.push(format!("{} minutes", minutes)); }
    parts.push(format!("{} seconds", seconds));

    parts.join(" ")
}

fn get_badge(time: &i64) -> String {
    // if less than 3 months return white circle
    if *time < 3 * 30 * 24 * 60 * 60 * 1000 {
        return String::from("⚪");
    }
    // if less than 6 months return green circle
    if *time < 6 * 30 * 24 * 60 * 60 * 1000 {
        return String::from("🟢");
    }
    // if less than 1 year return blue circle
    if *time < 12 * 30 * 24 * 60 * 60 * 1000 {
        return String::from("🔵");
    }
    // if less than 1.5 years return orange circle
    if *time < 18 * 30 * 24 * 60 * 60 * 1000 {
        return String::from("🟠");
    }
    // if less than 2 years return red circle
    if *time < 24 * 30 * 24 * 60 * 60 * 1000 {
        return String::from("🔴");
    }
    // if less than 3 years return yellow circle
    if *time < 36 * 30 * 24 * 60 * 60 * 1000 {
        return String::from("🟡");
    }
    // if more than 3 years return star circle
    String::from("⭐")

}

// Cleanup struct runs drop code on program exit
struct Cleanup {
    start_time: i64,
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        stop_logging(&self.start_time);
    }
}
// cargo build --release
// sudo cp ./target/release/code_clocker /usr/local/bin/tmr

