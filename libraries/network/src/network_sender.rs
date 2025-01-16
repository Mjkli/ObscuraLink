// Project should be able to generate an ethernet packet and send it to a desired interface.

use pnet:: datalink;
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::{ checksum as ipv4_checksum, MutableIpv4Packet};
use pnet::packet::{Packet, PacketSize};
use pnet::packet::udp::{ ipv4_checksum as udp_ipv4_checksum, MutableUdpPacket};



use std::thread::sleep;
use utils::dictionary::Dictionary;
use crate::network_capture::get_interfaces;
use std::time::Duration;
use std::net::IpAddr;
use std::path::{self, Path, absolute};

use encrypt::encrypt_text::encrypt_text;


pub fn send_packets(key: Vec<u8>, iv: Vec<u8>) {
    // Package the payload in a network packet


    let interfaces = get_interfaces();

    let lo_int = interfaces
                    .into_iter()
                    .find(|iface| iface.name == "lo")
                    .expect("failed to get interface");
    
    let p = absolute("data/first-names.txt");
    let dic = Dictionary::new(&p.unwrap());

    // Need to setup tx, rx channels and attach them to an interface.
    let (mut tx, mut _rx) =  match datalink::channel(&lo_int, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx,rx)) => (tx,rx),
        Ok(_) => panic!("Unhandeled channel type"),
        Err(e) => panic!("err: {}", e)
    };

    loop {
        let data = dic.random();
        println!("name: {}", data);

         
        let encrypted_data = encrypt_text(data, &key, &iv);


        // Setup checksum
        let source_ip = match lo_int.ips[0].ip() {
            IpAddr::V4(ipv4) => {ipv4}
            IpAddr::V6(_ipv6) => { 
                eprintln!("Not working with ipv6");
                0.into()
            }
        };

        let destination_ip = source_ip;
        
        //Build Udp Packet

        let mut udp_buffer = vec![0u8; MutableUdpPacket::minimum_packet_size() + encrypted_data.len()];
        {
            let mut udp_packet = MutableUdpPacket::new(&mut udp_buffer).expect("Error");
            udp_packet.set_source(80);
            udp_packet.set_destination(80);
            udp_packet.set_length((MutableUdpPacket::minimum_packet_size() + encrypted_data.len()) as u16);
            udp_packet.set_payload(&encrypted_data);
        


            let udp_checksum = udp_ipv4_checksum(&udp_packet.to_immutable(), &source_ip, &destination_ip);
            udp_packet.set_checksum(udp_checksum);
        }

        //Build IPv4
        let mut ipv4_buff = vec![0u8; 20 + udp_buffer.len()];
        {
            let mut ipv4_packet = MutableIpv4Packet::new(&mut ipv4_buff).expect("error");
            ipv4_packet.set_version(4);
            ipv4_packet.set_header_length(5);
            ipv4_packet.set_total_length((20 + udp_buffer.len()) as u16);
            ipv4_packet.set_identification(0x1234);
            ipv4_packet.set_ttl(64);
            ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Udp);
            ipv4_packet.set_source(source_ip);
            ipv4_packet.set_destination(destination_ip);
            ipv4_packet.set_payload(&udp_buffer);
            ipv4_packet.set_checksum(ipv4_checksum(&ipv4_packet.to_immutable()));

        }

        // // Build ethernet frame
        let mut frame = vec![0u8; MutableEthernetPacket::minimum_packet_size() + ipv4_buff.len()];
        {
            let mut packet = MutableEthernetPacket::new(&mut frame).unwrap();
            packet.set_destination(lo_int.mac.expect("failed to get mac"));
            packet.set_source(lo_int.mac.expect("failed to get mac"));
            packet.set_ethertype(EtherTypes::Ipv4);
            packet.set_payload(&ipv4_buff);

        }

        match tx.send_to(&frame, Some(lo_int.clone())) {
            Some(Ok(_)) => println!("Sent packet: {:?}", frame),
            Some(Err(e)) => eprintln!("Failed to send packet: {}", e),
            None => eprintln!("Failed to send packet: No result")
        }
        
        sleep(Duration::from_secs(1));
    }
}
