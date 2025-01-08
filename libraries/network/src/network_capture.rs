extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::packet::Packet;
// use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ethernet::EthernetPacket;
use pnet:: datalink::Channel::Ethernet;

use encrypt::encrypt_text;


pub fn get_interfaces() -> Vec<NetworkInterface> {
    // get network interfaces print them to screen then return a vector of them
    let interfaces = datalink::interfaces();
    for interface in &interfaces {
        println!("{:#?}", interface.name);
    }
    
    return interfaces;
}


pub fn capture() {

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
                if let Some(eth_packet) = EthernetPacket::new(packet) {
                    if let Some(ip_packet) = Ipv4Packet::new(eth_packet.payload()) {

                        //Filter UDP packet
                        if ip_packet.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
                            if let Some(udp_packet) = UdpPacket::new(ip_packet.payload()) {
                                let raw_payload = udp_packet.payload();
                                
                                // Add decryption logic here

                                match std::str::from_utf8(raw_payload) {
                                    Ok(text) => println!("payload as text: {}", text),
                                    Err(_) => println!("payload is not utf-8"),
                                }
                            }
                        }


                    }
                
                } else {
                    println!("failed to parse packet");
                }
            },
            Err(e) => {
                eprintln!("Failed to receive packet: {:?}", e);
            }
        }
    }
}
