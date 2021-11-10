struct Person{
  name: String,
  age: u8
}

// custom traits 
trait HasVoiceBox{
  fn speak(&self);
  fn canspeak(&self,values:bool)->bool;
}


impl Person {
  fn new(name:String,age:u8) -> Person {
    Person { name :name, age :age }
  }
}

impl HasVoiceBox for Person{
  fn speak(&self){ 
    println!("hello");
  }

  fn canspeak(&self,value:bool) -> bool{
    return value;
  }
}

// predefined traits
impl ToString for Person {
  fn to_string(&self) -> String {
    return format!("name = {} age ={}", &self.name,&self.age);
  }
}

pub fn traits(){
let dom = Person::new(String::from("mk"), 21);

// custom traits call
println!("{}",dom.canspeak(false));
dom.speak();

println!("{}",dom.to_string());
}