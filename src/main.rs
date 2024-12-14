use std::fs;

fn main() {
    println!("disk-analyzer starting...");
    let read_dir_result= fs::read_dir("/Users/deepwater/code");
    let read_dir = match read_dir_result {
        Ok(read_dir) => read_dir,
        Err(error) => panic!("Error while reading directory: {:?}", error),
    };

    for dir_entry_result in read_dir {
        let dir_entry = match dir_entry_result {
            Ok(dir_entry) => dir_entry,
            Err(error) => panic!("Error while reading directory entry: {:?}", error),
        };
        println!("{:?}", dir_entry.file_name());
    }

    println!("disk-analyzer stopping...");
}
