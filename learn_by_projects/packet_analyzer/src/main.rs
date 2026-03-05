use packet_analyzer::{parse_packet,Packet};
use std::io;
use std::thread;
use std::time::Duration;

fn main(){
    // let logs= vec![
    //     "192.168.1.10 -> 10.0.0.5 TCP 443 1500",
    //     "192.168.1.12 -> 10.0.0.5 UDP 53 200",
    //     "192.168.1.15 -> 8.8.8.8 TCP 80 900",
    // ];

    // let mut packets = Vec::new();
    // for log in logs{
    //     let packet = parse_packet(log);
    //     packets.push(packet);
    // }
    let mut packets: Vec<Packet> = Vec::new();
    println!("Enter Packet logs: (Enter 'exit' to stop)");
    println!("Enter in the following format:\nSourceiP DestinationIP Protocol Port Duration");
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "exit"{
            println!("Exiting..");
            thread::sleep(Duration::from_secs(2));
            break;
        }
        let packet = parse_packet(input);
        packets.push(packet);
    }   
    
    for packet in &packets{
        println!("{:?}", packet);
    }
}


