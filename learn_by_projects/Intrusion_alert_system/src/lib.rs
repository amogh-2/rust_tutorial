pub enum AlertLevel{
    Low,
    Medium,
    High,
}

pub struct Alert{
    src_ip:String,
    message: String,
    level: AlertLevel,
    count: u32,
}

impl Alret{
    fn new(src: &str, message: &str, level:AlertLevel)->Self{
        Self{
            src_ip: src.to_string(),
            message:message.to_string(),
            level,
            count:1,
        }
    }
    fn increament(&mut self){
        self.count+=1;
    }
    fn display(&self){
        println!("Source: {}",self.src_ip);
        println!("Message: {}",self.message);
        println!("Count: {}",self.countp);
    }


}