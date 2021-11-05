pub fn refs(){

  let mut name ="domenic";
  let name_r = &mut name;

  *name_r = "dom";
  println!("{} ",name);
}