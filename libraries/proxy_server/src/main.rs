

mod proxy;

use std::env;



fn main() {
   

    let args: Vec<String> = env::args().collect();
    // proxy service to catch reuqests
    if args.len() > 2 {
        proxy::proxy(&args[1], &args[2]);
    } else {
        proxy::proxy(&args[1], "");
    }
}
