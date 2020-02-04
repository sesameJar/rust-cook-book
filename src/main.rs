#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate tar;

use flate2::read::GzDecoder;
use std::fs::File;

use std::path::PathBuf;
use tar::Archive;

error_chain! {
    foreign_links {
      Io(std::io::Error);
      StripPrefixError(::std::path::StripPrefixError);
    }
}

fn main() -> Result<()> {
    let file = File::open("archive.tar.gz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "backup/logs";

    let ent = archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.strip_prefix(prefix)?.to_owned();
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(())
}
