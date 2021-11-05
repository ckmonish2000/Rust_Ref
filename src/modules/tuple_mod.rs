pub fn tuple_mod(){
  let tup1: (u32,u32,f32,&str) = (10,20,30.5,"string");
  let tup2 = (10,20,30.5,"string",(1,4,7)); 
  // both tup 1 and 2 are same


  // refrencing to a particular val 
  println!("{:?}",tup2.4.2);
}