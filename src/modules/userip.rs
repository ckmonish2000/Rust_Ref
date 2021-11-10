use std::io::stdin;

pub fn userip(){
  let mut line = String::new();
  let data = stdin().read_line(&mut line).unwrap();
  println!("{}",line);
}