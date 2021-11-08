use std::fs::File;
use std::env;
use std::io::prelude::*;

pub fn write(){
  let mut file = File::create("output.txt")
  .expect("could not create file");

  file.write_all(b"welcome home")
  .expect("wrong");
}