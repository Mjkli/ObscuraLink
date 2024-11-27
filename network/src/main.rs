// Project to handle network portion of sending and receiving messages.

mod network_capture;
mod network_sender;

use std::env;
use std::process::exit;


fn main() {
    // Setup command line arguments to switch between sending and receiving setups
    // let args: Vec<String> = env::args().collect();

    // if args.len() <= 1 {
    //     eprintln!("Enter arguments: capture or send");
    //     exit(1);
    // }
    
    // let argument = &args[1];

    // if argument == "capture" {
    //     network_capture::capture();
    // }
    // else if argument == "send" {

    // } else {
    //     eprintln!("Valid arguments: capture, send");
    // }

    network_sender::send_packets();

}
