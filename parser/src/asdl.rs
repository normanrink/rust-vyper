use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0},
    sequence::{terminated, tuple},
};

type Name<'a> = &'a str;

#[derive(Debug, PartialEq)]
pub struct Module<'a> {
    name: Name<'a>,
    definitions: Vec<Definition<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Definition<'a> {
    name: Name<'a>,
    value: Type<'a>,
}

#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    Product { fields: Fields<'a>, attributes: Fields<'a> },
    Sum { constructors: Vec<Constructor<'a>>, attributes: Fields<'a> },
}

#[derive(Debug, PartialEq)]
pub struct Constructor<'a> {
    name: Name<'a>,
    fields: Fields<'a>,
}

type Fields<'a> = Vec<Field<'a>>;

#[derive(Debug, PartialEq)]
pub struct Field<'a> {
    name: Name<'a>,
    definition: Name<'a>,
    count: Count,
}

#[derive(Debug, PartialEq)]
pub enum Count {
    One,
    ZeroOrOne,
    ZeroOrMany,
}

fn is_symbol_start(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

fn is_symbol_char(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic() || c.is_digit(10)
}

fn symbol(i: &str) -> IResult<&str, &str> {
    take_while1(is_symbol_start)(i)?;
    take_while1(is_symbol_char)(i)
}

fn symbol_token(i: &str) -> IResult<&str, &str> {
    terminated(symbol, multispace0)(i)
}

fn module(i: &str) -> IResult<&str, Module> {
    let (i, _) = tuple((multispace0, tag("module"), multispace0))(i)?;
    let (i, name) = symbol_token(i)?;
    let (i, _) = terminated(char('{'), multispace0)(i)?;
    let (i, definitions) = (i, vec![]);
    let (i, _) = terminated(char('}'), multispace0)(i)?;

    Ok((i, Module {
        name: name,
        definitions: definitions,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module() {
        assert_eq!(
            module(r###"
module Python
{
}
"###),
            Ok(("", Module {
                name: "Python",
                definitions: vec![],
            })),
        );
    }
}
