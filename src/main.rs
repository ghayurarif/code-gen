use std::fs;
mod generate_code;
mod parser;
fn main() {
    let result = fs::read_to_string("input.json").unwrap();
    let res = parser::parse(result).unwrap();
    generate_code::generate(res[1].clone(), res[0].clone()).unwrap();
}
