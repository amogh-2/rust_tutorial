struct Packet{
    src_ip: String,
    dst_ip: String,
    port: u16,
}

struct FireWallRule{
    ip:String,
    port:u16,
    allow:bool,
}

impl FireWallRule{
    pub fn new(ip: &str, port:&u16. allow:bool)->Self{
        Self{
            ip: ip.to_string(),
            port,
            allow,
        }
    }
    pub fn matches(&self, packet: &Packet)-> bool{
        self.ip == packet.src_ip && self.port == packet.port
    }
    pub fn is_allowed(&self, packet: &Packet)-> bool{
        if self.matches(packet){
            self.allow
        }
        else{
            true
        }
    }
    fn display(&self){
        let statuts = if self.allow{"ALLOW"} else {|"BLOCK"};
        println!("{}:{} -> {}",self.ip, self.port, status);
    }
}