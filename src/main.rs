use std::time::{SystemTime};
use std::{thread, fs};
use num_format::{ToFormattedString, Locale};
use std::io::Write;

fn main() {
    println!("CPU Cores: {}", num_cpus::get());
    let now = SystemTime::now();
    let mut handles = vec![];

    let threads:i64 = num_cpus::get() as i64;
    let from:i64    = 1i64;
    let to:i64      = 1_000_000_000i64;
    let base:i64    = to/threads;

    for i in 1..=threads {
        let from:i64 = match i {
            1 => from,
            _ => if base > from { ((i-1)*base)+1 } else { ((i-1)*base)+from+1 }
        };
        let to:i64 = if base > from { base * i } else { (base*1)+from };
        let handle  = thread::spawn(move ||{
            run(from, to)
        });
        handles.push(handle)
    }

    for handle in  handles{
        handle.join().unwrap();
    }
    match now.elapsed() {
        Ok(elapsed) => {
            let millis = elapsed.as_millis();
            write_to_log(convert_time(millis).as_str())
        }
        Err(e)=> {
            println!("Error: {:?}", e);
        }
    }
}

fn run(from: i64, to: i64) {
    let filename = format!("{}-{}.log",from.to_formatted_string(&Locale::en), to.to_formatted_string(&Locale::en));
    create_file(&filename);
    for i in from..=to {
        let number = i.to_formatted_string(&Locale::en);
        println!("{}",&number);
        let factors = test(i);
        let log_data = format!("{}:{:?}", number, &factors);
        write_to_file(&log_data, &filename)
    }
}

fn test(n: i64)-> Vec<i64> {
    let mut factors: Vec<i64> = Vec::new();
    factors.push(n);
    let mut temp = n;
    loop {
        temp = if temp == 1 { temp } else if temp%2 == 0 { temp/2 } else { (temp*3)+1 };
        if temp ==1 {
            factors.push(temp);
            break;
        } else {
            factors.push(temp);
        }
    }

    factors
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
        format!("{}h:{}m:{}s:{}ms", hours, minutes, seconds, millis)
    }
}

fn create_file(filename: &str) {
    fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create_new(true)
        .open(filename)
        .unwrap();
}

fn write_to_file(data: &str, filename: &str) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();
    write!(file, "{}\n", data).unwrap();
}

fn write_to_log(data: &str) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("time.log")
        .unwrap();
    write!(file, "{}", data).unwrap();
}