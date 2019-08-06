pub fn is_leap_year(y: i32) -> bool {
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
