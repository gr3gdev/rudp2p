#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        println!("\x1b[32m[INFO]\x1b[0m {}", res);
    }}
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        println!("\x1b[33m[ERROR]\x1b[0m {}", res);
        panic!("{}", res);
    }}
}
