use super::stat::Stat;
use rand::prelude::*;

pub struct Trainer {
    months: [u8; 12],
    month_names: [&'static str; 12],
    centuries: [u8; 4],
}

impl Trainer {
    pub fn new() -> Trainer {
        Trainer {
            months: [4, 0, 0, 3, 5, 1, 3, 6, 2, 4, 0, 2],
            month_names: [
                "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
            ],
            centuries: [0, 5, 3, 1],
        }
    }

    pub fn learn_months(&self) {
        println!("Month codes:");
        for (c, n) in self.months.iter().zip(self.month_names.iter()) {
            println!("{}: {}", n, c);
        }
    }

    pub fn learn_centuries(&self) {
        println!("Century codes:");
        let mut base_year = 1600;
        for _ in 0..2 {
            for (i, c) in self.centuries.iter().enumerate() {
                println!("{}: {}", i * 100 + base_year, c);
            }
            base_year += 400;
        }
    }

    pub fn learn_years(&self) {
        println!("Year codes:");
        println!("year, code");
        for i in 0..100 {
            println!("{:#02}, {}", i, year_code(i));
        }
    }


    pub fn practice_centuries(&self) {
        let mut stats = Stat::new();
        println!("Practice century codes:");
        println!("Quit after 50 tries or by entering number > 6");
        for _ in 0..50 {
            let idx = rand::thread_rng().gen_range(0, 4);
            let req = format!("{}s?", (idx + 16) * 100);
            if !stats.try_again(Some(&req), self.centuries[idx] as u32, 6) {
                break;
            }
        }
        stats.print();
    }
    
    pub fn practice_months(&self) {
        let mut stats = Stat::new();
        println!("Practice month codes:");
        println!("Quit after 50 tries or by entering number > 6");
        for _ in 0..50 {
            let idx = rand::thread_rng().gen_range(0, 12);
            let req = format!("{}?", self.month_names[idx]);
            if !stats.try_again(Some(&req), self.months[idx] as u32, 6) {
                break;
            }
        }
        stats.print();
    }

    pub fn practice_years(&self) {
        let mut stats = Stat::new();
        println!("Practice year codes:");
        println!("Quit after 50 tries or by entering number > 6");
        for _ in 0..50 {
            let year = rand::thread_rng().gen_range(0, 100);
            let req = format!("xx{}?", year);
            if !stats.try_again(Some(&req), year_code(year), 6) {
                break;
            }
        }
        stats.print();
    }

    pub fn practice_full(&self) {
    
    }

    pub fn practice_day_month(&self) {
    
    }
}


fn year_code(year: u32) -> u32 {
    (year + year / 4 + 2) % 7
}
