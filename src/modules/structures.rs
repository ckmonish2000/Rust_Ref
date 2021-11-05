// basic struct
#[derive(Debug)]
struct Color{
  r: u8,
  g: u8,
  b: u8,
}

// tuple struct
#[derive(Debug)]
struct Colors(u8,u8,u8);

pub fn structures(){
  
  // basic struct
  let mut bg_color = Color{r:0,g:0,b:0};
  bg_color.r = 255;
  println!("{:?}", bg_color);

  // tuple struct
  let mut color = Colors(255,255,255);
  color.2 = 60;
  println!("{:?}", color);

  // pass by val

  // pass_by_val(color);
  

  // pass by ref 
  pass_by_ref(&mut color);
  println!("{:?} this is print",color);
}


fn pass_by_val(x:Colors){
  let mut y = x;
  y.0 = 0; 
  y.1 = 0;
  y.2 = 0;
  println!("{:?}", y)
}


fn pass_by_ref(x:&mut Colors){
  x.0 = 0; 
  x.1 = 0;
  x.2 = 0;
}