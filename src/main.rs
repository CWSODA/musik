use musik::fil::*;
use musik::log::*;
use std::path::{Path, PathBuf};
use std::io;

fn main() {
    let Ok(mut logger) = Logger::new(Path::new("")) else{
        eprint!("Error: cannot get log file!");
        return;
    };

    let mut added_items = Vec::new();

    println!(">>> Welcome to bad file manager!");

    loop{
        let mut input = String::new();
        println!("\n>>> Please enter command:");
        io::stdin().read_line(&mut input).expect("cmd broken");
        let input = input.trim();

        match &input[..]{
            "run" => run(&mut added_items, &mut logger),
            "undo" => undo(&mut added_items, &mut logger),
            "quit" => return,
            _ => println!("invalid command, try again")
        }
    }

}

fn run(added_items: &mut Vec<PathBuf>, logger: &mut Logger){
    let src_dir = Path::new("/Users/admin/rust_projects/musik/origin");
    let target_dir = Path::new("/Users/admin/rust_projects/musik/future");

    let tgt_ext = vec!["txt", "mp3"];

    if let Err(msg) = copy_over(&src_dir, &target_dir, &tgt_ext, added_items){
        eprintln!("!!! Error: {msg}")
    }

    let mut log_this = String::from("\nItems added:\n");
    for item in added_items{
        log_this.push_str(item.to_str().unwrap());
        log_this += "\n";
    }
    logger.log(&log_this);
}

fn undo(targets: &mut Vec<PathBuf>, logger: &mut Logger){
    let mut log_this = String::from("\nItems to be deleted:\n");
    for item in targets.iter(){
        log_this.push_str(item.to_str().unwrap());
        log_this += "\n";
    }
    logger.log(&log_this);

    if let Err(msg) = delete_files(targets){
        eprint!("!!! Error: {msg}")
    }
}