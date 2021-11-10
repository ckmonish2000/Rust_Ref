pub fn matchs(){
  let number = 10;

  match number{
    1 => println!("its 1"),
    2 => println!("its 2"),
    // 10 | 11 => println!("its 10 or 11"),
    10...12 => println!("its b/w 10 to 12"),
    _ => println!("not under 15")
  }
}


