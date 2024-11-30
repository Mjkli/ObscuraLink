// Project should be able to generate an ethernet packet and send it to a desired interface.

use pnet::datalink::{self, NetworkInterface};
use pnet::packet::Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ethernet::EthernetPacket;
use pnet:: datalink::Channel::Ethernet;

use utils::dictionary::Dictionary;

use std::path::Path;


pub fn send_packets() {
    // Get payload (String for now)
    let mut dic = Dictionary::new(Path::new("first-names.txt"));
    println!("Name: {}", dic.random());
   
    // Package the payload in a network packet
    // - capture is using a EthernetPacket-IPv4-TCP 
    
    // 1. how to create an Ethernet packet with payload
    // 2. how to send that packet on a selected interface.

    


}
