use std::fs;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Read file from preopened dir /data/input.txt and print it
    let content = fs::read_to_string("/data/input.txt")?;
    println!("WASI read /data/input.txt:\n{}", content);

    // Also print an environment variable if present
    if let Ok(v) = std::env::var("WASI_DEMO_ENV") {
        println!("WASI_DEMO_ENV = {}", v);
    } else {
        println!("WASI_DEMO_ENV not set");
    }
    Ok(())
}
