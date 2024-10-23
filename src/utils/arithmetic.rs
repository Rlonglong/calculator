extern crate round;

#[derive(Debug, Clone)]
pub enum Number{Int(i64), Float(f64)}

#[derive(Debug, Clone)]
pub enum Element{
    Num(Number),
    Op(char),
    List(Vec<Option<Element>>),
    Error(String),
}

impl Element {
    fn output(&self, bb : bool) {
        if bb || has_error(self) {
            match self {
                Element::List(l) => {
                    for i in l {
                        i.as_ref().unwrap().output(true);
                    }
                }
                Element::Error(e) => if *e != "".to_string() {print!("❣️{e}")},
                _ => (),
            }
        }
        else {
            match self {
                Element::Num(num) => {
                    match num{
                        Number::Float(f) => {
                            let f = if round::round(*f, 10) == -0.0 { 0.0 } else { *f };
                            print!("{}", f);
                        }
                        Number::Int(i) => print!("{}", i),
                    }
                }
                Element::Op(op) => print!("{op}"),
                Element::List(l) => {
                    print!("[");
                    let mut b = false;
                    for i in l {
                        if b {print!(", ")}
                        i.as_ref().unwrap().output(false);
                        b = true;
                    }
                    print!("]");
                }
                Element::Error(e) => if *e != "".to_string() {print!("❣️{e}")},
            }
        }
    }
    pub fn output_n(&self) {
        self.output(false);
        println!("");
    }
}


pub fn arithmetic(v : Vec<Element>) -> Element {
    if have_list(&v) || have_float(&v) {
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
                    if stack.is_empty() {return Element::Error(String::from("InputError: operation is illegal."))}
                    let f2 = stack.pop().unwrap();
                    if stack.is_empty() {return Element::Error(String::from("InputError: operation is illegal."))}
                    let f1 = stack.pop().unwrap();
                    match op {
                        '+' => stack.push(ee_addition(&f1, &f2)),
                        '-' => stack.push(ee_subtraction(&f1, &f2)),
                        '*' => stack.push(ee_multiplication(&f1, &f2)),
                        '/' => stack.push(ee_division(&f1, &f2)),
                        _ => return Element::Error(format!("UndefinedError: the operator \"{}\" is undefinded!!", op)),
                    }
                }
                _ => (),
            }
        }
        if stack.len() > 1 {return Element::Error(String::from("InputError: operation is illegal."))}
        return stack.remove(0);
    }

    else {
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
                    if stack.is_empty() {return Element::Error(String::from("InputError: operation is illegal."))}
                    let f2 = stack.pop().unwrap();
                    if stack.is_empty() {return Element::Error(String::from("InputError: operation is illegal."))}
                    let f1 = stack.pop().unwrap();
                    match op {
                        '+' => stack.push(f1+f2),
                        '-' => stack.push(f1-f2),
                        '*' => stack.push(f1*f2),
                        '/' => {
                            if f2 == 0 {
                                return Element::Error(String::from("Math error: cannot divide zero!"));
                            }
                            stack.push(f1/f2);
                        }
                        _ => (),
                    }
                }
                Element::Error(_) => return e,
                _ => (),
            }
        }
        if stack.len() > 1 {return Element::Error(String::from("InputError: operation is illegal."))}
        return Element::Num(Number::Int(stack[0]));
    }
}

fn has_error(e : &Element) -> bool {
    match e {
        Element::Error(_) => true,
        Element::List(l) => {
            let mut ha = false;
            for ee in l {
                ha = ha || has_error(&ee.clone().unwrap());
            }
            ha
        }
        _ => false,
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
        _ => 0,
    }
}

fn have_float(v : &Vec<Element>) -> bool{
    for e in v {
        match e {
            Element::Num(num) => {
                match num {
                    Number::Float(_) => return true,
                    _ => (),
                }
                
            }
            _ => (),
        }
    }
    false
}

fn have_list(v : &Vec<Element>) -> bool{
    for e in v {
        match e {
            Element::List(_) => return true,
            _ => (),
        }
    }
    false
}

fn ee_addition (e1: &Element, e2: &Element) -> Element{
    match e1 {
        Element::Error(_) => return e1.clone(),
        _ => (),
    }
    match e2 {
        Element::Error(_) => return e2.clone(),
        _ => (),
    }
    match (e1, e2) {
        (Element::Num(num1), Element::Num(num2)) => {
            match (num1, num2) {
                (Number::Float(f1), Number::Float(f2)) => return Element::Num(Number::Float(f1+f2)),
                _ => return Element::Error(String::from("MyError: you shouldn't see this. zzz")),
            }
        }
        (Element::List(l1), Element::List(l2)) => {
            if len_of_list(&e1) !=  len_of_list(&e2) {
                return Element::Error(String::from("TypeError: cannot add with two different lenght list"));
            }
            let mut new_l : Vec<Option<Element>> = Vec::new();
            for i in 0..l1.len() {
                new_l.push(Some(ee_addition(l1[i].as_ref().unwrap(), l2[i].as_ref().unwrap())));
            }
            Element::List(new_l)
        }
        _ => return Element::Error(String::from("TypeError: addition with two different type!!")),
    }
}

fn ee_subtraction (e1: &Element, e2: &Element) -> Element{
    match e1 {
        Element::Error(_) => return e1.clone(),
        _ => (),
    }
    match e2 {
        Element::Error(_) => return e2.clone(),
        _ => (),
    }
    match (e1, e2) {
        (Element::Num(num1), Element::Num(num2)) => {
            match (num1, num2) {
                (Number::Float(f1), Number::Float(f2)) => return Element::Num(Number::Float(f1-f2)),
                _ => return Element::Error(String::from("MyError: you shouldn't see this. zzz")),
            }
        }
        (Element::List(l1), Element::List(l2)) => {
            if len_of_list(&e1) !=  len_of_list(&e2) {
                return Element::Error(String::from("TypeError: cannot subtract with two different lenght list"));
            }
            let mut new_l : Vec<Option<Element>> = Vec::new();
            for i in 0..l1.len() {
                new_l.push(Some(ee_subtraction(l1[i].as_ref().unwrap(), l2[i].as_ref().unwrap())));
            }
            Element::List(new_l)
        }
        _ => return Element::Error(String::from("TypeError: subtraction with two different type!!")),
    }
}

fn ee_multiplication (e1: &Element, e2: &Element) -> Element{
    match e1 {
        Element::Error(_) => return e1.clone(),
        _ => (),
    }
    match e2 {
        Element::Error(_) => return e2.clone(),
        _ => (),
    }
    match (e1, e2) {
        (Element::Num(num1), Element::Num(num2)) => {
            match (num1, num2) {
                (Number::Float(f1), Number::Float(f2)) => return Element::Num(Number::Float(f1*f2)),
                _ => return Element::Error(String::from("MyError: you shouldn't see this. zzz")),
            }
        }
        (Element::List(l1), Element::List(l2)) => {
            if len_of_list(&e1) !=  len_of_list(&e2) {
                return Element::Error(String::from("TypeError: cannot multiplicate with two different lenght list"));
            }
            let mut new_l : Vec<Option<Element>> = Vec::new();
            for i in 0..l1.len() {
                new_l.push(Some(ee_multiplication(l1[i].as_ref().unwrap(), l2[i].as_ref().unwrap())));
            }
            Element::List(new_l)
        }
        (Element::List(l1), Element::Num(num2)) => {
            match num2 {
                Number::Float(f2) => {
                    let mut new_l : Vec<Option<Element>> = Vec::new();
                    for i in 0..l1.len() {
                        new_l.push(Some(ee_multiplication(l1[i].as_ref().unwrap(),  &Element::Num(Number::Float(*f2)))));
                    }
                    Element::List(new_l)
                }
                _ => return Element::Error(String::from("MyError: you shouldn't see this. zzz")),
            }
        }
        (Element::Num(num1), Element::List(l2)) => {
            match num1 {
                Number::Float(f1) => {
                    let mut new_l : Vec<Option<Element>> = Vec::new();
                    for i in 0..l2.len() {
                        new_l.push(Some(ee_multiplication(&Element::Num(Number::Float(*f1)), l2[i].as_ref().unwrap())));
                    }
                    Element::List(new_l)
                }
                _ => return Element::Error(String::from("MyError: you shouldn't see this. zzz")),
            }
        }
        _ => return Element::Error(String::from("TypeError: multiplication with two illegal type!!")),
    }
}

fn ee_division (e1: &Element, e2: &Element) -> Element{
    match e1 {
        Element::Error(_) => return e1.clone(),
        _ => (),
    }
    match e2 {
        Element::Error(_) => return e2.clone(),
        _ => (),
    }
    match (e1, e2) {
        (Element::Num(num1), Element::Num(num2)) => {
            match (num1, num2) {
                (Number::Float(f1), Number::Float(f2)) => {
                    if *f2 == 0.0 {
                        return Element::Error(String::from("Math error: cannot divide zero!"));
                    }
                    return Element::Num(Number::Float(f1/f2));
                }
                _ => return Element::Error(String::from("MyError: you shouldn't see this. zzz")),
            }
        }
        (Element::List(l1), Element::List(l2)) => {
            if len_of_list(&e1) !=  len_of_list(&e2) {
                return Element::Error(String::from("TypeError: cannot divise with two different lenght list"));
            }
            let mut new_l : Vec<Option<Element>> = Vec::new();
            for i in 0..l1.len() {
                new_l.push(Some(ee_division(l1[i].as_ref().unwrap(), l2[i].as_ref().unwrap())));
            }
            Element::List(new_l)
        }
        (Element::List(l1), Element::Num(num2)) => {
            match num2 {
                Number::Float(f2) => {
                    let mut new_l : Vec<Option<Element>> = Vec::new();
                    for i in 0..l1.len() {
                        new_l.push(Some(ee_division(l1[i].as_ref().unwrap(),  &Element::Num(Number::Float(*f2)))));
                    }
                    Element::List(new_l)
                }
                _ => return Element::Error(String::from("MyError: you shouldn't see this. zzz")),
            }
        }
        _ => return Element::Error(String::from("TypeError:division with two illegal type!!")),
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
            Element::Error(e) =>  v_new.push(Element::Error(e)),
        }
        
    }
    v_new
}