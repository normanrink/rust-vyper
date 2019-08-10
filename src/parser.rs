use pest::Parser;

#[derive(Parser)]
#[grammar = "vyper.pest"]
pub struct VyperParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_good_symbols() {
        let examples = vec![
            "foo",
            "_foo",
            "camelCase",
            "snake_case",
            "TitleCase",
            "__dunder",
        ];

        for e in examples {
            VyperParser::parse(Rule::symbol, e).unwrap();
        }
    }

    #[test]
    fn parse_bad_symbols() {
        let examples = vec![
            "3foo",
            "",
            "4_this_symbol",
        ];

        for e in examples {
            let result = VyperParser::parse(Rule::symbol, e);
            
            if let Ok(_) = result {
                panic!("parsing unexpectedly succeeded: {:?}", result);
            }
        }
    }
}
