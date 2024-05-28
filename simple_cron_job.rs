extern crate cron;

use cron::Schedule;
use std::thread;
use std::time::Duration;

fn main() {
    let schedule = "*/15 * * * * *".parse::<Schedule>().unwrap();

    loop {
        let next = schedule.upcoming(Local::now()).next().unwrap();
        let now = Local::now();
        if next > now {
            thread::sleep(next - now);
            println!("Running cron job...");
            println!("Current time: {:?}", Local::now());
        } else {
            thread::sleep(Duration::from_secs(1));
        }
    }
}
