
use std::fs::File;
use std::io::{ErrorKind, Read};
use croco::input;


fn read_text(path:String) -> String {
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    return s;
}


fn main() {
    println!("Hello, world!");
}
