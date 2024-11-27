// Project should be able to generate an ethernet packet and send it to a desired interface.

use pnet::datalink::{self, NetworkInterface};
use pnet::packet::Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ethernet::EthernetPacket;
use pnet:: datalink::Channel::Ethernet;


fn gen_string() {
    // generate a random string
    
}


pub fn send_packets() {
    println!("Hello, world!");
}
