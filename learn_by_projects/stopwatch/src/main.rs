use std::io;
use std::time::Instant;

fn main() {
    println!("Press Enter to start time!!");
    let mut inpt=String::new();
    io::stdin().read_line(&mut inpt).unwrap();
    let start=Instant::now();
    println!("The stopwatch started. Press Enter to stop time.");
    inpt.clear();  // in this case inpt.clear() isn't really needed but it's safe to. Also read_line() doesn't overwrite exisiting but is concatenated to existing value.
    io::stdin().read_line(&mut inpt).unwrap();
    let duration=start.elapsed();
    println!("Total time elapsed: {:.2?}", duration);
}
