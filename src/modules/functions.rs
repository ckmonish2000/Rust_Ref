pub fn print_2_max(x:u64){
  for i in 0..x+1{
    if(is_even(i)){
      println!("{}",i)
    }else{
        println!("---\t")
    }
  }
}

pub fn is_even(x:u64) -> bool{
  x%2 == 0  
}