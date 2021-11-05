#[derive(Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right
}

pub fn enum_mod(){
  let player_direction: Direction = Direction::Down;

  match player_direction {
    Direction::Up =>{ println!("your going up"); }
    Direction::Down =>{ println!("your going down"); }
    Direction::Left =>{ println!("your going left"); }
    Direction::Right =>{ println!("your going right"); }
  }
}