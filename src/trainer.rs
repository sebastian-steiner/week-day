use super::num_io;
use super::stat::Stat;
use super::date;
use rand::{distributions::Uniform, prelude::*};

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
        let min = std::cmp::max(0, num_io::req_num(Some("Enter lower range"), Some(0)));
        let max = std::cmp::min(99, num_io::req_num(Some("Enter upper range"), Some(99)));
        for i in min..=max {
            println!("{:#02}, {}", i, date::year_code(i));
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
        stats.print("data/centuries.csv");
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
        stats.print("data/months.csv");
    }

    pub fn practice_years(&self) {
        let mut stats = Stat::new();
        println!("Practice year codes:");
        let min = std::cmp::max(0, num_io::req_num(Some("Enter lower range"), Some(0)));
        let max = std::cmp::min(99, num_io::req_num(Some("Enter upper range"), Some(99)));
        println!("Quit after 50 tries or by entering number > 6");
        for _ in 0..50 {
            let year = Uniform::from(min..=max).sample(&mut rand::thread_rng());
            let req = format!("xx{}?", year);
            if !stats.try_again(Some(&req), date::year_code(year % 100), 6) {
                break;
            }
        }
        stats.print("data/years.csv");
    }

    pub fn practice_day_month(&self) {
        let mut stats = Stat::new();
        println!("Practice day-month combinations:");
        println!("Quit after 50 tries or by entering number > 6");
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let month = Uniform::from(1..=12).sample(&mut rng);
            let day = Uniform::from(1..=date::day_max_from_date(month, None)).sample(&mut rng);
            let req = format!("{:#02}.{:#02}.", day, month);
            if !stats.try_again(Some(&req), self.week_day(day, month, None), 6) {
                break;
            }
        }
        stats.print("data/day-month.csv");
    }

    pub fn practice_full(&self) {
        let mut stats = Stat::new();
        println!("Practice full dates:");
        println!("Quit after 50 tries or by entering number > 6");
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let year = Uniform::from(1600..=2400).sample(&mut rng);
            let month = Uniform::from(1..=12).sample(&mut rng);
            let day = Uniform::from(1..=date::day_max_from_date(month, Some(year))).sample(&mut rng);
            let req = format!("{:#02}.{:#02}.{:#04}", day, month, year);
            if !stats.try_again(Some(&req), self.week_day(day, month, Some(year)), 6) {
                break;
            }
        }
        stats.print("data/full.csv");
    }

    pub fn practice_day(&self) {
        let mut stats = Stat::new();
        println!("Practice days:");
        println!("Quit after 50 tries or by entering number > 6");
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let day = Uniform::from(1..=31).sample(&mut rng);
            let req = format!("{:#02}.", day);
            if !stats.try_again(Some(&req), day % 7, 6) {
                break;
            }
        }
        stats.print("data/day.csv");
    }

    fn week_day(&self, day: u32, month: u32, year: Option<u32>) -> u32 {
        let month_code = self.months[(month - 1) as usize] as u32;
        let year_code = match year {
            Some(year) => {
                let code = date::year_code(year % 100);
                if date::is_leap_year(year) && month < 3 {
                    code - 1
                } else {
                    code
                }
            },
            None => 0
        };
        let century_code = match year {
            Some(year) => self.centuries[((year / 100) % 4) as usize] as u32,
            None => 0
        };
        (day + month_code + year_code + century_code) % 7
    }
}

