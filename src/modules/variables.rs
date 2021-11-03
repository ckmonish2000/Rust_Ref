pub fn vars(){
  let x = 45;
  let mut y = 60;
  
  /* refer more types from 
  (https://doc.rust-lang.org/book/ch03-02-data-types.html) */
  
  let x2: i32 = -20;
  let f: f64 = 30.00;
  let b: bool = false;
  
  println!("var = {} mutable varible = {}",x,y);

  println!("assigned type = {},{},{}",x2,f,b);
}