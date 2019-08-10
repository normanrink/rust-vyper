use pest::Parser;

use rust_vyper::parser::{VyperParser, Rule};

fn main() {
    let result = VyperParser::parse(Rule::symbol, "foo");
    println!("{:?}", result);
}
