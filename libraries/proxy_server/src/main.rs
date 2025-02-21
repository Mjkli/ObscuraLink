

mod proxy;

use std::env;



fn main() {
   

    let args: Vec<String> = env::args().collect();
    // proxy service to catch reuqests
    proxy::proxy(&args[1]);
    
}
