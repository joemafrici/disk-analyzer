use std::fs;
use walkdir::WalkDir;

const BYTES_PER_GB: f64 = 1024.0 * 1024.0 * 1024.0;
fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / BYTES_PER_GB
}

fn main() {
    println!("disk-analyzer starting...");
    let root_dir = "/Users/deepwater/code";
    let mut size: u64 = 0;
    let mut num_files = 0;

    for entry in WalkDir::new(root_dir) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                println!("Error reading directory: {:?}", error);
                continue
            },
        };
        if entry.file_type().is_file() {
            match entry.metadata() {
                Ok(metadata) => {
                    size += metadata.len();
                    num_files += 1;
                },
                Err(error) => {
                    println!("Error reading metadata: {:?}", error);
                    continue
                }
            }
        }
    }

    println!("directory: {} size (GB): {} number of files: {}", root_dir, size, num_files);

    println!("disk-analyzer stopping...");
}
