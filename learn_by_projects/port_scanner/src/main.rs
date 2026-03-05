use port_scanner::ScanTarget;

fn main(){
    let target = ScanTarget{
        ip: "127.0.0.1".to_string(),
        ports: vec![22,80,443,8080],
    };
    let results = target.scan_all();

    println!("scan results for {}",target.ip);
    for res in results{
        if res.open{
            println!("Port {}: OPEN", res.port);
        }
        else{
            println!("Port {}: CLOSED", res.port);
        }
    }
}