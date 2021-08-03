use std::time::{Duration, SystemTime};
use std::{thread, fs};
use std::string;
use num_format::{ToFormattedString, Locale};
use std::io::Write;
use std::fmt::format;
use std::thread::JoinHandle;

fn main() {
    println!("CPU Cores: {}", num_cpus::get());
    let now = SystemTime::now();
    let mut handles = vec![];

    for i in 1..=8 {
        let base: u64               = 130_000_000;
        let to:u64                  = base * i;
        let from:u64                = if i==1 { 1 } else { ((i-1)*base)+1 };
        let handle: JoinHandle<()>  = thread::spawn(move || {
            run(from, to);
        });
        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap();
    }
    match now.elapsed() {
        Ok(elapsed) => {
            let millis = elapsed.as_millis();
            write_to_file(convert_time(millis).as_str())
        }
        Err(e)=> {
            println!("Error: {:?}", e);
        }
    }
}

fn run(from: u64, to: u64) {
    let thread_start = SystemTime::now();
    for i in from..=to {
        println!("{}",i.to_formatted_string(&Locale::en));
        test(i);
    }
    match thread_start.elapsed() {
        Ok(elapsed) => {
            let millis = elapsed.as_millis();
            let log_entry = format!("{}-{} Duration: {}\n",
                                    from.to_formatted_string(&Locale::en),
                                    to.to_formatted_string(&Locale::en),
                                    convert_time(millis));
            write_to_file(log_entry.as_str())
        }
        Err(e)=> {
            println!("Error: {:?}", e);
        }
    }
}

fn test(n: u64)-> u64 {
    let mut temp = n;
    loop {
        if temp == 1 {
            break temp
        } else {
            temp = collatz_func(temp)
        }
    }
}

fn collatz_func(number: u64) -> u64 {
    if number%2 == 0 {
        number/2
    } else {
        (number*3)+1
    }
}

fn convert_time(duration: u128) -> String {
    let hours: u128     = duration/3_600_000;
    let minutes: u128   = (duration % 3_600_000)/60_000;
    let seconds: u128   = (duration%3_600_000)%60_000/1_000;
    let millis: u128    = ((duration%3_600_000)%60_000)%1000;

    if hours == 0 && minutes == 0 && seconds == 0 {
        format!("{}ms", millis)
    } else if hours == 0 && minutes == 0{
        format!("{}s:{}ms", seconds, millis)
    }else if hours == 0 {
        format!("{}m:{}s:{}ms", minutes, seconds, millis)
    } else {
        format!("{}h:${}m:{}s:{}ms", hours, minutes, seconds, millis)
    }
}

fn write_to_file(data: &str) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("time.log")
        .unwrap();
    write!(file, "{}", data);
}