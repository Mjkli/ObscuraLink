// Project should be able to generate an ethernet packet and send it to a desired interface.

use pnet::packet::ethernet::{ Ethernet, EtherType };
use std::{thread, time};
use std::path::Path;


use utils::dictionary::Dictionary;
use crate::network_capture::get_interfaces;



pub fn send_packets() {
    // Package the payload in a network packet
    // - capture is using a EthernetPacket-IPv4-TCP 
    
    // 1. how to create an Ethernet packet with payload
    //      -- Since we are only testing on the lo interface we need the lo mac address to load the
    //      dest and source
    // 2. how to send that packet on a selected interface.

    // EtherTypes are defined using int and describe what kind of protocol the ethernet packet is
    // i.e. IPv4 / IPv6 / ARP / etc.
    // IPv4 = 2048
    // ieee reference:
    // payload in this case need to be a byte-array

    let interfaces = get_interfaces();

    let lo_int = interfaces
                    .into_iter()
                    .find(|iface| iface.name == "lo")
                    .expect("failed to get interface");


    let dic = Dictionary::new(Path::new("first-names.txt"));


    loop {
        let data = dic.random();
        println!("name: {}", data);
        
        let packet = Ethernet {
            destination: lo_int.mac.expect("Failed to get lo mac"),
            source: lo_int.mac.expect("Failed to get lo mac"),
            ethertype: EtherType(2048),
            payload: data.as_bytes().to_vec()
        };
        
        println!("packet: {:?}", packet);


        thread::sleep(time::Duration::from_secs(1));


    }
}
