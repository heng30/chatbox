use std::fs;
use std::io;

pub fn dir_size(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut total_size: u64 = 0;
    let metadata = fs::metadata(path)?;

    if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            if let Ok(entry) = entry {
                let size = entry.metadata()?.len();
                total_size += size;
            }
        }
    } else {
        total_size += metadata.len();
    }

    Ok(format!("{:.2}M", total_size as f64 / 1024.0 / 1024.0))
}

pub fn remove_dir_files(path: &str) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}
