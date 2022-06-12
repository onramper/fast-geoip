use std::io;

pub fn get(folder: &str, ipv4: &str) -> io::Result<()> {
    println!("folder: {folder}, ip: {ipv4}");

    Ok(())
}
