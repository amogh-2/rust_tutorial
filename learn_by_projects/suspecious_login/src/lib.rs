use std::collections::HashMap;

pub enum Status{
    Success,
    Failed,
}

pub struct LoginAttempt{
    user: String,
    ip: String,
    status: Status,
}

impl LoginAttempt{
    pub fn new(user: &str, ip: &str, status:Status)->Self{
        Self{
            user:user.to_string(),
            ip:ip.to_string(),
            status,
        }
    }
}

pub fn check_count(attempts:&[LoginAttempt]){
    let mut failed_counts = HashMap::new();
    for attempt in attempts{
        match attempt.status {
            Status::Failed => { 
                let count = failed_counts.entry(attempt.ip.clone()).or_insert(0);
                *count += 1;
             }
            _ => {}
        }
    }
    for (ip, count) in failed_counts.iter() {
        if *count >= 3 {
            println!("Suspicious IP: {} → {} failed attempts", ip, count);
        } else {
            println!("IP: {} → {} failed attempts", ip, count);
        }
    }

}





