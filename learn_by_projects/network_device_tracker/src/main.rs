struct NetworkDevice {
    name: String,
    ip: String,
    online: bool,
    connected_domains: Vec<String>, 
}

fn main(){
    let device= NetworkDevice{
        name:String::from("Router01"),
        ip:String::from("192.168.0.1"),
        online: true,
        connected_domains: vec![String::from("example.com"), String::from("google.com")], 
    };
    
    let mut switch = NetworkDevice {
        name: String::from("Switch01"),
        ip: String::from("192.168.1.2"),
        online: false,
        connected_domains: vec![
            String::from("intranet.local"),
        ],
    };

    switch.connected_domains.push(String::from("rust-lang.org"));
    println!("Device: {}", device.name);
    println!("Status: {} with ip_address: {}",if device.online{"online"} else {"offline"},device.ip);
}