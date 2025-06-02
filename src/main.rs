use std::sync::atomic::{AtomicBool, Ordering};
use user_input::get_user_input;
use std::sync::Arc;
use std::{thread, time::Duration};
use std::env;
mod user_input;
mod file_handler;
mod time_handler;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if check_args(args) {return Ok(())} 

    let timestamp: i64 = time_handler::get_current_time();

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

    let mut last_save = time_handler::get_current_time();
    let interval = 60 * 60; // 1 hour in seconds
    let interval_ms = interval * 1000;

    // Loop until Ctrl+C is pressed
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
        let now = time_handler::get_current_time();
        if now - last_save >= interval_ms {
            time_handler::get_time_hourly(&timestamp, interval);
            last_save = now;
        }
    }

    Ok(())
}

fn check_args(args: Vec<String>) -> bool{
     if  args.len() > 1 && args[1] == "reset" {
        confirm_reset_file();
        return true;
    }
    return false;
}

fn confirm_reset_file(){
    println!("Are you sure you want to reset your progress? [Y] [N]");
    if get_user_input().to_lowercase().trim() == "y" {
        println!("Count has ben reset");
        file_handler::reset_file();
    } else {
        println!("Cancelled action");
    }
}

// Stop logging and print the total time
fn stop_logging(start_time: &i64) {
    println!(
        "Stopped the count, time programmed today: {} seconds",
        time_handler::convert_time(time_handler::get_time_today(start_time))
    );
    let total = time_handler::get_total_time(start_time);
    file_handler::save_to_file(total);
    let total_str: String = time_handler::convert_time(total);
    println!("Total time (all sessions):{} {}", total_str, get_badge(&total));
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

