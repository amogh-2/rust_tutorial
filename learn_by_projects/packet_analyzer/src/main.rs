use packet_analyzer::parse_packet;

fn main(){
    let logs= vec![
        "192.168.1.10 -> 10.0.0.5 TCP 443 1500",
        "192.168.1.12 -> 10.0.0.5 UDP 53 200",
        "192.168.1.15 -> 8.8.8.8 TCP 80 900",
    ];
    let mut packets = Vec::new();
    for log in logs{
        let packet = parse_packet(log);
        packets.push(packet);
    }
    for packet in &packets{
        println!("{:?}", packet);
    }
}


