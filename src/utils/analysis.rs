use std::collections::HashMap;

use super::{arithmetic::{arithmetic, Element, Number}, predefined_constant::{assignment_constant, is_constant}, predefined_function::call_function, undefinded_function::change_unfn};
use super::arithmetic;
use super::undefinded_function::add_unfn;

pub fn input_string(input : &str, var : &mut HashMap<String, Element>, b :  &mut bool, e : &mut Element) ->  Vec<String> {
    let ha = remove_whitespace(&mut input.to_owned());
    let ha = ha.trim().to_owned();
    let ha = remove_pair_parentheses(&ha);
    let mut ha = insert_0(ha);
    if !is_define_fn(&ha) {
        ha = change_unfn(ha);
    }
    else {
        *b = true;
    }
    if !backet_good(ha.clone()) {
        *b = true;
        *e = Element::Error(String::from("InputError: the number of backet is incorrect!"));
    }
    if !parentheses_good(ha.clone()) {
        *b = true;
        *e = Element::Error(String::from("InputError: the number of parentheses  is incorrect!"));
    }
    
    return convert_postfix(ha, var);
}


pub fn change_to_element(v : Vec<String>, var : &mut HashMap<String, Element>) -> Vec<Element> {
    let mut v_element = Vec::new();
    for str in v {
        let char_vec: Vec<char> = str.chars().collect();
        if var.contains_key(&str) {
            v_element.push(var.get(&str).unwrap().clone());
        }
        else if str.len() == 1 && is_op(char_vec[0]) {
            v_element.push(Element::Op(char_vec[0]));
        }
        else if is_constant(&str) {
            v_element.push(assignment_constant(&str));
        }
        else if str.chars().nth(0).unwrap() == '[' {
            v_element.push(analysis_list(&str, var));
        }
        else if have_parentheses(&str) {
            v_element.push(analysis_function(&str, var));
        }
        else if is_num(&str) {
            v_element.push(analysis_num(&str));
        }
        else {
            v_element.push(Element::Error(format!("UndefinedError: the variable \"{}\" is undefinded!!", str)));
        }
    }
    v_element
}

fn is_num(s : &str) -> bool {
    let mut dot = false;
    let mut doot = true;
    for c in s.chars() {
        if c.is_numeric() {
            dot = true;
        }
        else if doot && dot && c == '.' {
            doot = false;
        }
        else {
            return false;
        }
    }
    true
}

fn remove_pair_parentheses(s: &str) -> String {
    let mut result = String::new();
    let mut skip_next = false;

    for (i, c) in s.chars().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        if c == '(' && s.chars().nth(i + 1) == Some(')') {
            skip_next = true;
        } else {
            result.push(c);
        }
    }

    result
}

fn backet_good(s : String) -> bool {
    let mut left_backet = 0;
    let mut right_backet = 0;
    for c in s.chars() {
        if c == '[' {left_backet += 1}
        if c == ']' {right_backet += 1}
    }
    if left_backet == right_backet {
       return true; 
    }
    return false;
}

fn parentheses_good(s : String) -> bool {
    let mut left_parentheses = 0;
    let mut right_parentheses = 0;
    for c in s.chars() {
        if c == '(' {left_parentheses += 1}
        if c == ')' {right_parentheses += 1}
    }
    if left_parentheses == right_parentheses {
       return true; 
    }
    return false;
}

fn convert2string(e : Element, mut s : String) -> String {
    match e {
        Element::Num(num) => {
            match num {
                Number::Float(f) => s += &f.to_string(),
                Number::Int(i) => s += &i.to_string()
            }
        }
        Element::Op(op) => s += &op.to_string(),
        Element::List(l) => {
            s += "[";
            let mut put_comma = false;
            for i in l {
                if put_comma {s += ","}
                put_comma = true;
                s = convert2string(i.unwrap(), s);
            }
            s += "]";
        }
        _ => (),
    }
    s
}

fn analysis_function(str : &str, var : &mut HashMap<String, Element>) -> Element {
    let mut s0 = "";
    let mut s1 = "";
    for (i, char) in str.chars().enumerate() {
        if char == '(' {
            s0 = &str[..i];
            s1 = &str[i+1..str.len()-1];
            break;
        }
    }
    if s1 == "" {return call_function(vec![s0])}
    let mut v = vec![s0];
    if s0 == "map" {
        for (i, char) in s1.chars().enumerate() {
            if char == ',' {
                v.push(&s1[0..i]);
                s1 = &s1[i+1..s1.len()];
                break;
            }
        }
    }
    let mut l  = 0;
    let s1 = s1.to_string() + ",";
    let mut temp_v : Vec<String>= Vec::new();
    let mut fumction_left_parentheses = 0;
    let mut in_the_fn = false;
    let mut left_backet = 0;
    let mut in_the_backet = false;
    for (i, char) in s1.chars().enumerate() {
        if in_the_fn {
            if char == '('{fumction_left_parentheses += 1}
            else if char == ')'{fumction_left_parentheses -= 1}
            if fumction_left_parentheses == 0 {in_the_fn = false}
            continue;
        }
        else if char == '(' {
            if !is_op_left_parentheses(i, &str) {
                in_the_fn = true;
                fumction_left_parentheses = 1;
                continue;
            }
        }
        if in_the_backet {
            if char == '['{
                left_backet += 1
            }
            else if char == ']'{
                left_backet -= 1
            }
            if left_backet == 0 {
                in_the_backet = false;
            }
            continue;
        }
        else if char == '[' {
            in_the_backet = true;
            left_backet = 1;
            continue;
        }
        if char == ',' {
            let sub_v = input_string(&s1[l..i].to_string(), var, &mut true, &mut Element::Error(String::from("")));
            let sub_v = change_to_element(sub_v, var);
            let a = convert2string(arithmetic(sub_v), "".to_string());
            temp_v.push(a);
            l = i+1;
        }
    }
    for str in &temp_v {
        v.push(str);
    }
    call_function(v)
}

fn analysis_num(str : &str) -> Element {
    let mut b :bool = false;
    for char in str.chars() {
        if char == '.' {
            b = true;
            break;
        }
    }
    if b {
        return Element::Num(Number::Float(str.parse().unwrap()));
    }
    else {
        return Element::Num(Number::Int(str.parse().unwrap()));
    }
}

fn analysis_list(str: &str, var : &mut HashMap<String, Element>) -> Element {
    let mut v = Vec::new();
    let mut l = 1;
    let mut fumction_left_parentheses = 0;
    let mut in_the_fn = false;
    let mut left_backet = 0;
    let mut in_the_backet = false;
    for (i, char) in str.chars().enumerate() {
        if i == 0 {
            continue;
        }
        if in_the_fn {
            if char == '('{
                fumction_left_parentheses += 1
            }
            else if char == ')'{
                fumction_left_parentheses -= 1
            }
            if fumction_left_parentheses == 0 {
                in_the_fn = false;
            }
            continue;
        }
        else if char == '(' {
            if !is_op_left_parentheses(i, &str) {
                in_the_fn = true;
                fumction_left_parentheses = 1;
                continue;
            }
        }
        if in_the_backet {
            if char == '['{
                left_backet += 1
            }
            else if char == ']'{
                left_backet -= 1
            }
            if left_backet == 0 {
                in_the_backet = false;
            }
            continue;
        }
        else if char == '[' {
            in_the_backet = true;
            left_backet = 1;
            continue;
        }
        if i == str.len()-1 && l < i {
            let sub_v = convert_postfix(str[l..i].to_string(), var);
            let sub_v = change_to_element(sub_v, var);
            let ans = arithmetic::arithmetic(sub_v);
            v.push(Some(ans));
        }
        if char == ',' {
            let sub_v = convert_postfix(str[l..i].to_string(), var);
            let sub_v = change_to_element(sub_v, var);
            let ans = arithmetic::arithmetic(sub_v);
            v.push(Some(ans));
            l = i+1;
        }
    }

    Element::List(v)
}

fn is_define_fn(s : &str) -> bool {
    for (i, char) in s.chars().enumerate() {
        if char == '=' {
            if have_parentheses(&s[..i]) {return true}
        }
    }
    false
}

fn have_parentheses(s : &str) -> bool {
    for char in s.chars() {
        if char == '(' {
            return true;
        }
    }
    return false;
}

fn prec(c : char) -> i64 {
    if c == '*' || c == '/' {
        return 2;
    }
    if c == '+' || c == '-' {
        return 1;
    }
    else {
        return -1;
    }
    
}

fn is_op(c : char) -> bool {
    if c == '+' || c == '-' || c == '*' || c == '/' {
        return true;
    }
    else {
        return false;
    }
}

fn is_parentheses(c : char) -> bool {
    if c == '(' || c == ')' {
        return true;
    }
    else {
        return false;
    }
}

fn is_op_left_parentheses(i : usize, s : &str) -> bool {
    let c = s.chars().nth(i).unwrap();
    if (c == '(') && (i == 0 ||  is_op(s.chars().nth(i-1).unwrap()) || s.chars().nth(i-1).unwrap() == '(' || s.chars().nth(i-1).unwrap() == ')') {
        return true;
    }
    else {
        return false;
    }
}

fn insert_0(str: String) -> String {
    let mut s = str.clone();
    let mut jj = 0;
    for (i, char) in str.chars().enumerate() {
        if i == 0 && char == '-' {
            s.insert(i+jj, '0');
            jj += 1;
        }
        else if char == '-' && (str.chars().nth(i-1).unwrap() == '(' || str.chars().nth(i-1).unwrap() == '[' || str.chars().nth(i-1).unwrap() == ',') {
            s.insert(i+jj, '0');
            jj += 1;
        }
    }
    s
}

fn convert_postfix(str : String, var : &mut HashMap<String, Element>) -> Vec<String> {
    let mut s = Vec::new();
    let mut stack = Vec::<char>::new();
    let mut l = 0;
    let mut fumction_left_parentheses = 0;
    let mut in_the_fn = false;
    let mut left_backet = 0;
    let mut in_the_backet = false;
    if is_define_fn(&str) {
        let mut name = String::new();
        let mut variables = Vec::new();
        let mut formula = String::new();
        let mut l = 0;
        let mut r = 0;
        let mut l_didnt_exist = true;
        let mut r_didnt_exist = true;
        for (i, char) in str.chars().enumerate() {
            if l_didnt_exist && char == '(' {
                name = str[..i].to_string();
                l = i+1;
                l_didnt_exist = false;
            }
            if !l_didnt_exist && r_didnt_exist && char == ','{
                variables.push(str[l..i].to_string());
                l = i+1;
            }
            if r_didnt_exist && char == ')' {
                variables.push(str[l..i].to_string());
                r = i+2;
                r_didnt_exist = false;
            }
            if char == '=' {
                formula = str[r..str.len()].to_string();
                break;
            }
        }
        add_unfn(name, variables, formula);
        return vec!["0".to_string()];
    }
    for (i, char) in str.chars().enumerate() {
        if char == '=' {
            let v = input_string(&str[i+1..str.len()], var, &mut true, &mut Element::Error(String::from("")));
            let vv = change_to_element(v, var);
            let ans = arithmetic::arithmetic(vv);
            let ha = str[l..i].to_owned();
            var.insert(ha, ans);
            return vec![str[l..i].to_owned()];
        }
        if in_the_backet {
            if char == '['{
                left_backet += 1
            }
            else if char == ']'{
                left_backet -= 1
            }
            if left_backet == 0 {
                in_the_backet = false;
            }
            continue;
        }
        else if char == '[' {
            in_the_backet = true;
            left_backet = 1;
            continue;
        }
        if is_op(char) {
            if fumction_left_parentheses != 0 {
                continue;
            }
            if l != i {
                s.push(str[l..i].to_string()); 
            }
            l = i+1;
            while !stack.is_empty() {
                let top = stack.pop();
                if prec(top.unwrap()) >= prec(char) {
                    s.push(top.unwrap().to_string());
                }
                else {
                    stack.push(top.unwrap());
                    break;
                }
            }
            stack.push(char);
        }
        if is_parentheses(char) {
            if in_the_fn {
                if char == '('{
                    fumction_left_parentheses += 1
                }
                else if char == ')'{
                    fumction_left_parentheses -= 1
                }
                if fumction_left_parentheses == 0 {
                    in_the_fn = false;
                }
                continue;
            }
            else if char == '(' {
                if !is_op_left_parentheses(i, &str) {
                    in_the_fn = true;
                    fumction_left_parentheses = 1;
                    continue;
                }
            }
            
            if l != i {
               s.push(str[l..i].to_string()); 
            }
            l = i+1;
            if char == '(' {
                stack.push(char);
            }
            else {
                while !s.is_empty() {
                    let top = stack.pop();
                    if top.unwrap() == '(' {
                        break;
                    }
                    s.push(top.unwrap().to_string());
                }
            }
        }
    }
    if l < str.len() {s.push(str[l..].to_string())};
    while !stack.is_empty() {
        s.push(stack.pop().unwrap().to_string());
    }
    s
}

pub fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}