
pub struct LogAnalyzer{
    pub logs: Vec<String>,
    pub info: u32,
    pub warn: u32,
    pub error: u32,
}

impl LogAnalyzer {

    pub fn new(logs: Vec<String>) -> Self {
        Self {
            logs,
            info: 0,
            warn: 0,
            error: 0,
        }
    }

    pub fn analyze(&mut self) {

        for log in &self.logs {

            if log.starts_with("INFO") {
                self.info += 1;
            }

            else if log.starts_with("WARN") {
                self.warn += 1;
            }

            else if log.starts_with("ERROR") {
                self.error += 1;
            }
        }
    }

    pub fn print_report(&self) {
        println!("INFO: {}", self.info);
        println!("WARN: {}", self.warn);
        println!("ERROR: {}", self.error);
    }
}