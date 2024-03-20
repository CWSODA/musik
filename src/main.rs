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
        let mut input_words = input.split_whitespace();

        if let Some(com) = input_words.next(){
            match com{
                "run" => run(input_words, &mut added_items),
                "clear_log" => clear_log(input_words),
                "undo" => undo(input_words, &mut added_items),
                "show_added" => show_added(input_words, &added_items),
                "quit" => if quit(input_words) { return },
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
    match get_arg_num(arg){
        ArgNum::Nein => coms::run(None, None, added_items, false),
        ArgNum::Uno(arg) => {
            if let Some(args) = letter_parse(arg.chars().collect()){
                let mut is_copy_empty = false;
                let mut is_run_with_path = false;
                
                for letter in args {
                    match letter {
                        'e' => { is_copy_empty = true },
                        'p' => { is_run_with_path = true },
                        // reject if not valid argument
                        unknown => {
                            println!("> Argument \"{unknown}\" does not exist");
                            return;
                        },
                    }
                }

                if is_run_with_path {
                    coms::run_with_path(added_items, is_copy_empty);
                    return;
                }
                coms::run(None, None, added_items, is_copy_empty);
                return;
            }

            println!("> Argument not valid");
        },
        ArgNum::Mas => println!("> run only takes none or one argument"),
    }
}

fn show_added(arg: SplitWhitespace<>, added_items: &Vec<PathBuf>){
    match get_arg_num(arg) {
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
    if get_arg_num(arg) == ArgNum::Nein {
        log::clear_log();
        println!("> Logs cleared");
        return;
    }

    println!("> clear_log does not accept any arguments");
}

fn undo(arg: SplitWhitespace<>, added_items: &mut Vec<PathBuf>){
    if get_arg_num(arg) == ArgNum::Nein {
        coms::undo(added_items);
        return;
    }

    println!("> undo does not accept any arguments");
}

fn quit(arg: SplitWhitespace) -> bool {
    if get_arg_num(arg) == ArgNum::Nein { return true }

    println!("> quit does not accept any arguments");
    false
}

fn get_arg_num(mut arg: SplitWhitespace<>) -> ArgNum {
    match (arg.next(), arg.next()){
        (Some(arg), None) => ArgNum::Uno(arg),
        (Some(_), Some(_)) => ArgNum::Mas,
        _ => ArgNum::Nein,
    }
}

// makes vector of only unique letters, returns none if duplicate letters
fn letter_parse(mut input: Vec<char>) -> Option< Vec<char> > {

    // should be impossible but im uneasy so
    if input.is_empty() { return None }

    // check if hyphen is first and if more characters follow
    if !(input[0] == '-') || input.len() < 2 { return None }
    // removes hyphen
    input.swap_remove(0);

    let mut output = vec![];

    for letter in input {
        if output.contains(&letter){
            return None;
        }
        output.push(letter);
    }

    Some(output)
}

#[derive(PartialEq)]
enum ArgNum<'a>{
    Nein,
    Uno(&'a str),
    Mas,
}