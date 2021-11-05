pub fn array(){
  let numbers = [1,2,3,4,5];
  let num:[u32;2] = [1,2]; // static declaration
  
  println!("{}",numbers[0]);
  
  for i in numbers.iter() {
    println!("{}",i);
  }

  println!("{}",numbers[numbers.len()-1]);

}