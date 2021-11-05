struct Person{
  name: String,
  age: u8
}

impl Person {
  fn new(name:String,age:u8) -> Person {
    Person { name :name, age :age }
  }
}

impl ToString for Person {
  fn to_string(&self) -> String {
    return format!("name = {} age ={}", &self.name,&self.age);
  }
}

pub fn traits(){
let dom = Person::new(String::from("mk"), 21);

println!("{}",dom.to_string());
}