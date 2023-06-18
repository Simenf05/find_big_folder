
use std::fs;
use std::path::Path;
use std::time::{Instant};


fn check_file(path: &Path) -> u64 {
    fs::metadata(path).unwrap().len()
}

fn check_dir(path: &Path) -> u64 {

    let mut size: u64 = 0;
    let dir_info = fs::read_dir(path).unwrap();

    
    for entry in dir_info {
        
        let pathbuf = &entry.unwrap().path();
        let path: &Path = Path::new(pathbuf);

        let isdir: bool = check_path(path);
        
        if isdir {
            size += check_dir(path);
        }
        else {
            size += check_file(path)
        }
    }
    
    size
}


fn check_path(path: &Path) -> bool {
    let inspecting = fs::metadata(path).unwrap();
    inspecting.is_dir()
}


fn main() {

    let path: &Path = Path::new("../../");
    
    let start_time = Instant::now();

    let isdir: bool = check_path(path);
    let result: u64;

    if isdir {
        result = check_dir(path);
    }
    else {
        result = check_file(path);
    }

    let elapsed_time = start_time.elapsed();

    println!("{result} bytes and it took {elapsed_time:?}")
}
