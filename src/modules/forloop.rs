pub fn forloop(){
  // 2nd part of for must be a iterator
  
  let range  = 0..11;
  // 0..10 range from 0 to 9

  let animals: Vec<&str> = vec!["dog","cat"];
  
  // using iter will not transfer ownership 
  for i in animals.iter(){
    println!("{}",i);
  }

  println!("{}",animals[0]);
}