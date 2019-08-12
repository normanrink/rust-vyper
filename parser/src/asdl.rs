//use nom::{
//    IResult,
//    bytes::complete::{tag, take_while1},
//    character::complete::{char, multispace0, one_of},
//    multi::{many0, separated_nonempty_list},
//    sequence::{terminated, tuple},
//};

//type Name<'a> = &'a str;

//#[derive(Debug, PartialEq)]
//pub struct Module<'a> {
//    name: Name<'a>,
//    definitions: Vec<Definition<'a>>,
//}

//#[derive(Debug, PartialEq)]
//pub struct Definition<'a> {
//    name: Name<'a>,
//    value: Type<'a>,
//}

//#[derive(Debug, PartialEq)]
//pub enum Type<'a> {
//    Product { fields: Fields<'a>, attributes: Fields<'a> },
//    Sum { constructors: Vec<Constructor<'a>>, attributes: Fields<'a> },
//}

//#[derive(Debug, PartialEq)]
//pub struct Constructor<'a> {
//    name: Name<'a>,
//    fields: Fields<'a>,
//}

//type Fields<'a> = Vec<Field<'a>>;

//#[derive(Debug, PartialEq)]
//pub struct Field<'a> {
//    name: Name<'a>,
//    definition: Name<'a>,
//    count: Count,
//}

//#[derive(Debug, PartialEq)]
//pub enum Count {
//    One,
//    ZeroOrOne,
//    ZeroOrMany,
//}

//fn is_symbol_start(c: char) -> bool {
//    c == '_' || c.is_ascii_alphabetic()
//}

//fn is_symbol_char(c: char) -> bool {
//    c == '_' || c.is_ascii_alphabetic() || c.is_digit(10)
//}

//fn symbol(i: &str) -> IResult<&str, &str> {
//    take_while1(is_symbol_start)(i)?;
//    take_while1(is_symbol_char)(i)
//}

//fn symbol_token(i: &str) -> IResult<&str, &str> {
//    terminated(symbol, multispace0)(i)
//}

//fn char_token(c: char) -> impl Fn(&str) -> IResult<&str, char> {
//    move |i: &str| terminated(char(c), multispace0)(i)
//}

//fn tag_token(t: &'static str) -> impl Fn(&str) -> IResult<&str, &str> {
//    move |i: &str| terminated(tag(t), multispace0)(i)
//}

//fn parse_module(i: &str) -> IResult<&str, Module> {
//    let (i, _) = tuple((multispace0, tag("module"), multispace0))(i)?;
//    let (i, name) = symbol_token(i)?;
//    let (i, _) = char_token('{')(i)?;
//    let (i, definitions) = many0(parse_definition)(i)?;
//    let (i, _) = char_token('}')(i)?;

//    Ok((i, Module {
//        name: name,
//        definitions: definitions,
//    }))
//}

//fn parse_definition(i: &str) -> IResult<&str, Definition> {
//    let (i, name) = symbol_token(i)?;
//    let (i, _) = char_token('=')(i)?;
//    let (i, value) = parse_product(i)?;

//    Ok((i, Definition {
//        name: name,
//        value: value,
//    }))
//}

//fn parse_product(i: &str) -> IResult<&str, Type> {
//    let (i, fields) = parse_fields(i)?;
//    //let (i, _) = tag_token("attributes")(i)?;
//    let (i, attributes) = (i, vec![]);

//    Ok((i, Type::Product {
//        fields: fields,
//        attributes: attributes,
//    }))
//}

//fn parse_fields(i: &str) -> IResult<&str, Fields> {
//    let (i, _) = char_token('(')(i)?;
//    let (i, fields) = separated_nonempty_list(tag(", "), parse_field)(i)?;
//    let (i, _) = char_token(')')(i)?;

//    Ok((i, fields))
//}

//fn parse_field(i: &str) -> IResult<&str, Field> {
//    let (i, definition) = symbol_token(i)?;
//    let (i, count) = one_of("*?")(i)?;
//    let (i, name) = symbol_token(i)?;

//    Ok((i, Field {
//        name: name,
//        definition: definition,
//        count: Count::One,
//    }))
//}

//#[cfg(test)]
//mod tests {
//    use super::*;

//    #[test]
//    fn test_module() {
//        assert_eq!(
//            parse_module(r###"
//module Python
//{
//    mod = (int col, int row)
//}
//"###),
//            Ok(("", Module {
//                name: "Python",
//                definitions: vec![
//                    Definition {
//                        name: "mod",
//                        value: Type::Product { fields: vec![], attributes: vec![] },
//                    }
//                ],
//            })),
//        );
//    }
//}
