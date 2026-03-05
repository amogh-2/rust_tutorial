#[derive(Debug)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
}

#[derive(Debug)]
pub struct Packet {
    pub source_ip: String,
    pub destination_ip: String,
    pub protocol: Protocol,
    pub port: u16,
    pub length: u16,
}

pub fn parse_protocol(proto:&str)-> Protocol{
    match proto{
        "TCP"=> Protocol::TCP,
        "UDP"=> Protocol::UDP,
        "ICMP" => Protocol::ICMP,
        _=> panic!("Unknown Protocol"),
    }
}

pub fn parse_packet(line: &str)-> Packet{
    let parts: Vec<&str> = line.split_whitespace().collect();

    Packet{
        source_ip: parts[0].to_string(),
        destination_ip: parts[2].to_string(),
        protocol: parse_protocol(parts[3]),
        port: parts[4].parse().unwrap(),
        length: parts[5].parse().unwrap(),
    } 
}
