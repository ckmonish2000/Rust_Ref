struct Person{
  name: String,
  age: u8
}

impl Person {
  fn new(name:String,age:u8) -> Person {
    Person { name :name, age :age }
  }
}

pub fn traits(){
let dom = Person::new(String::from("mk"), 21);

println!("{}",dom.to_string());
}