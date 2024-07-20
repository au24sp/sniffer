extern crate pnet;
extern crate rusqlite;
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate base64;
extern crate hex;

use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{Packet};
use pnet::packet::ethernet::{EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use rusqlite::{params, Connection, Result};
use std::env;
use std::process::exit;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use base64::encode;
use hex::encode as hex_encode;

#[derive(Serialize, Deserialize)]
pub struct PacketData {
    timestamp: String,
    packet_type: String,
    source: String,
    destination: String,
    protocol: Option<String>,
    payload_base64: String,
    payload_hex: String,
    payload_raw: Vec<u8>,
    payload_string: String,
}

pub fn handle_ethernet_packets(packet: &EthernetPacket, conn: &Connection, table_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.to_rfc3339();

    println!("Ethernet Packet: {} -> {}; EtherType: {:?}", packet.get_source(), packet.get_destination(), packet.get_ethertype());

    match packet.get_ethertype() {
        pnet::packet::ethernet::EtherTypes::Ipv4 => {
            if let Some(ipv4_packet) = Ipv4Packet::new(packet.payload()) {
                handle_ipv4_packets(&ipv4_packet, conn, table_name, &timestamp)?;
            }
        }
        pnet::packet::ethernet::EtherTypes::Ipv6 => {
            if let Some(ipv6_packet) = Ipv6Packet::new(packet.payload()) {
                handle_ipv6_packets(&ipv6_packet, conn, table_name, &timestamp)?;
            }
        }
        _ => eprintln!("Unknown Ethernet packet type"),
    }

    Ok(())
}

pub fn handle_ipv4_packets(packet: &Ipv4Packet, conn: &Connection, table_name: &str, timestamp: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("IPv4 Packet: {} -> {}; Protocol: {:?}", packet.get_source(), packet.get_destination(), packet.get_next_level_protocol());
    let payload_raw = packet.payload().to_vec();
    let packet_data = PacketData {
        timestamp: timestamp.to_string(),
        packet_type: "IPv4".to_string(),
        source: packet.get_source().to_string(),
        destination: packet.get_destination().to_string(),
        protocol: Some(format!("{:?}", packet.get_next_level_protocol())),
        payload_base64: encode(&payload_raw), // Encode payload to Base64
        payload_hex: hex_encode(&payload_raw), // Encode payload to Hex
        payload_raw: payload_raw.clone(), // Store raw payload as bytes
        payload_string: String::from_utf8_lossy(&payload_raw).to_string(), // Store payload as string
    };

    let insert_query = format!(
        "INSERT INTO {} (timestamp, packet_type, source, destination, protocol, payload_base64, payload_hex, payload_raw, payload_string) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        table_name
    );
    conn.execute(
        &insert_query,
        params![
            packet_data.timestamp,
            packet_data.packet_type,
            packet_data.source,
            packet_data.destination,
            packet_data.protocol,
            packet_data.payload_base64,
            packet_data.payload_hex,
            packet_data.payload_raw,
            packet_data.payload_string,
        ],
    )?;
    Ok(())
}

pub fn handle_ipv6_packets(packet: &Ipv6Packet, conn: &Connection, table_name: &str, timestamp: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("IPv6 Packet: {} -> {}; Protocol: {:?}", packet.get_source(), packet.get_destination(), packet.get_next_header());
    let payload_raw = packet.payload().to_vec();
    let packet_data = PacketData {
        timestamp: timestamp.to_string(),
        packet_type: "IPv6".to_string(),
        source: packet.get_source().to_string(),
        destination: packet.get_destination().to_string(),
        protocol: Some(format!("{:?}", packet.get_next_header())),
        payload_base64: encode(&payload_raw), // Encode payload to Base64
        payload_hex: hex_encode(&payload_raw), // Encode payload to Hex
        payload_raw: payload_raw.clone(), // Store raw payload as bytes
        payload_string: String::from_utf8_lossy(&payload_raw).to_string(), // Store payload as string
    };

    let insert_query = format!(
        "INSERT INTO {} (timestamp, packet_type, source, destination, protocol, payload_base64, payload_hex, payload_raw, payload_string) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        table_name
    );
    conn.execute(
        &insert_query,
        params![
            packet_data.timestamp,
            packet_data.packet_type,
            packet_data.source,
            packet_data.destination,
            packet_data.protocol,
            packet_data.payload_base64,
            packet_data.payload_hex,
            packet_data.payload_raw,
            packet_data.payload_string,
        ],
    )?;
    Ok(())
}
