
use croco::*;


fn main() {
    let path1 = PathBuf::tmp_dir().collapseuser().expanduser();
    println!("{:?}", path1);
    println!("Finished");
}
