mod modules;
extern crate rand;
use rand::Rng;
/* pass by value ref struct module
*/

fn main() {
    // modules::print::run();
    // modules::variables::vars();
    // modules::ifelse::ifelse();
    // modules::infiniteloop::infiniteloop();
    // modules::whileloop::whileloop();
    // modules::forloop::forloop();
    // modules::enum_mod::enum_mod();
    // println!("{}",modules::VERSION);
    // modules::tuple_mod::tuple_mod();
    
    
    // funtions
    // modules::functions::print_2_max(20);
    // let x = modules::functions::is_even(20);
    // println!("{}",x);


    // code block are isolated codes that run in the code base
    // let x = 10;
    // {
    //     let y = 15;
    //     println!("{}--{}",x,y);
    // }


    // modules::refs::refs();
    // modules::structures::structures();
    // modules::array::array();
    // modules::struct2::struct2();
    // modules::traits::traits();
    // modules::readfile::read_file();
    // modules::evn_var::evn_var();
    // modules::write::write();
    // modules::matchs::matchs();
    // modules::userip::userip();
    // modules::hashmap::hashmap();

    println!("{}",rand::thread_rng().gen_range(1,11));

}
