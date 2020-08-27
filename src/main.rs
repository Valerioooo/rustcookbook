use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("ola.txt")?;
    file.write_all("adeus, world!".as_bytes())?;
    Ok(())
}
