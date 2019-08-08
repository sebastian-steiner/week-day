pub fn is_leap_year(y: u32) -> bool {
    if y % 400 == 0 {
        true
    } else if y % 100 == 0 {
        false
    } else if y % 4 == 0 {
        true
    } else {
        false
    }
}

pub fn day_max_from_date(month: u32, year: Option<u32>) -> u32 {
    match month {
        2 => {
            match year {
                Some(year) => {
                    if is_leap_year(year) {
                        29
                    } else {
                        28
                    }
                },
                None => 29
            }
        },
        4  => 30,
        6  => 30,
        9  => 30,
        11 => 30,
        _ => 31
    }
}

pub fn year_code(year: u32) -> u32 {
    (year + year / 4 + 2) % 7
}
