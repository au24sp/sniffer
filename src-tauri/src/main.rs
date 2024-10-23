extern crate tokio;
use pnet::datalink::NetworkInterface;
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::{Packet};
use pnet::packet::ethernet::{EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use rusqlite::{params, Connection, Result};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use base64::encode;
use hex::encode as hex_encode;
use serde_json::json;
use tauri::http::Request;
use std::fmt::format;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use tauri::{State, Builder};
use reqwest::Client;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::env;
use std::path::PathBuf;
use tauri::command;



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
    // applayer: String
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

const rohith_url:&str = "/home/rohi/packet_data.db";
const RISHI_URL:&str = "/Users/Rishikumar/packet_data.db";
const ABHI_URL:&str = "/home/abhi/Documents/summerproj/packet_data.db";

impl Default for AppState {
    fn default() -> Self {
        let conn = Connection::open(RISHI_URL).expect("Failed to open database");
        Self {
            running: Arc::new(AtomicBool::new(false)),
            conn: Arc::new(Mutex::new(conn)),
            handle: Arc::new(Mutex::new(None)),
        }
    }
}

fn start_sniffer(app_state: Arc<AppState>,interface : String) {
    let interfaces = datalink::interfaces();
    let interface_name = interface.clone(); // Set your interface name here
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

    println!("{:?}",interface);

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

    db_conn.execute(&create_table_query, []).expect("Failed to create table");

    println!("Listening on the interface: {}", interface_name);

    while app_state.running.load(Ordering::SeqCst) {
        match rx.next() {
            Ok(packet) => {
                let ether_packet = EthernetPacket::new(packet).unwrap();
                handle_ethernet_packets(&ether_packet, &db_conn, &table_name).expect("Failed to handle packet");
            },
            Err(_) => {
                thread::sleep(Duration::from_millis(100)); // Small sleep to avoid busy-waiting
            }
        }
    }

    println!("Packet sniffer stopped.");
}

#[tauri::command]
fn start_packet_sniffer(state: State<'_, Arc<AppState>>,interface : String) {
    if state.running.load(Ordering::SeqCst) {
        println!("Packet sniffer is already running.");
        return;
    }

    state.running.store(true, Ordering::SeqCst);
    let state_clone = state.inner().clone(); // Use the `inner` method to get the `Arc<AppState>`
    let interface = interface.replace('"', "");
    println!("Packet sniffer started. {}",&interface);

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
fn list_names()->Vec<String> {
    let con = Connection::open(RISHI_URL).expect("err in line 142");
    let mut smt = con.prepare("SELECT name FROM sqlite_master WHERE type='table'").expect("err in table queryiong");
    let res_iter = smt.query_map([], |row|{
        row.get(0)
    }).unwrap();
    let mut res: Vec<_> = Vec::new();
    for i in res_iter {
        res.push(i.unwrap());
    }
    res
}

#[tauri::command]
fn get_table_data(table: &str) -> Vec<PacketData> {
    let conn = Connection::open(RISHI_URL).unwrap();
    let mut fromat_smt = format!("select * from {}",table);
    let mut smt = conn.prepare(&fromat_smt).unwrap();
    let result_iter = smt.query_map([], |row|{
        Ok(PacketData{
            // id                : row.get(0).unwrap(),
            timestamp         : row.get(1).unwrap(),
            packet_type       : row.get(2).unwrap(),
            source            : row.get(3).unwrap(),
            destination       : row.get(4).unwrap(),
            protocol          : row.get(5).unwrap(),
            payload_base64    : row.get(6).unwrap(),
            payload_hex       : row.get(7).unwrap(),
            payload_raw       : row.get(8).unwrap(),
          payload_string      : row.get(9).unwrap(),
        //   applayer            : row.get(10).unwrap()
          
        })
    }).unwrap();
    let mut res = Vec::new();
    for i in result_iter {
        res.push(i.unwrap());
    }
    res

}

#[derive(Debug,Serialize,Deserialize)]
struct NetInterface {
    name : String,
    mac  : String,
    ipv4 : String
}

#[tauri::command]
fn list_interfacce() -> Vec<NetInterface> {
    let interface = datalink::interfaces();
    let mut res:Vec<NetInterface> = Vec::new();
    for _tmp_interfaces in interface.iter() {
        let _ipv4 = _tmp_interfaces.ips.get(0).map(|ip| ip.ip().to_string()).unwrap_or_else(|| "N/A".to_string());
        println!("| {:<14} |  {:<3}  | {:<16} | {:<14} |",_tmp_interfaces.name,_tmp_interfaces.index, _tmp_interfaces.mac.unwrap(),_ipv4 );
        let mut tmp_strt = NetInterface {
            name : format!("{:?}",_tmp_interfaces.name),
            mac  : format!("{:?}",_tmp_interfaces.mac),
            ipv4 : _ipv4
        };
        res.push(tmp_strt);
    }
    res
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

fn get_next_level_protocol(protocol: &IpNextHeaderProtocol) -> &'static str {
    match protocol.0 {
        0 => "IPv6 Hop-by-Hop Option",
        1 => "ICMP",
        2 => "IGMP",
        3 => "Gateway-to-Gateway",
        4 => "IPv4 encapsulation",
        5 => "Stream",
        6 => "TCP",
        7 => "CBT",
        8 => "Exterior Gateway Protocol",
        9 => "Interior Gateway Protocol (IGP)",
        10 => "BBN RCC Monitoring",
        11 => "Network Voice Protocol",
        12 => "PUP",
        13 => "ARGUS (deprecated)",
        14 => "EMCON",
        15 => "Cross Net Debugger",
        16 => "CHAOS",
        17 => "UDP",
        18 => "Multiplexing",
        19 => "DCN Measurement Subsystems",
        20 => "Host Monitoring",
        21 => "Packet Radio Measurement",
        22 => "XEROX NS IDP",
        23 => "Trunk-1",
        24 => "Trunk-2",
        25 => "Leaf-1",
        26 => "Leaf-2",
        27 => "Reliable Data Protocol",
        28 => "Internet Reliable Transaction",
        29 => "ISO Transport Protocol Class 4",
        30 => "Bulk Data Transfer Protocol",
        31 => "MFE Network Services Protocol",
        32 => "MERIT Internodal Protocol",
        33 => "Datagram Congestion Control Protocol",
        34 => "Third Party Connect Protocol",
        35 => "Inter-Domain Policy Routing Protocol",
        36 => "XTP",
        37 => "Datagram Delivery Protocol",
        38 => "IDPR Control Message Transport Protocol",
        39 => "TP++ Transport Protocol",
        40 => "IL Transport Protocol",
        41 => "IPv6 encapsulation",
        42 => "Source Demand Routing Protocol",
        43 => "Routing Header for IPv6",
        44 => "Fragment Header for IPv6",
        45 => "Inter-Domain Routing Protocol",
        46 => "Reservation Protocol",
        47 => "Generic Routing Encapsulation",
        48 => "Dynamic Source Routing Protocol",
        49 => "BNA",
        50 => "Encapsulating Security Payload (ESP)",
        51 => "Authentication Header (AH)",
        52 => "Integrated Net Layer Security",
        53 => "IP with Encryption (deprecated)",
        54 => "NBMA Address Resolution Protocol",
        55 => "Minimal IPv4 Encapsulation",
        56 => "Transport Layer Security Protocol",
        57 => "SKIP",
        58 => "ICMP for IPv6",
        59 => "No Next Header for IPv6",
        60 => "Destination Options for IPv6",
        61 => "Any host internal protocol",
        62 => "CFTP",
        63 => "Any local network",
        64 => "SATNET and Backroom EXPAK",
        65 => "Kryptolan",
        66 => "MIT Remote Virtual Disk Protocol",
        67 => "Internet Pluribus Packet Core",
        68 => "Any distributed file system",
        69 => "SATNET Monitoring",
        70 => "VISA Protocol",
        71 => "Internet Packet Core Utility",
        72 => "Computer Protocol Network Executive",
        73 => "Computer Protocol Heart Beat",
        74 => "Wang Span Network",
        75 => "Packet Video Protocol",
        76 => "Backroom SATNET Monitoring",
        77 => "SUN ND PROTOCOL-Temporary",
        78 => "Wideband Monitoring",
        79 => "Wideband EXPAK",
        80 => "ISO Internet Protocol",
        81 => "VMTP",
        82 => "Secure VMTP",
        83 => "VINES",
        84 => "Internet Protocol Traffic Manager",
        85 => "NSFNET-IGP",
        86 => "Dissimilar Gateway Protocol",
        87 => "TCF",
        88 => "EIGRP",
        89 => "OSPFIGP",
        90 => "Sprite RPC Protocol",
        91 => "Locus Address Resolution Protocol",
        92 => "Multicast Transport Protocol",
        93 => "AX.25 Frames",
        94 => "IP-within-IP Encapsulation Protocol",
        95 => "Mobile Internetworking Control Protocol (deprecated)",
        96 => "Semaphore Communications Sec. Protocol",
        97 => "Ethernet-within-IP Encapsulation",
        98 => "Encapsulation Header",
        99 => "Any private encryption scheme",
        100 => "GMTP",
        101 => "Ipsilon Flow Management Protocol",
        102 => "PNNI over IP",
        103 => "Protocol Independent Multicast",
        104 => "ARIS",
        105 => "SCPS",
        106 => "QNX",
        107 => "Active Networks",
        108 => "IP Payload Compression Protocol",
        109 => "Sitara Networks Protocol",
        110 => "Compaq Peer Protocol",
        111 => "IPX in IP",
        112 => "Virtual Router Redundancy Protocol",
        113 => "PGM Reliable Transport Protocol",
        114 => "Any 0-hop protocol",
        115 => "Layer Two Tunneling Protocol (L2TP)",
        116 => "D-II Data Exchange",
        117 => "Interactive Agent Transfer Protocol",
        118 => "Schedule Transfer Protocol",
        119 => "SpectraLink Radio Protocol",
        120 => "UTI",
        121 => "Simple Message Protocol",
        122 => "Simple Multicast Protocol (deprecated)",
        123 => "Performance Transparency Protocol",
        124 => "ISIS over IPv4",
        125 => "FIRE",
        126 => "Combat Radio Transport Protocol",
        127 => "Combat Radio User Datagram",
        128 => "SSCOPMCE",
        129 => "IPLT",
        130 => "Secure Packet Shield",
        131 => "Private IP Encapsulation within IP",
        132 => "Stream Control Transmission Protocol (SCTP)",
        133 => "Fibre Channel",
        134 => "RSVP-E2E-IGNORE",
        135 => "Mobility Header",
        136 => "UDP Lite",
        137 => "MPLS-in-IP",
        138 => "MANET Protocols",
        139 => "Host Identity Protocol",
        140 => "Shim6 Protocol",
        141 => "Wrapped Encapsulating Security Payload",
        142 => "Robust Header Compression",
        143 => "Ethernet",
        144 => "AGGFRAG encapsulation payload for ESP",
        145 => "Network Service Header",
        146..=252 => "Unassigned",
        253 => "Use for experimentation and testing",
        254 => "Use for experimentation and testing",
        255 => "Reserved",
        _ => "Unknown Protocol",
    }
}


fn identify_application_layer_protocol(packet: &[u8], protocol: IpNextHeaderProtocol) -> Option<&'static str> {
    match protocol.0 {
        6 => {  // TCP protocol
            if let Some(tcp) = TcpPacket::new(packet) {
                let src_port = tcp.get_source();
                let dst_port = tcp.get_destination();
                println!("{} {}",src_port,dst_port);
                return identify_tcp_application(src_port, dst_port);
            }
        },
        17 => {  // UDP protocol
            if let Some(udp) = UdpPacket::new(packet) {
                let src_port = udp.get_source();
                let dst_port = udp.get_destination();
                println!("{} {}",src_port,dst_port);
                return identify_udp_application(src_port, dst_port);
            }
        },
        _ => return None,
    }
    None
}

fn identify_tcp_application(src_port: u16, dst_port: u16) -> Option<&'static str> {
    match dst_port {
        0 => Some("Reserved"),
        1 => Some("TCP Port Service Multiplexer (TCPMUX)"),
        2 => Some("Management Utility"),
        3 => Some("CompressNET Management Utility"),
        5 => Some("Remote Job Entry (RJE)"),
        7 => Some("Echo"),
        9 => Some("Discard"),
        11 => Some("Active Users"),
        13 => Some("Daytime"),
        15 => Some("Not Used"),
        17 => Some("Quote of the Day (QOTD)"),
        19 => Some("Chargen"),
        20 => Some("FTP Data"),
        21 => Some("FTP Control"),
        22 => Some("SSH"),
        23 => Some("Telnet"),
        25 => Some("SMTP"),
        37 => Some("Time"),
        42 => Some("WINS"),
        43 => Some("WHOIS"),
        49 => Some("TACACS"),
        53 => Some("DNS"),
        67 => Some("DHCP Server"),
        68 => Some("DHCP Client"),
        69 => Some("TFTP"),
        79 => Some("Finger"),
        80 => Some("HTTP"),
        110 => Some("POP3"),
        119 => Some("NNTP"),
        123 => Some("NTP"),
        143 => Some("IMAP"),
        161 => Some("SNMP"),
        162 => Some("SNMP Trap"),
        194 => Some("IRC"),
        220 => Some("IMAP3"),
        443 => Some("HTTPS"),
        445 => Some("SMB"),
        464 => Some("Kerberos Change/Set Password"),
        514 => Some("Syslog"),
        515 => Some("LPD"),
        543 => Some("Klogin"),
        544 => Some("Kshell"),
        548 => Some("AFP"),
        587 => Some("SMTP Secure"),
        631 => Some("IPP"),
        993 => Some("IMAPS"),
        995 => Some("POP3S"),
        2049 => Some("NFS"),
        3306 => Some("MySQL"),
        3389 => Some("RDP (Remote Desktop Protocol)"),
        5432 => Some("PostgreSQL"),
        5900 => Some("VNC"),
        6379 => Some("Redis"),
        6660..=6669 => Some("IRC"),
        8080 => Some("HTTP Alternative"),
        8443 => Some("HTTPS Alternative"),
        8888 => Some("HTTP Alternative"),
        9090 => Some("Web Management"),
        10000 => Some("Webmin"),
        _ => Some("Reserved/Unassigned"),
    }
}



fn identify_udp_application(src_port: u16, dst_port: u16) -> Option<&'static str> {
    match dst_port {
        0 => Some("Reserved"),
        53 => Some("DNS"),
        67 => Some("DHCP Server"),
        68 => Some("DHCP Client"),
        69 => Some("TFTP"),
        123 => Some("NTP"),
        161 => Some("SNMP"),
        162 => Some("SNMP Trap"),
        514 => Some("Syslog"),
        1883 => Some("MQTT"),
        3333 => Some("Cassandra"),
        3702 => Some("WS-Discovery"),
        4500 => Some("IPsec NAT-T"),
        5353 => Some("mDNS"),
        5060 => Some("SIP"),
        5061 => Some("SIP Secure"),
        51413 => Some("BitTorrent"),
        18787 => Some("AVAHI"),
        8021 => Some("FTP-Proxy"),
        4242 => Some("Warcraft III"),
        28960 => Some("Call of Duty"),
        5222 => Some("XMPP"),
        5555 => Some("ADB (Android Debug Bridge)"),
        6666 => Some("IRC"),
        9119 => Some("Steam"),
        2049 => Some("NFS"),
        5355 => Some("LLMNR"),
        6101 => Some("Worms Armageddon"),
        6667 => Some("IRC"),
        9000 => Some("Sonos"),
        8080 => Some("HTTP Alternative"),
        9001 => Some("Tor"),
        1935 => Some("RTMP (Real-Time Messaging Protocol)"),
        554 => Some("RTSP (Real-Time Streaming Protocol)"),
        7070 => Some("Real-Time Streaming Protocol (RTSP)"),
        5004 => Some("RTP (Real-Time Transport Protocol)"),
        5005 => Some("RTCP (Real-Time Control Protocol)"),
        55443 => Some("WebRTC"),
        3434 => Some("MSN Messenger"),
        _ => Some("Reserved/Unassigned"),
    }
}




fn handle_ipv4_packets(packet: &Ipv4Packet, conn: &Connection, table_name: &str, timestamp: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("IPv4 Packet: {} -> {}; Protocol: {:?}", packet.get_source(), packet.get_destination(), get_next_level_protocol(&packet.get_next_level_protocol()));
    println!("{:?}",identify_application_layer_protocol(packet.packet(), packet.get_next_level_protocol()));
    let payload_raw = packet.payload().to_vec();
    let packet_data = PacketData {
        timestamp: timestamp.to_string(),
        packet_type: "IPv4".to_string(),
        source: packet.get_source().to_string(),
        destination: packet.get_destination().to_string(),
        protocol: Some(get_next_level_protocol(&packet.get_next_level_protocol()).to_string()),
        payload_base64: encode(&payload_raw),
        payload_hex: hex_encode(&payload_raw),
        payload_raw: payload_raw.clone(),
        payload_string: String::from_utf8_lossy(&payload_raw).to_string(),
        // applayer : format!("{:?}",identify_application_layer_protocol(packet.packet(), packet.get_next_level_protocol()))
    };

    // println!("{:?}",packet.);

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
            // packet_data.applayer
            //push cmt
            //m
        ],
    )?;
    Ok(())
}

fn handle_ipv6_packets(packet: &Ipv6Packet, conn: &Connection, table_name: &str, timestamp: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("IPv6 Packet: {} -> {}; Protocol: {:?}", packet.get_source(), packet.get_destination(), get_next_level_protocol(&packet.get_next_header()));
    let payload_raw = packet.payload().to_vec();
    let packet_data = PacketData {
        timestamp: timestamp.to_string(),
        packet_type: "IPv6".to_string(),
        source: packet.get_source().to_string(),
        destination: packet.get_destination().to_string(),
        protocol: Some(get_next_level_protocol(&packet.get_next_header()).to_string()),
        payload_base64: encode(&payload_raw),
        payload_hex: hex_encode(&payload_raw),
        payload_raw: payload_raw.clone(),
        payload_string: String::from_utf8_lossy(&payload_raw).to_string(),
        // applayer : format!("{:?}",identify_application_layer_protocol(packet.packet(), packet.get_next_header()))
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
            // packet_data.applayer
        ],
    )?;
    Ok(())
}

fn query_ip_stats(conn: &Connection, table_name: &str) -> Result<HashMap<String, IpStats>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT source, destination FROM {}",
        table_name
    ))?;

    let mut ip_stats: HashMap<String, IpStats> = HashMap::new();
    let rows = stmt.query_map(params![], |row| {
        let source: String = row.get(0)?;
        let destination: String = row.get(1)?;
        Ok((source, destination))
    })?;
    for row in rows {
        let (source, destination) = row?;
        
        ip_stats.entry(source)
            .and_modify(|stats| stats.source_count += 1)
            .or_insert(IpStats { source_count: 1, destination_count: 0 });

        ip_stats.entry(destination)
            .and_modify(|stats| stats.destination_count += 1)
            .or_insert(IpStats { source_count: 0, destination_count: 1 });
    }

    Ok(ip_stats)
}

fn query_packet_per_second(conn: &Connection, table_name: &str) -> Result<HashMap<String, u32>, rusqlite::Error> {
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
    let conn = Connection::open(RISHI_URL).map_err(|e| e.to_string())?;
    let ip_stats = query_ip_stats(&conn, table_name).map_err(|e| e.to_string())?;

    let formatted_ip_stats: Vec<serde_json::Value> = ip_stats.into_iter().map(|(ip, stats)| {
        json!({
            "IP": ip,
            "Source": stats.source_count,
            "Destination": stats.destination_count
        })
    }).collect();

    Ok(formatted_ip_stats)
}

#[tauri::command]
fn get_packet_per_second(table_name: &str) -> Result<Vec<serde_json::Value>, String> {
    let conn = Connection::open(RISHI_URL).map_err(|e| e.to_string())?;
    let packet_per_second = query_packet_per_second(&conn, table_name).map_err(|e| e.to_string())?;

    let mut formatted_packet_per_second: Vec<(String, u32)> = packet_per_second.into_iter().collect();
    formatted_packet_per_second.sort_by(|a, b| a.0.cmp(&b.0));

    let formatted_packet_per_second: Vec<serde_json::Value> = formatted_packet_per_second.into_iter().map(|(timestamp, count)| {
        json!({
            "timeStamp": timestamp,
            "traffic": count
        })
    }).collect();

    Ok(formatted_packet_per_second)
}

#[tauri::command]
fn get_packet_types(table_name: &str) -> Result<Vec<serde_json::Value>, String> {
    let conn = Connection::open(RISHI_URL).map_err(|e| e.to_string())?;
    let packet_types = query_packet_types(&conn, table_name).map_err(|e| e.to_string())?;
    
    let formatted_packet_types: Vec<serde_json::Value> = packet_types.into_iter().map(|(packet_type, count)| {
        json!({
            "type": packet_type,
            "count": count
        })
    }).collect();

    Ok(formatted_packet_types)
}

fn _llama_data_fetcher(
    table: &str,
    protocol: Option<&str>,
    source_ip: Option<&str>,
    destination_ip: Option<&str>
) -> Vec<PacketData> {
    let conn = Connection::open(RISHI_URL).unwrap();
    let mut query = format!("select * from {} WHERE 1=1",table);
    // if let Some(protocol) = protocol {
    //     query.push_str(&format!(" AND protocol = '{}'", protocol));
    // }
    if let Some(source_ip) = source_ip {
        query.push_str(&format!(" AND source = '{}'", source_ip));
    }
    if let Some(destination_ip) = destination_ip {
        query.push_str(&format!(" AND destination = '{}'", destination_ip));
    }
    let mut smt = conn.prepare(&query).unwrap();
    let result_iter = smt.query_map([], |row|{
        Ok(PacketData{
            // id                : row.get(0).unwrap(),
            timestamp         : row.get(1).unwrap(),
            packet_type       : row.get(2).unwrap(),
            source            : row.get(3).unwrap(),
            destination       : row.get(4).unwrap(),
            protocol          : row.get(5).unwrap(),
            payload_base64    : row.get(6).unwrap(),
            payload_hex       : row.get(7).unwrap(),
            payload_raw       : row.get(8).unwrap(),
          payload_string      : row.get(9).unwrap(),
        //   applayer            : row.get(10).unwrap()
        })
    }).unwrap();
    let mut res = Vec::new();
    for i in result_iter {
        res.push(i.unwrap());
    }
    res

}

// #[tauri::command]
// async fn handle_ollama(table: &str) -> Result<serde_json::Value, String> {
//     println!("table: {}", table);
//     let data = _llama_data_fetcher(table);
//     let formatted_data = data.iter()
//         .map(|packet| format!(
//             "Timestamp: {}, Packet Type: {}, Source: {}, Destination: {}, Protocol: {:?}, Payload (String): {}",
//             packet.timestamp,
//             packet.packet_type,
//             packet.source,
//             packet.destination,
//             packet.protocol,
//             packet.payload_string,
//             // packet.applayer
//         ))
//         .collect::<Vec<String>>()
//         .join("\n");
//     let prompt = format!(
//         "Analyze the Data like a senior data analyst:\n{}",
//         formatted_data
//     );
    
//     let client = Client::new();
//     let api_url = "http://localhost:11434/api/generate";
//     let response = client.post(api_url)
//         .json(&json!({
//             "model": "llama3.1",
//             "prompt": prompt,
//             "stream": false
//         }))
//         .send()
//         .await
//         .map_err(|e| e.to_string())?;

//     let response_text = response.text().await.map_err(|e| e.to_string())?;

//     println!("Response: {}", response_text);
//     // Return the response as a JSON object
//     Ok(json!({
//         "response": response_text
//     }))
// }



#[derive(Serialize)]
struct OllamaData {
    response: String,
}

async fn llama_data_fetcher_packets(
    table: &str,
    protocol: Option<&str>,
    source_ip: Option<&str>,
    destination_ip: Option<&str>,
) -> Result<Vec<PacketData>, String> {
    let conn = Connection::open(RISHI_URL).map_err(|e| e.to_string())?;
    
    let mut query = format!("SELECT * FROM {} WHERE 1=1", table);
    
    if let Some(protocol) = protocol {
        query.push_str(&format!(" AND protocol = '{}'", protocol));
    }
    if let Some(source_ip) = source_ip {
        query.push_str(&format!(" AND source = '{}'", source_ip));
    }
    if let Some(destination_ip) = destination_ip {
        query.push_str(&format!(" AND destination = '{}'", destination_ip));
    }
    
    query.push_str(" LIMIT 50 OFFSET 2");
    
    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let result_iter = stmt.query_map([], |row| {
        Ok(PacketData {
            // id                : row.get(0).unwrap(),
            timestamp         : row.get(1).unwrap(),
            packet_type       : row.get(2).unwrap(),
            source            : row.get(3).unwrap(),
            destination       : row.get(4).unwrap(),
            protocol          : row.get(5).unwrap(),
            payload_base64    : row.get(6).unwrap(),
            payload_hex       : row.get(7).unwrap(),
            payload_raw       : row.get(8).unwrap(),
            payload_string    : row.get(9).unwrap(),
            // applayer            : row.get(10).unwrap()
        })
    }).map_err(|e| e.to_string())?;
    
    let mut res = Vec::new();
    for i in result_iter {
        res.push(i.map_err(|e| e.to_string())?);
    }
    
    Ok(res)
}

#[tauri::command]
async fn list_src_ips(table: &str) -> Result<Vec<String>, String> {
    print!("funs invoked");
    let conn = Connection::open(RISHI_URL).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(&format!(
        "SELECT DISTINCT source FROM {}",
        table
    )).map_err(|e| e.to_string())?;
    
    let result_iter = stmt.query_map([], |row| {
        row.get(0)
    }).map_err(|e| e.to_string())?;
    
    let mut res = Vec::new();
    for i in result_iter {
        res.push(i.map_err(|e| e.to_string())?);
    }

    println!("{:?}",res);
    
    Ok(res)
}

#[tauri::command]
async fn list_dst_ips(table: &str) -> Result<Vec<String>, String> {
    let conn = Connection::open(RISHI_URL).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(&format!(
        "SELECT DISTINCT destination FROM {}",
        table
    )).map_err(|e| e.to_string())?;
    
    let result_iter = stmt.query_map([], |row| {
        row.get(0)
    }).map_err(|e| e.to_string())?;
    
    let mut res = Vec::new();
    for i in result_iter {
        res.push(i.map_err(|e| e.to_string())?);
    }
    
    Ok(res)
}

#[tauri::command]
async fn list_protocol(table: &str) -> Result<Vec<String>, String> {
    let conn = Connection::open(RISHI_URL).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(&format!(
        "SELECT DISTINCT protocol FROM {}",
        table
    )).map_err(|e| e.to_string())?;
    
    let result_iter = stmt.query_map([], |row| {
        row.get(0)
    }).map_err(|e| e.to_string())?;
    
    let mut res = Vec::new();
    for i in result_iter {
        res.push(i.map_err(|e| e.to_string())?);
    }
    
    Ok(res)
}

#[tauri::command]
async fn ollama_frontend(
    table: String,
    protocol: Option<String>,
    sourceIp: String,
    destinationIp: String,
) -> Result<serde_json::Value, String> {
    print!("table: {}", table);
    let protocol_ref = protocol.as_deref();
    // match llama_data_fetcher_packets(&table,protocol_ref, Some(&sourceIp), Some(&destinationIp)).await {
        // Ok(data) => {
            let data = _llama_data_fetcher(table.as_str(), protocol_ref, Some(&sourceIp), Some(&destinationIp));
            print!("Data is received here in side the handle_ollama_packets");
            let formatted_data = data.iter()
                .map(|packet| format!(
                    "Timestamp: {}, Packet Type: {}, Source: {}, Destination: {}, Protocol: {:?}, Payload (String): {}",
                    packet.timestamp,
                    packet.packet_type,
                    packet.source,
                    packet.destination,
                    packet.protocol,
                    packet.payload_string,
                ))
                .collect::<Vec<String>>()
                .join("\n");
            // let formatted_data = _llama_data_fetcher(&table);

            let prompt = format!(
                // "Analyze the Data like a senior data analyst:\n{}",
                "Analyze the RAW network packets and proivde with any insights as possible like an network engineer \n{}",
                formatted_data
            );
            
            println!("{}",formatted_data);

            let client = Client::new();
            let api_url = "http://localhost:11434/api/generate";
            let response = client.post(api_url)
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
            // Return the response as a JSON object
            Ok(json!({
                "response": response_text
            }))
        // },
        // Err(e) => Err(format!("Error fetching data: {}", e)),
    }




#[tokio::main]
async fn main() {
    Builder::default()
        .manage(Arc::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![start_packet_sniffer, stop_packet_sniffer, list_names,list_src_ips,list_dst_ips,list_protocol, list_interfacce, get_table_data, get_packet_types,get_packet_per_second, get_ip_stats,ollama_frontend])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}