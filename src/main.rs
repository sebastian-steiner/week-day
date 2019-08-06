extern crate chrono;
extern crate rand;

use rand::prelude::*;
use stat::Stat;

#[macro_use]
mod io;
mod date;
mod stat;

fn main() {
    let trainer = Trainer::new();
    trainer.learn_months();
    trainer.learn_centuries();
    trainer.practice_months();
}

struct Trainer {
    months: [u8; 12],
    month_names: [&'static str; 12],
    centuries: [u8; 4],
}

impl Trainer {
    fn new() -> Trainer {
        Trainer {
            months: [4, 0, 0, 3, 5, 1, 3, 6, 2, 4, 0, 2],
            month_names: [
                "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
            ],
            centuries: [0, 5, 3, 1],
        }
    }

    fn practice_months(&self) {
        let mut stats = Stat::new();
        wprintln!("Practice month codes:");
        wprintln!("Quit by entering number > 6");
        loop {
            let idx = rand::thread_rng().gen_range(0, 12);
            let req = format!("{}?", self.month_names[idx]);
            stats.start_try();
            let ans = io::req_num(Some(&req), None);
            if ans == self.months[idx] as u32 {
                stats.end_try(true);
                wprintln!("✓");
            } else if ans > 6 {
                // user wants to quit
                break;
            } else {
                stats.end_try(false);
                wprintln!("❌: {}", self.months[idx]);
            }
        }
        wprintln!("{}", stats);
        stats.to_file("data/months.csv")
    }

    fn learn_months(&self) {
        wprintln!("Month codes:");
        for (c, n) in self.months.iter().zip(self.month_names.iter()) {
            wprintln!("{}: {}", n, c);
        }
    }

    fn learn_centuries(&self) {
        wprintln!("Century codes:");
        let mut base_year = 1600;
        for _ in 0..2 {
            for (i, c) in self.centuries.iter().enumerate() {
                wprintln!("{}: {}", i * 100 + base_year, c);
            }
            base_year += 400;
        }
    }
}
