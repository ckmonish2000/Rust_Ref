#[derive(Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right
}

pub fn enum_mod(){
  let player_direction: Direction = Direction::Up;

  println!("{:?}", player_direction);
}