//#![windows_subsystem = "windows"]
mod event;

use clokwerk::{Scheduler, TimeUnits};
use clokwerk::Interval::*;
use tokio::time::{self, Duration};
//use std::thread;

#[tokio::main]
async fn main() {
    // Create a new scheduler
    //let mut scheduler = Scheduler::new();
    // Add some tasks to it
    //event::event_main();
    //scheduler.every(1.minutes())
    //    .run(|| event::event_main());

    let mut interval = time::interval(Duration::from_secs(60));

    loop {
        event::event_main();

        interval.tick().await;

        //scheduler.run_pending();
        //thread::sleep(Duration::from_millis(10));
    }
}