use chrono::Utc;
use rand::{thread_rng, Rng};
use crate::file_handler;


// print and save elapsed time every hour
// interval is the time between each save
pub fn get_time_hourly(timestamp: &i64, interval: i64) {
    if get_time_today(&timestamp) >= interval * 1000 {
        let elapsed = get_time_today(&timestamp);
        println!(
            "Elapsed time : {} seconds",
            convert_time(elapsed)
        );
        print_quote();
        let total = get_total_time(&timestamp);
        file_handler::save_to_file(total);
        let total_str: String = convert_time(total);
        println!("Total time (all sessions):{} {}", total_str, get_badge(&total));
    }
}

// Calculate time elapsed today (since program started)
pub fn get_total_time(time: &i64) -> i64 {
    let previous: i64 = file_handler::read_from_file().trim().parse().unwrap_or(0);
    get_current_time() - *time + previous
}

// Calculate time elapsed today (since program started)
pub fn get_time_today(time: &i64) -> i64 {
    get_current_time() - *time
}

pub fn get_current_time() -> i64 {
    Utc::now().timestamp_millis()
}

// Convert milliseconds to a human-readable format
pub fn convert_time(time: i64) -> String {
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
        let mut rng = thread_rng();

    println!("{}", quotes[rng.gen_range(0..quotes.len())]);
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
