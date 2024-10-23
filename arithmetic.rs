use core::panic;
extern crate round;

#[derive(Debug, Clone)]
pub enum Number{Int(i64), Float(f64)}

#[derive(Debug, Clone)]
pub enum Element{
    Num(Number),
    Bool(bool),
    Op(char),
    List(Vec<Option<Element>>),
}

impl Element {
    fn output(&self) {
        match self {
            Element::Num(num) => {
                match num{
                    Number::Float(f) => print!("{}", round::round(*f, 10)),
                    Number::Int(i) => print!("{}", i),
                }
            }
            Element::Op(op) => print!("{op}"),
            Element::List(l) => {
                print!("[");
                let mut b = false;
                for i in l {
                    if b {print!(", ")}
                    i.as_ref().unwrap().output();
                    b = true;
                }
                print!("]");
            }
            _ => (),
        }
    }
    pub fn output_n(&self) {
        self.output();
        println!("");
    }
}


pub fn arithmetic(v : Vec<Element>) -> Element {
    if have_list(&v) || have_float(&v) {     // list
        /* 
        let len = len_of_list(&v[0]);
        for e in &v {
            if len_of_list(&e) != len {
                panic!("ListError: input of lists have different length.");
            }
        }
        */

        let v = convert_to_float(v);
        let mut stack : Vec<Element>= Vec::new();
        for e in v {
            match e {
                Element::Num(num) => {
                    match num {
                        Number::Float(f) => stack.push(Element::Num(Number::Float(f))),
                        _ => (),
                    }
                }
                Element::List(l) => stack.push(Element::List(l)),
                Element::Op(op) => {
                    let f2 = stack.pop().unwrap();
                    let f1 = stack.pop().unwrap();
                    match op {
                        '+' => stack.push(ee_addition(&f1, &f2)),
                        '-' => stack.push(ee_subtraction(&f1, &f2)),
                        '*' => stack.push(ee_multiplication(&f1, &f2)),
                        '/' => {
                            //if f2 == 0.0 {
                            //    panic!("Math error: cannot divide zero!")
                            //}
                            stack.push(ee_division(&f1, &f2));
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        return stack.remove(0);
    }
    /* 
    else if have_float(&v) {     // float
        let v = convert_to_float(v);
        let mut stack : Vec<f64>= Vec::new();
        for e in v {
            match e {
                Element::Num(num) => {
                    match num {
                        Number::Float(f) => stack.push(f),
                        _ => (),
                    }
                }
                Element::Op(op) => {
                    let f2 = stack.pop().unwrap();
                    let f1 = stack.pop().unwrap();
                    match op {
                        '+' => stack.push(f1+f2),
                        '-' => stack.push(f1-f2),
                        '*' => stack.push(f1*f2),
                        '/' => {
                            if f2 == 0.0 {
                                panic!("Math error: cannot divide zero!")
                            }
                            stack.push(f1/f2);
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        return  Element::Num(Number::Float(stack[0]));
    }
    */
    else {     // int
        let mut stack : Vec<i64>= Vec::new();
        for e in v {
            match e {
                Element::Num(num) => {
                    match num {
                        Number::Int(i) => stack.push(i),
                        _ => (),
                    }
                }
                Element::Op(op) => {
                    let f2 = stack.pop().unwrap();
                    let f1 = stack.pop().unwrap();
                    match op {
                        '+' => stack.push(f1+f2),
                        '-' => stack.push(f1-f2),
                        '*' => stack.push(f1*f2),
                        '/' => {
                            if f2 == 0 {
                                panic!("Math error: cannot divide zero!")
                            }
                            stack.push(f1/f2);
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        return Element::Num(Number::Int(stack[0]));
    }
}

fn len_of_list(e : &Element) -> usize {
    match e {
        Element::List(list) => {
            let mut num = 0;
            for _ in list {
                num += 1;
            }
            return num;
        }
        _ => panic!("InputError: input must be a list."),
    }
}

fn have_float(v : &Vec<Element>) -> bool{
    let mut b = false;
    for e in v {
        match e {
            Element::Num(num) => {
                match num {
                    Number::Float(_) => b = true,
                    _ => (),
                }
                
            }
            _ => (),
        }
    }
    b
}

fn have_list(v : &Vec<Element>) -> bool{
    let mut b = false;
    for e in v {
        match e {
            Element::List(_) => b = true,
            _ => (),
        }
    }
    b
}

fn ee_addition (e1: &Element, e2: &Element) -> Element{
    //assert_eq!(len_of_list(&e1),  len_of_list(&e2));
    match (e1, e2) {
        (Element::Num(num1), Element::Num(num2)) => {
            match (num1, num2) {
                (Number::Float(f1), Number::Float(f2)) => return Element::Num(Number::Float(f1+f2)),
                _ => return Element::Bool(false),
            }
        }
        (Element::List(l1), Element::List(l2)) => {
            let mut new_l : Vec<Option<Element>> = Vec::new();
            for i in 0..l1.len() {
                new_l.push(Some(ee_addition(l1[i].as_ref().unwrap(), l2[i].as_ref().unwrap())));
            }
            Element::List(new_l)
        }
        _ => return Element::Bool(false),
    }
}

fn ee_subtraction (e1: &Element, e2: &Element) -> Element{
    //assert_eq!(len_of_list(&e1),  len_of_list(&e2));
    match (e1, e2) {
        (Element::Num(num1), Element::Num(num2)) => {
            match (num1, num2) {
                (Number::Float(f1), Number::Float(f2)) => return Element::Num(Number::Float(f1-f2)),
                _ => return Element::Bool(false),
            }
        }
        (Element::List(l1), Element::List(l2)) => {
            let mut new_l : Vec<Option<Element>> = Vec::new();
            for i in 0..l1.len() {
                new_l.push(Some(ee_subtraction(l1[i].as_ref().unwrap(), l2[i].as_ref().unwrap())));
            }
            Element::List(new_l)
        }
        _ => return Element::Bool(false),
    }
}

fn ee_multiplication (e1: &Element, e2: &Element) -> Element{
    //assert_eq!(len_of_list(&e1),  len_of_list(&e2));
    match (e1, e2) {
        (Element::Num(num1), Element::Num(num2)) => {
            match (num1, num2) {
                (Number::Float(f1), Number::Float(f2)) => return Element::Num(Number::Float(f1*f2)),
                _ => return Element::Bool(false),
            }
        }
        (Element::List(l1), Element::List(l2)) => {
            let mut new_l : Vec<Option<Element>> = Vec::new();
            for i in 0..l1.len() {
                new_l.push(Some(ee_multiplication(l1[i].as_ref().unwrap(), l2[i].as_ref().unwrap())));
            }
            Element::List(new_l)
        }
        _ => return Element::Bool(false),
    }
}

fn ee_division (e1: &Element, e2: &Element) -> Element{
    //assert_eq!(len_of_list(&e1),  len_of_list(&e2));
    match (e1, e2) {
        (Element::Num(num1), Element::Num(num2)) => {
            match (num1, num2) {
                (Number::Float(f1), Number::Float(f2)) => return Element::Num(Number::Float(f1/f2)),
                _ => return Element::Bool(false),
            }
        }
        (Element::List(l1), Element::List(l2)) => {
            let mut new_l : Vec<Option<Element>> = Vec::new();
            for i in 0..l1.len() {
                new_l.push(Some(ee_division(l1[i].as_ref().unwrap(), l2[i].as_ref().unwrap())));
            }
            Element::List(new_l)
        }
        _ => return Element::Bool(false),
    }
}



fn convert_to_float(v : Vec<Element>) -> Vec<Element>{
    let mut v_new = Vec::new();
    for e in v {
        match e {
            Element::Num(num) => {
                match num {
                    Number::Int(int) => v_new.push(Element::Num(Number::Float(int as f64))),
                    Number::Float(float) => v_new.push(Element::Num(Number::Float(float))),
                }
                
            }
            Element::Bool(bool) => v_new.push(Element::Bool(bool)),
            Element::Op(op) => v_new.push(Element::Op(op)),
            Element::List(list) => {
                let mut ls : Vec<Element> = Vec::new();
                for i in list {
                    ls.push(i.unwrap());
                }
                let mut vv_new : Vec<Option<Element>> = Vec::new();
                for i in convert_to_float(ls) {
                    vv_new.push(Some(i));
                }
                v_new.push(Element::List(vv_new));
            }
        }
        
    }
    v_new
}