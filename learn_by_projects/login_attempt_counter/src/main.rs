use std::collections::HashMap;

fn main() {
    let logins = vec![
        "alice",
        "bob",
        "alice",
        "eve",
        "bob",
        "alice",
    ];

    let mut attempts = HashMap::new();

    for user in logins {
        *attempts.entry(user).or_insert(0) += 1;
    }

    for (user, count) in attempts {
        if count >2{
            panic!("{} is fishy",user);
        }
        println!("{} -> {} attempts", user, count);
    }
}