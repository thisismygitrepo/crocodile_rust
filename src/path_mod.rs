
pub use std::path::{Path, PathBuf};
// use std::fs::{canonicalize, copy};
// use dirs::*; 
// extern crate dirs;

use std::fs::File;
use std::fs;
use std::io::{Read, Write};
use std::env;

use opener::OpenError;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
pub fn rand_str(len:usize) -> String{return thread_rng().sample_iter(&Alphanumeric).take(len).map(char::from).collect();}  // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html


pub trait SuperPath {
    fn read_text(&self) -> String; fn write_text(self, text: &str) -> Self;
    fn home() -> PathBuf; fn cwd() -> PathBuf; fn tmp() -> PathBuf; 
    fn tmp_file(ext:&str) -> PathBuf; fn tmp_dir() -> PathBuf;
    fn create(&self) -> &Path;
    fn launch(&self) -> Result<(), OpenError>;
    fn expanduser(&self) -> Self;
    fn collapseuser(&self) -> Self;
}


impl SuperPath for PathBuf {
    fn read_text(&self) -> String {let mut file = File::open(self).expect("Failed to open file"); let mut contents = String::new(); file.read_to_string(&mut contents).expect("Failed to read file"); return contents;}
    fn write_text(self, text: &str) -> Self {self.parent().unwrap().to_path_buf().create(); let mut file = File::create(&self).expect("Failed to create file"); file.write_all(text.as_bytes()).expect("Failed to write file"); return self}
    fn create(&self) -> &Path {fs::create_dir_all(self).expect("Failed to create directory"); return &self}
    fn home() -> PathBuf {dirs::home_dir().unwrap()}
    fn cwd() -> PathBuf {env::current_dir().unwrap()}
    fn tmp() -> PathBuf {PathBuf::home().join("tmp_results")}
    fn tmp_file(ext: &str) -> PathBuf {PathBuf::tmp().join(format!("tmp_files/{}{}", rand_str(10), ext))}
    fn tmp_dir() -> PathBuf {PathBuf::tmp().join(format!("tmp_dirs/{}", rand_str(10)))}
    fn launch(&self) -> Result<(), OpenError> {opener::open(self)}
    fn expanduser(&self) -> Self {return Path::new(&self.to_str().unwrap().replace("~", PathBuf::home().to_str().unwrap())).to_path_buf();}
    fn collapseuser(&self) -> Self {return Path::new(&self.to_str().unwrap().replace(PathBuf::home().to_str().unwrap(), "~")).to_path_buf();}
}

