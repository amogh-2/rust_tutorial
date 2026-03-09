pub enum AlertLevel{
    Low,
    Medium,
    High,
}

pub struct Alert{
    src_ip:String,
    message: String,
    pub level: AlertLevel,
    count: u32,
}

impl Alert{
    pub fn new(src: &str, message: &str, level:AlertLevel)->Self{
        Self{
            src_ip: src.to_string(),
            message:message.to_string(),
            level,
            count:1,
        }
    }
    pub fn increament(&mut self){
        self.count+=1;
    }
    pub fn display(&self){
        println!("Source: {}",self.src_ip);
        println!("Message: {}",self.message);
        println!("Count: {}",self.count);
    }
    // fn escalate(&self)->AlertLevel{
    //     if self.count>=3{
    //         AlertLevel::Medium
    //     }
    //     if self.count>=5{
    //         AlertLevel::High
    //     }
    // }

}

impl AlertLevel{

    pub fn severity(&self)-> &str{
        match self{
            AlertLevel::Low => "LOW",
            AlertLevel::Medium => "MEDIUM",
            AlertLevel::High => "HIGH",
        }
    }
}