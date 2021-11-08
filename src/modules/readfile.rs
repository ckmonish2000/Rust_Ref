use std::fs::*;
use std::env;
use std::io::prelude::*;

pub fn read_file(){
  
  let mut path = env::current_dir();
  let mut path = String::from(path.unwrap().to_str().unwrap());
  path.push_str("\\src\\modules\\info.txt");

  let mut file = File::open(path).expect("cant open file");
  let mut content = String::new();

  file.read_to_string(&mut content).unwrap();

  println!("{}", content);
}