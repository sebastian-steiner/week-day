use std::io;

macro_rules! wprintln {
    ($fmt:expr) => (print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\r\n"), $($arg)*));
}

pub fn req_num(req: Option<&str>, def: Option<u32>) -> u32 {
    match (req, def) {
        (Some(req), Some(def)) => wprintln!("{} (or {})", req, def),
        (Some(req), _) => wprintln!("{}", req),
        _ => ()
    };
    read_num(def)
}

fn read_num(default: Option<u32>) -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    match default {
        Some(default) => input.trim().parse().unwrap_or(default),
        None => {
            match input.trim().parse::<u32>() {
                Ok(val) => val,
                Err(_)  => read_num(None)
            }
        }
    }
}

