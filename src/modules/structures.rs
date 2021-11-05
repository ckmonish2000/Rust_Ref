#[derive(Debug)]
struct Color{
  r: u8,
  g: u8,
  b: u8,
}

pub fn structures(){
  let mut bg_color = Color{r:0,g:0,b:0};
  bg_color.r = 255;
  println!("{:?}", bg_color);
}