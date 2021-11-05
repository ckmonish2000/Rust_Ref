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
  let color = Colors(255,255,255);
  println!("{:?}", color);


}