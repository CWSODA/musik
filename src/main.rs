use musik::coms;
use musik::log;
use std::io;
use std::path::PathBuf;
use std::str::SplitWhitespace;

// /Users/admin/Downloads
// /Users/admin/Desktop/wow

fn main() {
    let mut added_items: Vec<PathBuf> = Vec::new();

    println!(">>> Welcome to bad file manager!");

    let mut input = String::new();
    loop{
        input.clear();
        println!("\n>>> Please enter command:");
        io::stdin().read_line(&mut input).expect("cmd broken");
        let mut inputs = input.split_whitespace();

        if let Some(com) = inputs.next(){
            match com{
                "run" => run(inputs, &mut added_items),
                "clear_log" => clear_log(inputs),
                "undo" => undo(inputs, &mut added_items),
                "show_added" => show_added(inputs, &added_items),
                "quit" => if quit(inputs) { return },
                _ => println!("> Invalid command or too many args, try again"),
            }
        }
        else{
            println!("> No command given, please try again");
            continue;
        }
    }
}

fn run(arg: SplitWhitespace<>, added_items: &mut Vec<PathBuf>){
    match args(arg){
        ArgNum::Nein => coms::run(None, None, added_items),
        ArgNum::Uno(arg) => {
            if arg == "-p" {
                coms::run_with_path(added_items);
                return
            }

            println!("> Argument not valid");
        },
        ArgNum::Mas => println!("> run only takes none or one argument"),
    }
}

fn show_added(arg: SplitWhitespace<>, added_items: &Vec<PathBuf>){
    match args(arg) {
        ArgNum::Nein => coms::show_added(None, added_items),
        ArgNum::Uno(count) => {
            if let Ok(count) = count.parse::<i32>() {
                if count <= 0 {
                    println!("> Only non-zero and non-negative numbers allowed");
                    return;
                }

                coms::show_added(Some(count as u16), added_items);
                return;
            }

            println!("> Argument needs to be an integer");
        },
        ArgNum::Mas => println!("> show_added only takes one argument"),
    }
}

fn clear_log(arg: SplitWhitespace<>){
    if args(arg) == ArgNum::Nein {
        log::clear_log();
        println!("> Logs cleared");
        return;
    }

    println!("> clear_log does not accept any arguments");
}

fn undo(arg: SplitWhitespace<>, added_items: &mut Vec<PathBuf>){
    if args(arg) == ArgNum::Nein {
        coms::undo(added_items);
        return;
    }

    println!("> undo does not accept any arguments");
}

fn quit(arg: SplitWhitespace) -> bool {
    if args(arg) == ArgNum::Nein { return true }

    println!("> quit does not accept any arguments");
    false
}

fn args(mut arg: SplitWhitespace<>) -> ArgNum {
    match (arg.next(), arg.next()){
        (Some(arg), None) => ArgNum::Uno(arg),
        (Some(_), Some(_)) => ArgNum::Mas,
        _ => ArgNum::Nein,
    }
}

#[derive(PartialEq)]
enum ArgNum<'a>{
    Nein,
    Uno(&'a str),
    Mas,
}