pub fn array(){
  let numbers = [1,2,3,4,5];
  let num:[u32;2] = [1,2]; // static declaration
  let num2 = [2;10];// default values
  
  println!("{}",num2[0]);
  
  for i in numbers.iter() {
    println!("{}",i);
  }

  println!("{}",numbers[numbers.len()-1]);

}