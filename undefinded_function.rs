use super::arithmetic::Element;
use std::borrow::Borrow;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::{BufWriter, Write};
use std::fs;
use serde::{Deserialize, Serialize};
use std::process::{Command, ExitStatus};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct unfn {
    name: String,
    variable: String,
    formula : String,
}


pub fn add_unfn(name : String, variable: String, formula: String) {
    let mut fns = read_unfn();
    let f = unfn {name, variable, formula};
    fns.push(f);
    
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("un_m.json")
        .expect("FileEror: cannot open the file.");
    let mut writer = BufWriter::new(file);
    let _= serde_json::to_writer(&mut writer, &fns).expect("WriteError: file failed to write.");
    let _= writer.flush();
}

fn build_match_rs() {
    let head = "pub fn call_function(v : Vec<&str>) {\n\tlet parameter = v[1..].to_vec();\n\tmatch v[0] {\n";
    let tail = "\t\t_ => (),\n\t}\n}\n";
    let fns = read_unfn();

    let mut to_write = String::from(head);
    for f in fns {
        to_write += &format!("\t\t\"{}\" => {}(parameter),\n", f.name, f.name);
    }
    to_write += tail;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("un_m.rs")
        .expect("FileEror: cannot open the file.");


    file.write_all(to_write.as_bytes()).expect("FileError: cannot write the file.");
}

fn build_main_rs() {
    let to_write = "use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let mut v = Vec::new();
		v.push(args[1].as_str());
		v.push(args[2].as_str());
        call_function(v);
    } else {
        println!(\"No enough arguments provided.\");
    }
}";

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("un_m.rs")
        .expect("FileEror: cannot open the file.");


    file.write_all(to_write.as_bytes()).expect("FileError: cannot write the file.");
}

pub fn create_all_unfn() {
    let fns = read_unfn();

    let mut to_write = String::from("");
    for f in fns {
        to_write += &format!("fn {}(v : Vec<&str>) {{
\tassert_eq!(v.len(), 1);
\tlet {} : f64 = v[0].parse().unwrap();
\tlet y = {};
\tprint!(\"{{}}\", y);
}}\n", f.name, f.variable, f.formula);
    }
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("un_m.rs")
        .expect("FileEror: cannot open the file.");


    file.write_all(to_write.as_bytes()).expect("FileError: cannot write the file.");

    build_match_rs();
    build_main_rs();
}

pub fn read_unfn() -> Vec<unfn> {
    let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open("un_m.json")
            .expect("FileEror: cannot read the file.");

    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(old_fns) => old_fns, 
        Err(_) => vec![],
    }
}


pub fn call_udfunction(args: Vec<&str>) -> String{
    if Path::new("un_m.rs_executable").exists(){
        fs::remove_file("un_m.rs_executable").expect("FileError: cannot delete file.");
    }
    
    let compile_status: ExitStatus = Command::new("rustc")
        .arg("un_m.rs")
        .arg("-o")
        .arg("un_m.rs_executable")
        .arg("-A")
        .arg("warnings")
        .status()
        .expect("Failed to compile other.rs");



    if compile_status.success() {
        let output = Command::new("./un_m.rs_executable")
            .args(&args)
            .output()
            .expect("Failed to execute other");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return stdout.as_ref().to_owned();
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("Error executing other.rs:\n{}", stderr);
        }
    } else {
        panic!("Failed to compile other.rs");
    }
}
