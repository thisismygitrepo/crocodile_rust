
use croco::*;
use std::ffi::OsString;
use std::env;

fn main() {

    let mut path1 = PathBuf::from("~/file");
    let y = Path::new("new");

    // let res = path1.into_os_string().replace("~", "alex");
    println!("{:?}", path1.canonicalize());
    println!("Finished");


}
