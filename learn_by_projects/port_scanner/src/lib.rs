use std::time::Duration;
use std::net::TcpStream;

#[derive(Debug)]
pub struct ScanTarget{
    pub ip: String,     
    pub ports: Vec<u16>,
}

#[derive(Debug)]
pub struct ScanResult{
    pub port: u16,
    pub open: bool,
}

impl ScanTarget{
    //scan a single port
    pub fn scan_port(&self, port: u16)-> ScanResult{
        let timeout=Duration::from_secs(1);
        let open=TcpStream::connect_timeout(
            &format!("{}:{}",self.ip,port).parse().unwrap(),
            timeout
        ).is_ok();
        
        ScanResult{port,open}
    }
    //Scan all ports in self.ports
    pub fn scan_all(&self)-> Vec<ScanResult>{
        self.ports.iter().map(|&port| self.scan_port(port)).collect()
    }
}

