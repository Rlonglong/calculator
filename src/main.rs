mod utils;
use std::{collections::HashMap, io};
use utils::analysis::{change_to_element, input_string, remove_whitespace};
use utils::arithmetic::{self, Element};
use std::fs;


fn main() {
    let mut variables : HashMap<String, Element> = HashMap::new();
    fs::File::create("un_m.json").expect("Failed to create file.");
    println!("Haha! I think it's a calculator!\nPlease input \"exit\" to leave and \"--help\" to get help.\nHave fun with it. Haha!!!");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = remove_whitespace(&input);
        if input == "exit".to_string() {
            break;
        }
        if input == "--help".to_string() {
            let file_content = include_str!(".././help.txt");
            println!("{file_content}");
            continue;
        }
        
        let mut is_defined = false;
        let mut e = Element::Error(String::from(""));
        let v = input_string(&input, &mut variables, &mut is_defined, &mut e);
        if v.len() == 0 {
            continue;
        }
        if is_defined {
            e.output_n();
            continue;
        }
        let vv = change_to_element(v, &mut variables);
        let ans = arithmetic::arithmetic(vv);
        
        ans.output_n();
    }
    fs::remove_file("un_m.json").expect("Failed to remove file.");
}