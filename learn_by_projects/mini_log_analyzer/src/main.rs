use mini_log_analyzer::LogAnalyzer;

fn main() {

    let logs = vec![
        "INFO User logged in".to_string(),
        "ERROR Database connection failed".to_string(),
        "INFO Request processed".to_string(),
        "WARN Disk space low".to_string(),
        "ERROR Timeout occurred".to_string(),
    ];

    let mut analyzer = LogAnalyzer::new(logs);

    analyzer.analyze();

    analyzer.print_report();
}