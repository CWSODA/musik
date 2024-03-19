use std::path::{Path, PathBuf};
use super::log::*;
use super::fil::*;
use std::io;

pub fn run_with_path(added_items: &mut Vec<PathBuf>){
    let Some(src) = get_dir("\nPlease enter SOURCE directory path:")
    else{ return };
    let Some(target) = get_dir("\nPlease enter TARGET directory path:")
    else{ return };
    
    run(Some(&src), Some(&target), added_items);
}

fn get_dir(msg: &str) -> Option<PathBuf>{
    let mut input = String::new();
    loop{
        input.clear();
        println!("{msg}");
        io::stdin().read_line(&mut input).expect("no msg");
        let input = input.trim();
        if input == "quit"{ return None}

        let dir = Path::new(input);
        if dir.exists(){ return Some(dir.to_path_buf()) }
        else{
            println!("Cannot find path, try again!");
        }
    }

}

pub fn run(src_dir: Option<&Path>, tgt_dir: Option<&Path>, added_items: &mut Vec<PathBuf>){
    let timer = std::time::Instant::now();

    let src_dir = match src_dir{
        Some(src) => src,
        None => Path::new("/Users/admin/rust_projects/musik/origin"),
    };
    let tgt_dir = match tgt_dir{
        Some(target) => target,
        None => Path::new("/Users/admin/rust_projects/musik/future"),
    };
    let tgt_ext = vec!["txt", "mp3"];

    // create initialization log
    let mut msg = format!(
"Running for paths:
    >>> {}
    >>> {}
    With extensions: ", 
    src_dir.display(), tgt_dir.display());
    for ext in &tgt_ext {
        msg.push_str(&format!("{ext} "));
    }

    log(&msg);

    if let Err(msg) = copy_over(&src_dir, &tgt_dir, &tgt_ext, added_items){
        print_and_log_err(msg);
    }
    else{
        print_and_log(&format!("Successfully ran in {}ms!!!", timer.elapsed().as_millis()) );
    }
}

pub fn undo(targets: &mut Vec<PathBuf>){
    let timer = std::time::Instant::now();

    if targets.is_empty(){ 
        println!("Nothing to undo!");
        return;
    }

    log("Started undo: ");
    if let Err(msg) = delete_files(targets){
        print_and_log_err(msg);
    }
    else{
        print_and_log(&format!("Successfully removed added items in {}ms!!!", timer.elapsed().as_millis()) );
    }
}

pub fn show_added(items: &Vec<PathBuf>){
    if items.len() == 0 {
        println!("No added items");
        return;
    }
    let mut count = 0;

    for item in items{
        println!(">>> {}", item.display());
        count += 1;

        if count >= 10{
            println!(">>> ... (see log for more)");
            return;
        }
    }
}

fn print_and_log(msg: &str){
    println!("{msg}");
    log(msg);
}

fn print_and_log_err(msg: &str){
    print_and_log(&format!("!!! Error: {msg}"));
}