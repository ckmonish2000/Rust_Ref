use std::collections::HashMap;

pub fn hashmap(){
 let mut map = HashMap::new();

 map.insert("monish","7892509760");
 map.insert("dad","9341877882");
 let x = map.get("monish").unwrap();
 println!("{}",x);
}