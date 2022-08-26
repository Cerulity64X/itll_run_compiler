use std::{env::{args, Args}, fs::{self, File}, path::Path};

fn main() {
    let argv: Args = args();
    let path: String;
    let mut argl: Vec<String> = vec![];
    for a in argv {
        argl.push(a);
    }
    path = argl.get(1).expect("Please input a file!").clone();
    let mut script: Vec<Vec<String>> = vec![];
    for s in fs::read_to_string(path).expect("Could not read the file!").split('\n') {
        let mut v: Vec<String> = vec![];
        for word in s.split(' ') {
            v.push(word.trim().to_string());
        }
        script.push(v);
    }
    let mut f = File::create("out.irc");
    dbg!(script);
}
