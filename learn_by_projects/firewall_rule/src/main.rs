use firewall_rule::FireWallRule;

fn main(){
    let packet= Packet{
        src_ip= "192.168.1.10".to_string(),
        dst_ip="10.0.0.5".to_string(),
        port: 22,
    };
    
    let rule1 = FirewallRule::new("192.168.1.10", 22, false); // block
    let rule2 = FirewallRule::new("192.168.1.20", 80, true); //allow

    rule1.display();
    rule2.display();

    println!("Packet allowed? {}",rule1.is_allowed(&packet));
}

