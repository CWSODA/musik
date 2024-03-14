use musik::fil::*;
use std::path::Path;

fn main() {
    
    println!(">>> Welcome to bad file manager!");

    let mut added_items = Vec::new();

    let src_dir = Path::new("/Users/admin/rust_projects/musik/origin");
    let target_dir = Path::new("/Users/admin/rust_projects/musik/future");

    let tgt_ext = vec!["txt", "mp3"];

    match copy_over(&src_dir, &target_dir, &tgt_ext, &mut added_items){
        Ok(_) => {},
        Err(msg) => eprintln!("!!! Error: {msg}")
    }

    dbg!(&added_items);
}
