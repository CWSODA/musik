use musik::coms::*;
use musik::log::*;
use std::io;

// /Users/admin/Downloads
// /Users/admin/Desktop/wow

fn main() {
    let mut added_items = Vec::new();

    println!(">>> Welcome to bad file manager!");

    let mut input = String::new();
    loop{
        input.clear();
        println!("\n>>> Please enter command:");
        io::stdin().read_line(&mut input).expect("cmd broken");
        let input = input.trim();

        match &input[..]{
            "run" => run(None, None, &mut added_items),
            "run_with_path" => run_with_path(&mut added_items),
            "undo" => undo(&mut added_items),
            "quit" => return,
            "show_added" => show_added(&added_items),
            "clear_log" => clear_log(),
            _ => println!("invalid command, try again")
        }
    }

}
