use std::path::PathBuf;
use std::collections::HashMap;
use std::time::Instant;
use walkdir::WalkDir;


const BYTES_PER_GB: f64 = 1024.0 * 1024.0 * 1024.0;
fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / BYTES_PER_GB
}

fn seconds_to_min_sec(total_seconds: f64) -> (u64, u64) {
    let minutes = total_seconds / 60.0;
    let seconds = total_seconds % 60.0;
    (minutes as u64, seconds as u64)
}

#[derive(Debug)]
struct DirEntry {
    path: PathBuf,
    size: u64,
    file_count: u64,

}
fn main() {
    println!("disk-analyzer starting...");
    println!("starting timer...");
    let now = Instant::now();
    let root_dir = "/System/Volumes/Data";

    let mut disk_stats: HashMap<PathBuf, DirEntry> = HashMap::new();

    for entry in WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(parent) = entry.path().parent() {
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);

                disk_stats.entry(parent.to_path_buf()).and_modify(|e| {
                    e.size += size;
                    e.file_count += 1;
                })
                    .or_insert(DirEntry {
                        path: parent.to_path_buf(),
                        size,
                        file_count: 1,
                    });
            }
        }
    }

    let mut disk_stats_vec: Vec<DirEntry> = disk_stats.into_values().collect();
    disk_stats_vec.sort_by_key(|k| std::cmp::Reverse(k.size));

    for entry in disk_stats_vec.iter().take(20) {
        println!("Directory: {}\tSize: {}\tNum Files: {}", entry.path.display(), bytes_to_gb(entry.size), entry.file_count);
    }
    let duration = now.elapsed().as_secs_f64();
    let (mins, secs) = seconds_to_min_sec(duration);
    println!("disk-analyzer stopping...");
    println!("Minutes elapsed: {}:{:02}", mins, secs);
}
