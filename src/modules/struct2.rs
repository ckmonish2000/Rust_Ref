#[derive(Debug)]
struct Rect{
  width: u32,
  height: u32,
}

impl Rect {

  fn rect_desc(&self) {
    println!("height = {} width = {} \n",&self.height,&self.width);
  }

  fn is_square(&self) -> bool {
    &self.height == &self.width
  }

  // returns new rect from exsisting one
  fn scale(&self, scale:u32) -> Rect {
     return Rect{
      width: scale * &self.width,
      height: scale * &self.height
    }
  }

}

pub fn struct2(){
  let my_rect = Rect{width: 10, height: 20};
  let scaled_rect = my_rect.scale(10);

  scaled_rect.rect_desc();

  println!("{:?} {}",scaled_rect,scaled_rect.is_square());
}