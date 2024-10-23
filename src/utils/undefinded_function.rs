extern crate regex;
use regex::Regex;
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::{BufWriter, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Unfn {
    name: String,
    variables: Vec<String>,
    formula : String,
}


pub fn add_unfn(name : String, variables: Vec<String>, formula: String) {
    let mut fns = read_unfn();
    let f = Unfn {name, variables, formula};
    let mut d = -1;
    for (i, pre_fn) in fns.iter().enumerate() {
        if pre_fn.name == f.name {
            d = i as i64;
        }
    } 
    if d != -1 {
        fns.remove(d as usize);
    }
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

pub fn exist_unfn(s : String) -> bool {
    let fns = read_unfn();
    for unfn in fns {
        if unfn.name == s{
            return true;
        }
    }
    return false;
}

fn read_unfn() -> Vec<Unfn> {
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
/*
fn make_map() -> HashMap<String, Unfn> {
    read_unfn()
        .into_iter()
        .map(|s| (s.name.clone(), s))
        .collect()
}
*/

pub fn change_unfn(mut s : String) -> String {
    let v = read_unfn();
    for unfn in v {
        let re = unfn.name + r"\(([^()]*|([^()]*\(.*\)))\)";
        let re = Regex::new(&re).unwrap();
        
       
        while re.is_match(&s) {
            s = re.replace_all(&s, |caps: &regex::Captures| {
                let inner_expressions = caps[1].to_owned();
                let mut new_expression = unfn.formula.clone();
                let mut l = 0;
                let mut left_backet = 0;
                let mut in_the_backet = false;
                let mut inner_expression = "";
                let mut cnt = 0;
                for j in l..inner_expressions.len()+1 {
                    if in_the_backet {
                        if inner_expressions.chars().nth(j).unwrap() == '['{
                            left_backet += 1
                        }
                        else if inner_expressions.chars().nth(j).unwrap() == ']'{
                            left_backet -= 1
                        }
                        if left_backet == 0 {
                            in_the_backet = false;
                        }
                        continue;
                    }
                    else if j != inner_expressions.len() && inner_expressions.chars().nth(j).unwrap() == '[' {
                        in_the_backet = true;
                        left_backet = 1;
                        continue;
                    }
                    if j == inner_expressions.len() || inner_expressions.chars().nth(j).unwrap() == ',' {
                        cnt += 1;
                    }
                }
                if cnt != unfn.variables.len() {return "".to_string();}
                println!("{} {}", cnt, unfn.variables.len());
                for i in 0..unfn.variables.len() {
                    for j in l..inner_expressions.len()+1 {
                        if in_the_backet {
                            if inner_expressions.chars().nth(j).unwrap() == '['{
                                left_backet += 1
                            }
                            else if inner_expressions.chars().nth(j).unwrap() == ']'{
                                left_backet -= 1
                            }
                            if left_backet == 0 {
                                in_the_backet = false;
                            }
                            continue;
                        }
                        else if j != inner_expressions.len() && inner_expressions.chars().nth(j).unwrap() == '[' {
                            in_the_backet = true;
                            left_backet = 1;
                            continue;
                        }
                        if j == inner_expressions.len() || inner_expressions.chars().nth(j).unwrap() == ',' {
                            inner_expression = &inner_expressions[l..j];
                            l = j+1;
                            break;
                        }
                    }
                    new_expression = new_expression.replace(&unfn.variables[i], &inner_expression);
                }
                format!("({})", new_expression)
            }).to_string();
        }
    }

    s
}