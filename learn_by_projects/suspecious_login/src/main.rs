use suspecious_login::{LoginAttempt,check_count,Status};

fn main(){
    let attempts= vec![
        LoginAttempt::new("emp","192.168.1.10",Status::Failed),
        LoginAttempt::new("purr","192.168.1.10",Status::Failed),
        LoginAttempt::new("avaya","10.10.10.1",Status::Success),
        LoginAttempt::new("amogh","8.8.8.8",Status::Success),
    ];

    check_count(&attempts);
}