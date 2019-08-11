use pest::Parser;

#[derive(Parser)]
#[grammar = "vyper.pest"]
pub struct VyperParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_multiple_statements() {
        parses_to! {
            parser: VyperParser,
            input: &r###"
event Greet:#test
    name: bytes32
    foo: bytes32
    bar: bytes32
"###[1..],
            rule: Rule::event_decl,
            tokens: [
                event_decl(0, 64, [
                    symbol(6, 11),
                    event_prop(17, 30, [symbol(17, 21), symbol(23, 30)]),
                    event_prop(35, 47, [symbol(35, 38), symbol(40, 47)]),
                    event_prop(52, 64, [symbol(52, 55), symbol(57, 64)]),
                ]),
            ]
        }
    }

    #[test]
    fn parse_multiple_imports() {
        parses_to! {
            parser: VyperParser,
            input: &r###"
import foo
import bar
"###[1..],
            rule: Rule::module,
            tokens: [
                simp_import(0, 10, [module_path(7, 10, [symbol(7, 10)])]),
                simp_import(11, 21, [module_path(18, 21, [symbol(18, 21)])]),
            ]
        }

        parses_to! {
            parser: VyperParser,
            input: &r###"
from foo import (
    bar as b,
    baz as z,
)
"###[1..],
            rule: Rule::module,
            tokens: [
                from_import(0, 47, [
                    module_path(5, 9, [symbol(5, 8)]),
                    import_list(16, 47, [
                        symbol_as_alias(22, 30, [symbol(22, 25), as_alias(26, 30, [symbol(29, 30)])]),
                        symbol_as_alias(36, 44, [symbol(36, 39), as_alias(40, 44, [symbol(43, 44)])]),
                    ])
                ]),
            ]
        }
    }

    #[test]
    fn parse_good_imports() {
        let examples = vec![
            "import foo",
            "import foo as bar",
            "from foo import bar",
            "from foo import bar as baz",
            "from foo import ( bar as baz, david as boaty )",
            "from foo import (bar as baz, david as boaty)",
            "import .foo",
            "import ..foo",
        ];

        for e in examples {
            VyperParser::parse(Rule::import, e).unwrap();
        }
    }

    #[test]
    fn parse_bad_imports() {
        let examples = vec![
            "import 2foo",
            "import! 2foo",
            "import! 2foo",
            "import ...foo",
        ];

        for e in examples {
            let result = VyperParser::parse(Rule::import, e);

            if let Ok(_) = result {
                panic!("parsing unexpectedly succeeded: {:?}", result);
            }
        }
    }

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
