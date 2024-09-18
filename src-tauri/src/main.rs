extern crate tokio;
use base64::encode;
use chrono::prelude::*;
use hex::encode as hex_encode;
use pnet::datalink::NetworkInterface;
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;
use reqwest::Client;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use tauri::http::Request;
use tauri::{Builder, State};

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

#[derive(Debug, Serialize, Deserialize)]
struct IpStats {
    source_count: u32,
    destination_count: u32,
}

pub struct AppState {
    pub running: Arc<AtomicBool>,
    pub conn: Arc<Mutex<Connection>>,
    pub handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

const rohith_url: &str = "/home/rohi/packet_data.db";
const RISHI_URL: &str = "/Users/Rishikumar/packet_data.db";
const ABHI_URL: &str = "/home/abhi/Documents/summerproj/packet_data.db";

impl Default for AppState {
    fn default() -> Self {
        let conn = Connection::open(rohith_url).expect("Failed to open database");
        Self {
            running: Arc::new(AtomicBool::new(false)),
            conn: Arc::new(Mutex::new(conn)),
            handle: Arc::new(Mutex::new(None)),
        }
    }
}

fn start_sniffer(app_state: Arc<AppState>, interface: String) {
    let interfaces = datalink::interfaces();
    let interface_name = interface.clone();
    let interface = interfaces
        .iter()
        .find(|iface| iface.name == interface_name)
        .expect("Interface not found");

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, rx)) => ((), rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(err) => {
            eprintln!("Error occurred while creating channel: {}", err);
            return;
        }
    };

    println!("{:?}", interface);

    let conn_arc = app_state.conn.clone();
    let db_conn = conn_arc.lock().unwrap();

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

    db_conn
        .execute(&create_table_query, [])
        .expect("Failed to create table");

    println!("Listening on the interface: {}", interface_name);

    while app_state.running.load(Ordering::SeqCst) {
        match rx.next() {
            Ok(packet) => {
                let ether_packet = EthernetPacket::new(packet).unwrap();
                handle_ethernet_packets(&ether_packet, &db_conn, &table_name)
                    .expect("Failed to handle packet");
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    println!("Packet sniffer stopped.");
}

#[tauri::command]
fn start_packet_sniffer(state: State<'_, Arc<AppState>>, interface: String) {
    if state.running.load(Ordering::SeqCst) {
        println!("Packet sniffer is already running.");
        return;
    }

    state.running.store(true, Ordering::SeqCst);
    let state_clone = state.inner().clone();
    let interface = interface.replace('"', "");
    println!("Packet sniffer started. {}", &interface);

    let handle = thread::spawn(move || {
        start_sniffer(state_clone, interface);
    });

    let mut handle_lock = state.handle.lock().unwrap();
    *handle_lock = Some(handle);
}

#[tauri::command]
fn stop_packet_sniffer(state: State<'_, Arc<AppState>>) {
    if !state.running.load(Ordering::SeqCst) {
        println!("Packet sniffer is not running.");
        return;
    }

    println!("Stopping packet sniffer...");
    state.running.store(false, Ordering::SeqCst);

    if let Some(handle) = state.handle.lock().unwrap().take() {
        handle.join().unwrap();
    }

    println!("Packet sniffer stopped.");
}

#[tauri::command]
fn list_names() -> Vec<String> {
    let con = Connection::open(rohith_url).expect("err in line 142");
    let mut smt = con
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .expect("err in table queryiong");
    let res_iter = smt.query_map([], |row| row.get(0)).unwrap();
    let mut res: Vec<_> = Vec::new();
    for i in res_iter {
        res.push(i.unwrap());
    }
    res
}

#[tauri::command]
fn get_table_data(table: &str) -> Vec<PacketData> {
    let conn = Connection::open(rohith_url).unwrap();
    let fromat_smt = format!("select * from {}", table);
    let mut smt = conn.prepare(&fromat_smt).unwrap();
    let result_iter = smt
        .query_map([], |row| {
            Ok(PacketData {
                // id                : row.get(0).unwrap(),
                timestamp: row.get(1).unwrap(),
                packet_type: row.get(2).unwrap(),
                source: row.get(3).unwrap(),
                destination: row.get(4).unwrap(),
                protocol: row.get(5).unwrap(),
                payload_base64: row.get(6).unwrap(),
                payload_hex: row.get(7).unwrap(),
                payload_raw: row.get(8).unwrap(),
                payload_string: row.get(9).unwrap(),
            })
        })
        .unwrap();
    let mut res = Vec::new();
    for i in result_iter {
        res.push(i.unwrap());
    }
    res
}

fn _llama_data_fetcher(table: &str) -> Vec<PacketData> {
    let conn = Connection::open(rohith_url).unwrap();
    let fromat_smt = format!("select * from {} LIMIT 50 OFFSET 2", table);
    let mut smt = conn.prepare(&fromat_smt).unwrap();
    let result_iter = smt
        .query_map([], |row| {
            Ok(PacketData {
                // id                : row.get(0).unwrap(),
                timestamp: row.get(1).unwrap(),
                packet_type: row.get(2).unwrap(),
                source: row.get(3).unwrap(),
                destination: row.get(4).unwrap(),
                protocol: row.get(5).unwrap(),
                payload_base64: row.get(6).unwrap(),
                payload_hex: row.get(7).unwrap(),
                payload_raw: row.get(8).unwrap(),
                payload_string: row.get(9).unwrap(),
            })
        })
        .unwrap();
    let mut res = Vec::new();
    for i in result_iter {
        res.push(i.unwrap());
    }
    res
}

#[tauri::command]
async fn handle_ollama(table: &str) -> Result<serde_json::Value, String> {
    println!("table: {}", table);
    let data = _llama_data_fetcher(table);
    let formatted_data = data.iter()
        .map(|packet| format!(
            "Timestamp: {}, Packet Type: {}, Source: {}, Destination: {}, Protocol: {:?}, Payload (String): {}",
            packet.timestamp,
            packet.packet_type,
            packet.source,
            packet.destination,
            packet.protocol,
            packet.payload_string
        ))
        .collect::<Vec<String>>()
        .join("\n");
    let prompt = format!(
        "Analyze the Data like a senior data analyst:\n{}",
        formatted_data
    );

    let client = Client::new();
    let api_url = "http://localhost:11434/api/generate";
    let response = client
        .post(api_url)
        .json(&json!({
            "model": "llama3.1",
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_text = response.text().await.map_err(|e| e.to_string())?;

    println!("Response: {}", response_text);

    Ok(json!({
        "response": response_text
    }))
}

#[derive(Debug, Serialize, Deserialize)]
struct NetInterface {
    name: String,
    mac: String,
    ipv4: String,
}

#[tauri::command]
fn list_interfacce() -> Vec<NetInterface> {
    let interface = datalink::interfaces();
    let mut res: Vec<NetInterface> = Vec::new();
    for _tmp_interfaces in interface.iter() {
        let _ipv4 = _tmp_interfaces
            .ips
            .get(0)
            .map(|ip| ip.ip().to_string())
            .unwrap_or_else(|| "N/A".to_string());
        println!(
            "| {:<14} |  {:<3}  | {:<16} | {:<14} |",
            _tmp_interfaces.name,
            _tmp_interfaces.index,
            _tmp_interfaces.mac.unwrap(),
            _ipv4
        );
        let mut tmp_strt = NetInterface {
            name: format!("{:?}", _tmp_interfaces.name),
            mac: format!("{:?}", _tmp_interfaces.mac),
            ipv4: _ipv4,
        };
        res.push(tmp_strt);
    }
    res
}

fn handle_ethernet_packets(
    packet: &EthernetPacket,
    conn: &Connection,
    table_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.to_rfc3339();

    println!(
        "Ethernet Packet: {} -> {}; EtherType: {:?}",
        packet.get_source(),
        packet.get_destination(),
        packet.get_ethertype()
    );

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

fn handle_ipv4_packets(
    packet: &Ipv4Packet,
    conn: &Connection,
    table_name: &str,
    timestamp: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "IPv4 Packet: {} -> {}; Protocol: {:?}",
        packet.get_source(),
        packet.get_destination(),
        packet.get_next_level_protocol()
    );
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

fn handle_ipv6_packets(
    packet: &Ipv6Packet,
    conn: &Connection,
    table_name: &str,
    timestamp: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "IPv6 Packet: {} -> {}; Protocol: {:?}",
        packet.get_source(),
        packet.get_destination(),
        packet.get_next_header()
    );
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

fn query_ip_stats(conn: &Connection, table_name: &str) -> Result<HashMap<String, IpStats>> {
    let mut stmt = conn.prepare(&format!("SELECT source, destination FROM {}", table_name))?;

    let mut ip_stats: HashMap<String, IpStats> = HashMap::new();
    let rows = stmt.query_map(params![], |row| {
        let source: String = row.get(0)?;
        let destination: String = row.get(1)?;
        Ok((source, destination))
    })?;
    for row in rows {
        let (source, destination) = row?;

        ip_stats
            .entry(source)
            .and_modify(|stats| stats.source_count += 1)
            .or_insert(IpStats {
                source_count: 1,
                destination_count: 0,
            });

        ip_stats
            .entry(destination)
            .and_modify(|stats| stats.destination_count += 1)
            .or_insert(IpStats {
                source_count: 0,
                destination_count: 1,
            });
    }

    Ok(ip_stats)
}

fn query_packet_per_second(
    conn: &Connection,
    table_name: &str,
) -> Result<HashMap<String, u32>, rusqlite::Error> {
    let mut packet_count: HashMap<String, u32> = HashMap::new();
    let query = format!("SELECT timestamp FROM {}", table_name);
    let mut stmt = conn.prepare(&query)?;

    let packet_iter = stmt.query_map([], |row| {
        let timestamp: String = row.get(0)?;
        Ok(timestamp)
    })?;

    for packet in packet_iter {
        let timestamp = packet?;
        let time_part = timestamp.split('T').nth(1).unwrap_or(&timestamp);
        let formatted_time = time_part.split('.').next().unwrap_or(&time_part);
        let count = packet_count.entry(formatted_time.to_string()).or_insert(0);
        *count += 1;
    }

    Ok(packet_count)
}

fn query_packet_types(conn: &Connection, table_name: &str) -> Result<HashMap<String, u32>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT packet_type, COUNT(*) as count
         FROM {}
         GROUP BY packet_type",
        table_name
    ))?;

    let mut packet_types: HashMap<String, u32> = HashMap::new();

    let rows = stmt.query_map(params![], |row| {
        let packet_type: String = row.get(0)?;
        let count: u32 = row.get(1)?;
        Ok((packet_type, count))
    })?;

    for row in rows {
        let (packet_type, count) = row?;
        packet_types.insert(packet_type, count);
    }

    Ok(packet_types)
}

#[tauri::command]
fn get_ip_stats(table_name: &str) -> Result<Vec<serde_json::Value>, String> {
    let conn = Connection::open(rohith_url).map_err(|e| e.to_string())?;
    let ip_stats = query_ip_stats(&conn, table_name).map_err(|e| e.to_string())?;

    let formatted_ip_stats: Vec<serde_json::Value> = ip_stats
        .into_iter()
        .map(|(ip, stats)| {
            json!({
                "IP": ip,
                "Source": stats.source_count,
                "Destination": stats.destination_count
            })
        })
        .collect();

    Ok(formatted_ip_stats)
}

#[tauri::command]
fn get_packet_per_second(table_name: &str) -> Result<Vec<serde_json::Value>, String> {
    let conn = Connection::open(rohith_url).map_err(|e| e.to_string())?;
    let packet_per_second =
        query_packet_per_second(&conn, table_name).map_err(|e| e.to_string())?;

    let mut formatted_packet_per_second: Vec<(String, u32)> =
        packet_per_second.into_iter().collect();
    formatted_packet_per_second.sort_by(|a, b| a.0.cmp(&b.0));

    let formatted_packet_per_second: Vec<serde_json::Value> = formatted_packet_per_second
        .into_iter()
        .map(|(timestamp, count)| {
            json!({
                "timeStamp": timestamp,
                "traffic": count
            })
        })
        .collect();

    Ok(formatted_packet_per_second)
}

#[tauri::command]
fn get_packet_types(table_name: &str) -> Result<Vec<serde_json::Value>, String> {
    let conn = Connection::open(rohith_url).map_err(|e| e.to_string())?;
    let packet_types = query_packet_types(&conn, table_name).map_err(|e| e.to_string())?;

    let formatted_packet_types: Vec<serde_json::Value> = packet_types
        .into_iter()
        .map(|(packet_type, count)| {
            json!({
                "type": packet_type,
                "count": count
            })
        })
        .collect();

    Ok(formatted_packet_types)
}

#[tokio::main]
async fn main() {
    Builder::default()
        .manage(Arc::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            start_packet_sniffer,
            handle_ollama,
            stop_packet_sniffer,
            list_names,
            list_interfacce,
            get_table_data,
            get_packet_types,
            get_packet_per_second,
            get_ip_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

