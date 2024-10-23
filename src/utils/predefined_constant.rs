use super::arithmetic::{Element, Number};

pub const PI : f64 = std::f64::consts::PI;
pub const E : f64 = std::f64::consts::E;

pub fn is_constant(s : &str) -> bool{
    if s == "pi" || s == "e" {
        return true;
    }
    else {
        return false;
    }
}

pub fn assignment_constant(s : &str) -> Element{
    match s {
        "pi" => Element::Num(Number::Float(PI)),
        "e" => Element::Num(Number::Float(E)),
        _ => Element::Error(String::from("MyError: you shouldn't see this. zzz")),
    }
}