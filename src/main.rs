
use croco::*;
use glob::glob;


fn main() {
    // why this doesnt work?
    let path1 = "src/main.rs";
    
    // path1.delete

    // let res = path1.into_os_string().replace("~", "alex");
    println!("{:?}", path1);
    println!("Finished");
}
