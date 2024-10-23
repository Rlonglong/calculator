use core::f64;
use std::collections::HashMap;

use super::analysis::{change_to_element, input_string};
use super::arithmetic::{self, Element, Number};
extern crate round;
use super::undefinded_function::{change_unfn, exist_unfn};
use rand::Rng;

fn sin(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(f64::sin(x)))
}


fn cos(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(f64::cos(x)))
}

fn tan(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(f64::tan(x)))
}

fn asin(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(f64::asin(x)))
}

fn acos(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(f64::acos(x)))
}

fn atan(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(f64::atan(x)))
}

fn ln(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(f64::ln(x)))
}

fn log10(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(f64::log10(x)))
}

fn log2(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(f64::log2(x)))
}

fn log(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 2);
    let x : f64 = v[0].parse().unwrap();
    let base : f64 = v[1].parse().unwrap();
    Element::Num(Number::Float(f64::log(x, base)))
}

fn round_0(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(round::round(x, 0)))
}

fn round_n(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 2);
    let x : f64 = v[0].parse().unwrap();
    let n : f64 = v[1].parse().unwrap();
    Element::Num(Number::Float(round::round(x, n as i32)))
}

fn floor_0(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(round::round_down(x, 0)))
}

fn floor_n(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 2);
    let x : f64 = v[0].parse().unwrap();
    let n : f64 = v[1].parse().unwrap();
    Element::Num(Number::Float(round::round_down(x, n as i32)))
}

fn ceil_0(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(round::round_up(x, 0)))
}

fn ceil_n(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 2);
    let x : f64 = v[0].parse().unwrap();
    let n : f64 = v[1].parse().unwrap();
    Element::Num(Number::Float(round::round_up(x, n as i32)))
}

fn sqrt(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(f64::sqrt(x)))
}

fn exp(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(f64::exp(x)))
}

fn pow(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 2);
    let x : f64 = v[0].parse().unwrap();
    let y : f64 = v[1].parse().unwrap();
    Element::Num(Number::Float(f64::powf(x, y)))
}

fn abs(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let x : f64 = v[0].parse().unwrap();
    Element::Num(Number::Float(f64::abs(x)))
}

fn min(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 2);
    let x : f64 = v[0].parse().unwrap();
    let y : f64 = v[1].parse().unwrap();
    Element::Num(Number::Float(x.min(y)))
}

fn max(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 2);
    let x : f64 = v[0].parse().unwrap();
    let y : f64 = v[1].parse().unwrap();
    Element::Num(Number::Float(x.max(y)))
}

fn min_list(v : Vec<&str>) -> Element {
    let mut mn : f64 = f64::MAX;
    let mut l = 1;
    for (i, char) in v[0].chars().enumerate() {
        if i == 0 {continue}
        if char == '[' {
            return Element::Error(String::from("TypeError: cannot compare with a list."));
        }
        if char == ','  || char == ']'{
            let x : f64 = v[0][l..i].parse().unwrap();
            mn = mn.min(x);
            l = i+1;
        }
    }
    Element::Num(Number::Float(mn))
}

fn max_list(v : Vec<&str>) -> Element {
    let mut mx : f64 = f64::MIN;
    let mut l = 1;
    for (i, char) in v[0].chars().enumerate() {
        if i == 0 {continue}
        if char == '[' {
            return Element::Error(String::from("TypeError: cannot compare with a list."));
        }
        if char == ','  || char == ']'{
            let x : f64 = v[0][l..i].parse().unwrap();
            mx = mx.max(x);
            l = i+1;
        }
    }
    Element::Num(Number::Float(mx))
}

fn rand0to10(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 0);
    let n1: f64 = rand::thread_rng().gen_range(0.0..10.0);
    Element::Num(Number::Float(n1))
}

fn rand0tox(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let n1: f64 = rand::thread_rng().gen_range(0.0..v[0].parse().unwrap());
    Element::Num(Number::Float(n1))
}

fn randxtoy(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 2);
    let n1: f64 = rand::thread_rng().gen_range(v[0].parse().unwrap()..v[1].parse().unwrap());
    Element::Num(Number::Float(n1))
}

fn range1tox(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let mut list = Vec::new();
    let stop = v[0].parse::<i64>().unwrap()+1;
    for i in 1..stop {
        list.push(Some(Element::Num(Number::Int(i))));
    }
    Element::List(list)
}

fn rangextoy(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 2);
    let mut list = Vec::new();
    let start = v[0].parse::<i64>().unwrap();
    let stop = v[1].parse::<i64>().unwrap()+1;
    for i in start..stop {
        list.push(Some(Element::Num(Number::Int(i))));
    }
    Element::List(list)
}

fn map(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 2);
    let function;
    if v[0].len() >= 2 && &v[0][v[0].len()-2..v[0].len()] == "()" {
        function = &v[0][..v[0].len()-2];
    }
    else {
        function = &v[0][..v[0].len()];
    }
    let mut list = Vec::new();
    let mut l = 0;
    let mut left_backet = 0;
    let mut in_the_backet = false;
    for (i, char) in v[1][1..v[1].len()-1].chars().enumerate() {
        if in_the_backet {
            if char == '['{
                left_backet += 1
            }
            else if char == ']'{
                left_backet -= 1
            }
            if left_backet == 0 {
                in_the_backet = false;
                let vv = vec![function, &v[1][l+1..i+1]];
                list.push(Some(map(vv)));
                l = i+2;
            }
            continue;
        }
        if char == '[' {
            in_the_backet = true;
            left_backet = 1;
            continue;
        }
        if char == ',' && l < i {
            let vv = vec![function, &v[1][l+1..i+1]];
            list.push(Some(call_function(vv)));
            l = i+1;
        }
        else if i == &v[1].len()- 3 && l <= i {
            let vv = vec![function, &v[1][l+1..v[1].len()-1]];
            list.push(Some(call_function(vv)));
            l = i+1;
        }
    }
    Element::List(list)
}

fn sort(v : Vec<&str>) -> Element {
    assert_eq!(v.len(), 1);
    let mut list = Vec::new();
    let mut l = 0;
    for (i, char) in v[0][1..v[0].len()-1].chars().enumerate() {
        if char == '[' {
            return Element::Error(String::from("TypeError: cannot compare with a list."));
        }
        if char == ','{
            let x : f64 = v[0][l+1..i+1].parse().unwrap();
            list.push(x);
            l = i+1;
        }
        else if i == &v[0].len()- 3 {
            let x : f64 = v[0][l+1..v[0].len()-1].parse().unwrap();
            list.push(x);
        }
    }
    list.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    let mut l = Vec::new();
    for i in list {
        l.push(Some(Element::Num(Number::Float(i))));
    }
    Element::List(l)
}

fn length(v : Vec<&str>) -> Element{
    assert_eq!(v.len(), 1);
    let mut cnt = 0;
    let mut left_backet = 0;
    let mut in_the_backet = false;
    for (i, char) in v[0][1..v[0].len()-1].chars().enumerate() {
        if in_the_backet {
            if char == '['{
                left_backet += 1
            }
            else if char == ']'{
                left_backet -= 1
            }
            if left_backet == 0 {
                in_the_backet = false;
                cnt += 1;
            }
            continue;
        }
        else if char == '[' {
            in_the_backet = true;
            left_backet = 1;
            continue;
        }
        if i == &v[0].len()- 3 || char == ','{
            cnt += 1;
        }
    }

    Element::Num(Number::Int(cnt))
}


pub fn call_function(v : Vec<&str>) -> Element{
    let parameter = v[1..].to_vec();
    match v[0] {
        "sin" => sin(parameter),
        "cos" => cos(parameter),
        "tan" => tan(parameter),
        "asin" => asin(parameter),
        "acos" => acos(parameter),
        "atan" => atan(parameter),
        "ln" => ln(parameter),
        "log10" => log10(parameter),
        "log2" => log2(parameter),
        "log" => log(parameter),
        "round" => {
            if v.len() == 2 {
                round_0(parameter)
            }
            else {
                round_n(parameter)
            }
        }
        "floor" => {
            if v.len() == 2 {
                floor_0(parameter)
            }
            else {
                floor_n(parameter)
            }
        }
        "ceil" => {
            if v.len() == 2 {
                ceil_0(parameter)
            }
            else {
                ceil_n(parameter)
            }
        }
        "sqrt" => sqrt(parameter),
        "exp" => exp(parameter),
        "pow" => pow(parameter),
        "min" => {
            if v.len() == 3 {
                min(parameter)
            }
            else {
                min_list(parameter)
            }
        }
        "max" => {
            if v.len() == 3 {
                max(parameter)
            }
            else {
                max_list(parameter)
            }
        }
        "rand" => {
            if v.len() == 1 {
                rand0to10(parameter)
            }
            else if v.len() == 2 {
                rand0tox(parameter)
            }
            else {
                randxtoy(parameter)
            }
        }
        "range" => {
            if v.len() == 2 {
                range1tox(parameter)
            }
            else {
                rangextoy(parameter)
            }
        }
        "map" => map(parameter),
        "sort" => sort(parameter),
        "length" => length(parameter),
        "abs" => abs(parameter),
        _ => {
            if !exist_unfn(v[0].to_owned()) {
                return Element::Error(format!("UndefinedError: the function \"{}\" is undefinded!!", v[0].to_owned()));
            }
            let sol = v[0].to_owned() + "(" + parameter[0] + ")";
            let input = change_unfn(sol);
            let mut var : HashMap<String, Element> = HashMap::new();
            let v = input_string(&input, &mut var, &mut true, &mut Element::Error(String::from("")));
            let vv = change_to_element(v, &mut var);
            let ans = arithmetic::arithmetic(vv);
            ans
        }
    }

}