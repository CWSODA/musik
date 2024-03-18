use musik::fil::*;
use musik::log::*;
use std::path::{Path, PathBuf};
use std::io;

fn main() {
    let mut added_items = Vec::new();

    println!(">>> Welcome to bad file manager!");

    loop{
        let mut input = String::new();
        println!("\n>>> Please enter command:");
        io::stdin().read_line(&mut input).expect("cmd broken");
        let input = input.trim();

        match &input[..]{
            "run" => run(&mut added_items),
            "undo" => undo(&mut added_items),
            "quit" => return,
            "clear_log" => clear_log(),
            _ => println!("invalid command, try again")
        }
    }

}

fn run(added_items: &mut Vec<PathBuf>){
    let src_dir = Path::new("/Users/admin/rust_projects/musik/origin");
    let target_dir = Path::new("/Users/admin/rust_projects/musik/future");
    let tgt_ext = vec!["txt", "mp3"];

    // create initialization log
    let mut msg = format!("Running for paths:
    >>> {}
    >>> {}
    With extensions: ", 
    src_dir.display(), target_dir.display());
    for ext in &tgt_ext {
        msg.push_str(&format!("{ext} "));
    }

    log(&msg);

    if let Err(msg) = copy_over(&src_dir, &target_dir, &tgt_ext, added_items){
        eprintln!("!!! Error: {msg}")
    }
    else{
        println!("Successfully ran!!!");
    }
}

fn undo(targets: &mut Vec<PathBuf>){
    if let Err(msg) = delete_files(targets){
        eprint!("!!! Error: {msg}")
    }
    else{
        println!("Successfully removed added items!!!");
    }
}