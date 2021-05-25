use structopt::StructOpt;
use std::{thread, time};
use notify_rust::Notification;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "25")]
    work_time: u8,
    
    #[structopt(short, long, default_value = "5")]
    break_time: u8,

    #[structopt(short = "B", long = "longbreak", default_value = "15")]
    long_break: u8, 

}
fn main() {
    let args = Cli::from_args(); 
    let counter = 0;
    loop {
        let counter = counter + 1;
        thread::sleep(time::Duration::from_secs((args.work_time as u64)*60));
        timeup(true);
        if counter == 4 {
            thread::sleep(time::Duration::from_secs((args.long_break as u64)*60));
        }else{
            thread::sleep(time::Duration::from_secs((args.break_time as u64)*60));
        }
        timeup(false)
    }
}

fn timeup(breaktime: bool){
    if breaktime {
        Notification::new()
            .summary("Pomodoro Break")
            .body("Take a Break")
            .show().unwrap();
        play::play("audio/Funny-melody-sound-effect.mp3").unwrap();
    }else {
        Notification::new()
            .summary("Pomodoro Work Time")
            .body("Get back to work")
            .show().unwrap();
        play::play("audio/Bird-tweeting-sound-effect.mp3").unwrap();
    }
}
