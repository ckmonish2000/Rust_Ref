use std::env;
pub fn evn_var(){
  let vect: Vec<String> = env::args().collect();

  println!("{:?}",vect);
}