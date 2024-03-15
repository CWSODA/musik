use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub struct Logger{
    log_file: File,
}

impl Logger{
    pub fn new(parent_dir: &Path) -> Result<Logger, &'static str>{
        match OpenOptions::new().append(true).create(true)
        .open(parent_dir.join("log.txt")){
            Ok(file) => Ok(Logger{ log_file: file }),
            Err(_) => Err("can not obtain a log.txt file")
        }
    }

    pub fn log(&mut self, msg: &str){
        self.log_file.write(msg.as_bytes()).expect("can not send msg");
    }
}