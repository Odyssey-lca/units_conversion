use pest::Parser;
use units_conversion::{parser::parse_unit, unit::UNITS};

fn main() {
    let kwh = UNITS.get("MJ").unwrap();
    println!("{:?}", kwh.convert(UNITS.get("MJ").unwrap()).unwrap());
    println!("{:?}", parse_unit("MJ/Sm3"));
}
