use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;


pub fn log(msg: &str){
    let msg = format!("{msg}\n");

    OpenOptions::new().append(true).create(true).open("log.txt")
    .expect("cant open log").write(msg.as_bytes()).expect("can not send msg");
}

pub fn clear_log(){
    File::create("log.txt").expect("cannot open file");
}