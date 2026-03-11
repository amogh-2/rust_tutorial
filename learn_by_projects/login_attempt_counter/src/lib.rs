// use std::collections::HashMap;

// let mut attempts = HashMap::new();

// for user in logins{
//     *attempts.entry(user).or_insert(0)+=1;
// }

// for (user,count) in attempts{
//     println!("{} -> {} attempts",user,count);
// }

// use std::collections::HashMap;

// fn main() {
//     let logins = vec![
//         "alice",
//         "bob",
//         "alice",
//         "eve",
//         "bob",
//         "alice",
//     ];

//     let mut attempts = HashMap::new();

//     for user in logins {
//         *attempts.entry(user).or_insert(0) += 1;
//     }

//     for (user, count) in attempts {
//         println!("{} -> {} attempts", user, count);
//     }
// }