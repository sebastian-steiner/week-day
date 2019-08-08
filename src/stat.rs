use std::time::{Duration, Instant};
use std::fs::OpenOptions;
use std::io::prelude::*;
use super::num_io;

pub struct Stat {
    times: Vec<Duration>,
    curr_time: Instant,
    tries: u32,
    incorrect: u32
}

impl Stat {
    pub fn new() -> Stat {
        Stat {
            times: Vec::new(),
            curr_time: Instant::now(),
            tries: 0,
            incorrect: 0
        }
    }

    pub fn try_again(&mut self, req: Option<&str>, corr: u32, end: u32) -> bool {
        self.curr_time = Instant::now();
        let ans = num_io::req_num(req, None);
        if ans == corr {
            self.end_try(true);
            println!("✓");
            true
        } else if ans > end {
            // user wants to quit
            false
        } else {
            self.end_try(false);
            println!("❌: {}", corr);
            true
        }
    }

    fn end_try(&mut self, correct: bool) {
        self.tries += 1;
        if !correct {
            self.incorrect += 1;
        } else {
            self.times.push(self.curr_time.elapsed());
        }
    }

    pub fn print(&self, file: &str) {
        println!("{}", self);
        self.to_file(file)
    }

    fn mean_times(&self) -> f64 {
        let times: Vec<f64> = self.times.iter().cloned().map(|x| x.as_millis() as f64 / 1000.0).collect();
	mean(times.iter()).expect("Could not calculate mean of time")
    }

    pub fn to_file(&self, file: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file)
            .unwrap();
        let tries_rat = self.incorrect as f64 / self.tries as f64;
        
        if let Err(e) = writeln!(file, "{:.3}; {:.3}\r", self.mean_times(), tries_rat) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

}


impl std::fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let times: Vec<f64> = self.times.iter().cloned().map(|x| x.as_millis() as f64 / 1000.0).collect();
        let mean_times = self.mean_times();
	let mut result = format!(
            "Time\r\n\tμ: {}s\r\n\tmax: {}s\r\n\tmin: {}s\r\n",
            mean_times,
            times.iter().cloned().fold(0. / 0., f64::max),
            times.iter().cloned().fold(1. / 0., f64::min)
        );
        result += &format!("Tries\r\n\ttotal: {}\r\n\tcorrect: {}\r\n\tincorrect: {}\n",
                self.tries,
		self.tries - self.incorrect,
		self.incorrect);



        write!(f, "{}", result)
    }
}

fn mean<'a, T, I>(iter: I) -> Option<f64>
where
    T: Into<f64> + std::iter::Sum<&'a T> + 'a,
    I: Iterator<Item = &'a T> {
    let mut len = 0;
    let sum = iter
        .map(|t| {
            len += 1;
            t
        })
        .sum::<T>();

    match len {
        0 => None,
        _ => Some(sum.into() / len as f64),
    }
}
