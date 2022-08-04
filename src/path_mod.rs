#![allow(dead_code, unused_variables, unused_imports)]

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


pub trait SuperPathBuf {
    fn read_text(&self) -> String;
    fn write_text(&self, text: &str) -> &Self;
    fn write_text_(&mut self, text: &str) -> &mut Self;  // when a function mutates an object, its syntactically obvious at call made as mutable reference is passed, thus no need for (!) as in Julia. However, in methods, `self` is implicit, and its no longer obvious whether `self` or `&self` or `&mut self` is passed. Thus I came up with `_` to indicate mutable reference. If its a move, it shouldn't be hard to see that, since the whole object is taken out of scope.
    fn create(&self) -> &Self;
    fn create_(&mut self) -> &mut Self;
    fn home() -> Self; 
    fn cwd() -> Self; 
    fn tmp() -> Self; 
    fn tmp_file(ext:&str) -> Self; 
    fn tmp_dir() -> Self;
    fn launch(&self) -> Result<(), OpenError>;
    fn expanduser(&self) -> Self;
    // fn expanduser_(&mut self) -> &mut Self; // not easy to manipulate underlying OsString // see: https://stackoverflow.com/questions/73241918/how-to-mutate-the-underlying-string-of-a-pathbuf/73242253#73242253
    fn collapseuser(&self) -> Self;
}


impl SuperPathBuf for PathBuf {
    fn read_text(&self) -> String {let mut file = File::open(self).expect("Failed to open file"); let mut contents = String::new(); file.read_to_string(&mut contents).expect("Failed to read file"); return contents;}
    fn write_text(&self, text: &str) -> &Self {self.parent().unwrap().to_path_buf().create(); let mut file = File::create(&self).expect("Failed to create file"); file.write_all(text.as_bytes()).expect("Failed to write file"); return self}
    fn write_text_(&mut self, text: &str) -> &mut Self {self.parent().unwrap().to_path_buf().create(); let mut file = File::create(&self).expect("Failed to create file"); file.write_all(text.as_bytes()).expect("Failed to write file"); return self}
    fn create(&self) -> &Self {fs::create_dir_all(&self).expect("Failed to create directory"); return self}
    fn create_(&mut self) -> &mut Self {fs::create_dir_all(&self).expect("Failed to create directory"); return self}
    fn home() -> PathBuf {dirs::home_dir().unwrap()}
    fn cwd() -> PathBuf {env::current_dir().unwrap()}
    fn tmp() -> PathBuf {PathBuf::home().join("tmp_results")}
    fn tmp_file(ext: &str) -> PathBuf {PathBuf::tmp().join(format!("tmp_files/{}{}", rand_str(10), ext))}
    fn tmp_dir() -> PathBuf {PathBuf::tmp().join(format!("tmp_dirs/{}", rand_str(10)))}
    fn launch(&self) -> Result<(), OpenError> {opener::open(self)}
    fn expanduser(&self) -> PathBuf {return PathBuf::from(&self.to_str().unwrap().replace("~", PathBuf::home().to_str().unwrap()));}
    // fn expanduser_(&mut self) -> &mut Self {return PathBuf::from(&self.to_str().unwrap().replace("~", PathBuf::home().to_str().unwrap())).to_path_buf();}
    fn collapseuser(&self) -> Self {return Path::new(&self.to_str().unwrap().replace(PathBuf::home().to_str().unwrap(), "~")).to_path_buf();}
}


// pub trait SuperPath {
//     fn read_text(&self) -> String;
//     fn write_text(&self, text: &str) -> &Self;
//     fn write_text_(&mut self, text: &str) -> &mut Self;  // when a function mutates an object, its syntactically obvious at call made as mutable reference is passed, thus no need for (!) as in Julia. However, in methods, `self` is implicit, and its no longer obvious whether `self` or `&self` or `&mut self` is passed. Thus I came up with `_` to indicate mutable reference. If its a move, it shouldn't be hard to see that, since the whole object is taken out of scope.
//     fn create(&self) -> &Self;
//     fn create_(&mut self) -> &mut Self;
//     fn home() -> &Self; 
//     fn cwd() -> Self; 
//     fn tmp() -> Self; 
//     fn tmp_file(ext:&str) -> Self; 
//     fn tmp_dir() -> Self;
//     fn launch(&self) -> Result<(), OpenError>;
//     fn expanduser(&self) -> Self;
//     // fn expanduser_(&mut self) -> &mut Self; // not easy to manipulate underlying OsString // see: https://stackoverflow.com/questions/73241918/how-to-mutate-the-underlying-string-of-a-pathbuf/73242253#73242253
//     fn collapseuser(&self) -> Self;
// }

// impl SuperPath for Path {
//     fn read_text(&self) -> String {let mut file = File::open(self).expect("Failed to open file"); let mut contents = String::new(); file.read_to_string(&mut contents).expect("Failed to read file"); return contents;}
//     fn write_text(&self, text: &str) -> &Self {self.parent().unwrap().to_path_buf().create(); let mut file = File::create(&self).expect("Failed to create file"); file.write_all(text.as_bytes()).expect("Failed to write file"); return self}
//     fn create(&self) -> &Path {fs::create_dir_all(self).expect("Failed to create directory"); return &self}
//     fn home() -> Self {Path::new(dirs::home_dir().unwrap().to_str().unwrap())}
//     fn cwd() -> Self {Path::new(env::current_dir().unwrap().to_str())}
//     fn tmp() -> Self {PathBuf::home().join("tmp_results").as_path()}
//     fn tmp_file(ext: &str) -> PathBuf {PathBuf::tmp().join(format!("tmp_files/{}{}", rand_str(10), ext))}
//     fn tmp_dir() -> PathBuf {PathBuf::tmp().join(format!("tmp_dirs/{}", rand_str(10)))}
//     fn launch(&self) -> Result<(), OpenError> {opener::open(self)}
//     fn expanduser(&self) -> Self {return Path::new(&self.to_str().unwrap().replace("~", PathBuf::home().to_str().unwrap())).to_path_buf();}
//     fn collapseuser(&self) -> Self {return Path::new(&self.to_str().unwrap().replace(PathBuf::home().to_str().unwrap(), "~")).to_path_buf();}
// }

// Notes: 
// * if self is taken to the method (borrow), then method must return Self, otherwise the object is lost, also, you must do assignment, running the method alone will cause the returned object tob e lost as well.
// * if an immutable reference is consumed by method, you can only return an immutable reference.
// * if mutable reference is consumned by method, then, 1- no need to return it, no need for assignment. However, when declaring path, you must use let mut.

