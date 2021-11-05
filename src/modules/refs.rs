pub fn refs(){

  let mut name ="domenic";
  
  {
    let name_r_2 = &name; // immutable reference
    let name_r = &mut name; // mutable refrence
    *name_r = "dom";
  }

  //  in a given line we can have multiple refrences for  given var
  //  but we can have only one refrence to a mutable refrence
  //  example println!("{}",name,name_r); run it with out the code block
  println!("{}",name);
}