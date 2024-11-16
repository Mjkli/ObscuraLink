extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::packet::{Packet, ethernet::EthernetPacket};
use pnet:: datalink::Channel::Ethernet;

fn main() {
    println!("Hello, world!");

    // print interfaces
    let interfaces = datalink::interfaces();
    // for interface in interfaces {
    //     // println!("{:#?}", interface.name);
    // }

    // select interface you want "lo"
    
    let all_ints = interfaces;

    let interface = all_ints
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
                println!("Captured packet: {:?}", eth_packet);

            },
            Err(e) => {
                eprintln!("Failed to receive packet: {:?}", e);
            }
        }
    }
}
