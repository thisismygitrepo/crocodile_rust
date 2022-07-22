
//! # lib.rs
//! `croco.core` provides convenience functions for high level coding. 
//! 
//! 

mod path_mod;
pub use path_mod::*;


/// Get input from commandline, specify type of input and optionally provide default value at empty entry. Repeat until success.
/// # Examples
/// ```rust
/// let input = get_input::<i32>(msg: "Enter a number", default: Some(0));
/// ```
#[inline]
pub fn input<T>(msg: &str, default: Option<T>) -> T where T: core::fmt::Debug + std::str::FromStr { 
    return loop {
        println!("{} [expected type = {:#?}, default = {:?}] ", msg, std::any::type_name::<T>(), default);
        let mut raw_input = String::new();
        std::io::stdin().read_line(&mut raw_input).expect("Failed to read line");
        raw_input = raw_input.trim().to_string();
        if raw_input == "" && default.is_some() {break default.unwrap();};  // return default value if empty input.
        let parsed_input: Result<T, _> = raw_input.parse::<T>();
        match parsed_input { Ok(num) => {break num;},  // return parsed input if success.
                             Err(_err) => {println!("could not parse the input `{:#?}`", raw_input); continue;},};
    }}

