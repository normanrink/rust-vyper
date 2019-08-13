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
