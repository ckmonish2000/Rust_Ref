mod modules;

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


    // code block
    let x = 10;

    {
        let y = 15;
        println!("{}--{}",x,y);
    }

    println!("{}",y);

}
