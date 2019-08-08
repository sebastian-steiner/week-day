extern crate chrono;
extern crate rand;

use trainer::Trainer;
use std::io;

mod trainer;
mod num_io;
mod date;
mod stat;

fn main() {
    let trainer = Trainer::new();
    println!("!!!Week-day trainer!!!");
    let mut input;
    loop {
        println!("Do you want to see codes for: centuries (cc), years (yc) or months (mc)?");
        println!("Or practice: centuries: (cp), years (yp), months (mp), day-month (dmp) or full (default)?");
        input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().as_ref() {
            "cc"    => trainer.learn_centuries(),
            "yc"    => trainer.learn_years(),
            "mc"    => trainer.learn_months(),
            "cp"    => trainer.practice_centuries(),
            "yp"    => trainer.practice_years(),
            "mp"    => trainer.practice_months(),
            "dmp"   => trainer.practice_day_month(),
            _       => trainer.practice_full()
        }
    }
}

