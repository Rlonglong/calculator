use std::collections::HashMap;
extern crate lazy_static;
use lazy_static::lazy_static;

use super::arithmetic::Element;

lazy_static! {
    pub static ref VARIABLE: HashMap<String, Element> = HashMap::new();
}