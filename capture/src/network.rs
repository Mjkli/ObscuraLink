extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::packet::{ethernet::EthernetPacket, Packet};
use pnet:: datalink::Channel::Ethernet;



pub fn get_interfaces() -> Vec<NetworkInterface> {
    let interfaces = datalink::interfaces();
    for interface in &interfaces {
        println!("{:#?}", interface.name);
    }
    
    return interfaces;
}


pub fn network() {

    let interfaces = get_interfaces();  

    let interface = interfaces
                        .into_iter()
                        .find(|iface| iface.name == "lo")
                        .expect("Failed to find interface");

    let (mut _tx, mut rx) = match datalink::channel(&interface, Default::default()){
            Ok(Ethernet(_tx, rx)) => (_tx,rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!("err: {}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let eth_packet = EthernetPacket::new(packet).unwrap();
                // Now that we can capture the packets we need to send them out.

                let payload = match std::str::from_utf8(eth_packet.payload()){
                    Ok(v) => println!("{}", v),
                    Err(e) => println!("Invalid: {}", e),
                };
                
            },
            Err(e) => {
                eprintln!("Failed to receive packet: {:?}", e);
            }
        }
    }
}
