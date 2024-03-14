use musik::fil::*;
use std::path::{Path, PathBuf};
use std::io;

fn main() {
    let mut added_items = Vec::new();

    println!(">>> Welcome to bad file manager!");
    println!(">>> Please enter command: \n");

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("cmd broken");

        match &input[..]{
            "run" => run(&mut added_items),
            "undo" => undo(&mut added_items),
            _ => println!("invalid command")
        }
    }

}

fn run(added_items: &mut Vec<PathBuf>){
    let src_dir = Path::new("/Users/admin/rust_projects/musik/origin");
    let target_dir = Path::new("/Users/admin/rust_projects/musik/future");

    let tgt_ext = vec!["txt", "mp3"];

    if let Err(msg) = copy_over(&src_dir, &target_dir, &tgt_ext, added_items){
        eprintln!("!!! Error: {msg}")
    }

    dbg!(&added_items);
}

fn undo(targets: &mut Vec<PathBuf>){
    if let Err(msg) = delete_files(targets){
        eprint!("!!! Error: {msg}")
    }
}