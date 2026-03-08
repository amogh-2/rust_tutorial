pub struct Packet{
    pub src_ip: String,
    pub dst_ip: String,
    pub port: u16,
}

pub struct FireWallRule{
    ip:String,
    port:u16,
    allow:bool,
}

impl FireWallRule{
    pub fn new(ip: &str, port: u16, allow:bool)->Self{
        Self{
            ip: ip.to_string(),
            port,
            allow,
        }
    }
    fn matches(&self, packet: &Packet)-> bool{
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
    pub fn display(&self){
        let status = if self.allow{"ALLOW"} else {"BLOCK"};
        println!("{}:{} -> {}",self.ip, self.port, status);
    }
}