use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{Packet};
use pnet::packet::ethernet::{EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use rusqlite::{params, Connection, Result};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use base64::encode;
use hex::encode as hex_encode;
use std::sync::{Arc, Mutex};
use tauri::State;

#[derive(Serialize, Deserialize)]
struct PacketData {
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

pub struct AppState {
    pub running: Arc<Mutex<bool>>,
    conn: Arc<Mutex<Connection>>,
}

impl Default for AppState {
    fn default() -> Self {
        let conn = Connection::open("packet_data.db").expect("Failed to open database");
        Self {
            running: Arc::new(Mutex::new(false)),
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl AppState {
    pub fn set_running(&self, value: bool) {
        let mut running = self.running.lock().unwrap();
        *running = value;
    }

    pub fn get_running(&self) -> bool {
        let running = self.running.lock().unwrap();
        *running
    }

    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)
    }
}

fn start_sniffer(app_state: Arc<AppState>) {
    let interfaces = datalink::interfaces();
    let interface_name = "lo"; // Set your interface name here
    let interface = interfaces.iter().find(|iface| iface.name == interface_name)
        .expect("Interface not found");

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, rx)) => ((), rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(err) => {
            eprintln!("Error occurred while creating channel: {}", err);
            return;
        }
    };

    let conn_arc = app_state.get_connection();
    let mut db_conn = conn_arc.lock().unwrap();

    let now: DateTime<Utc> = Utc::now();
    let table_name = format!("packet_data_{}", now.format("%Y%m%d%H%M%S"));

    let create_table_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            timestamp TEXT NOT NULL,
            packet_type TEXT NOT NULL,
            source TEXT NOT NULL,
            destination TEXT NOT NULL,
            protocol TEXT,
            payload_base64 TEXT,
            payload_hex TEXT,
            payload_raw BLOB,
            payload_string TEXT
        )",
        table_name
    );

    db_conn.execute(&create_table_query, []).expect("Failed to create table");

    println!("Listening on the interface: {}", interface_name);

    while app_state.get_running() {
        if let Ok(packet) = rx.next() {
            let ether_packet = EthernetPacket::new(packet).unwrap();
            handle_ethernet_packets(&ether_packet, &db_conn, &table_name).expect("Failed to handle packet");
        }
    }
}

#[tauri::command]
fn start_packet_sniffer(state: State<'_, Arc<AppState>>) {
    state.set_running(true);
    start_sniffer(Arc::clone(&state));
}

#[tauri::command]
fn stop_packet_sniffer(state: State<'_, Arc<AppState>>) {
    state.set_running(false);
}

fn handle_ethernet_packets(packet: &EthernetPacket, conn: &Connection, table_name: &str) -> Result<(), Box<dyn std::error::Error>> {
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

fn handle_ipv4_packets(packet: &Ipv4Packet, conn: &Connection, table_name: &str, timestamp: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("IPv4 Packet: {} -> {}; Protocol: {:?}", packet.get_source(), packet.get_destination(), packet.get_next_level_protocol());
    let payload_raw = packet.payload().to_vec();
    let packet_data = PacketData {
        timestamp: timestamp.to_string(),
        packet_type: "IPv4".to_string(),
        source: packet.get_source().to_string(),
        destination: packet.get_destination().to_string(),
        protocol: Some(format!("{:?}", packet.get_next_level_protocol())),
        payload_base64: encode(&payload_raw),
        payload_hex: hex_encode(&payload_raw),
        payload_raw: payload_raw.clone(),
        payload_string: String::from_utf8_lossy(&payload_raw).to_string(),
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

fn handle_ipv6_packets(packet: &Ipv6Packet, conn: &Connection, table_name: &str, timestamp: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("IPv6 Packet: {} -> {}; Protocol: {:?}", packet.get_source(), packet.get_destination(), packet.get_next_header());
    let payload_raw = packet.payload().to_vec();
    let packet_data = PacketData {
        timestamp: timestamp.to_string(),
        packet_type: "IPv6".to_string(),
        source: packet.get_source().to_string(),
        destination: packet.get_destination().to_string(),
        protocol: Some(format!("{:?}", packet.get_next_header())),
        payload_base64: encode(&payload_raw),
        payload_hex: hex_encode(&payload_raw),
        payload_raw: payload_raw.clone(),
        payload_string: String::from_utf8_lossy(&payload_raw).to_string(),
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

fn main() {
    let app_state = Arc::new(AppState::default());
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![start_packet_sniffer, stop_packet_sniffer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
