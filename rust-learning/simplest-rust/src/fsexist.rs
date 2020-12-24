use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    println!("{}", Path::new("fsexist.rs1").exists());
    let attr = fs::metadata("fsexist.rs1")?;
    // inspect attr ...
    Ok(())
}