use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str = 
"the qiuck brown fox jumped over the lazy dog\n";

fn main() {
    let process = match Command::new("wc").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn(){
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()){
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("sent pangram to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s){
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}
