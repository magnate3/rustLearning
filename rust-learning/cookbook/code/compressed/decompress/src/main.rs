//Decompress (GzDecoder) and extract (Archive::unpack) all files from a compressed tarball named archive.tar.gz located in the current working directory to the same location.
extern crate flate2;
extern crate tar;

use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() -> Result<(), std::io::Error> {
    let path = "archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

