use Intrusion_alert_system::{Alert,AlertLevel};

fn main(){

    let mut alert = Alert::new(
        "192.168.1.5",
        "Port scanning detected",
        AlertLevel::High,
    );

    alert.display();

    alert.increament();
    
    println!("Severity: {}", alert.level.severity());
    
    alert.display();
}