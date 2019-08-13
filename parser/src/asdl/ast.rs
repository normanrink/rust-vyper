pub type Name<'a> = &'a str;

#[derive(Debug, PartialEq)]
pub struct Module<'a> {
    pub name: Name<'a>,
    pub definitions: Vec<Definition<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Definition<'a> {
    pub name: Name<'a>,
    pub value: Type<'a>,
}

#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    Product { fields: Fields<'a>, attributes: Fields<'a> },
    Sum { constructors: Vec<Constructor<'a>>, attributes: Fields<'a> },
}

#[derive(Debug, PartialEq)]
pub struct Constructor<'a> {
    pub name: Name<'a>,
    pub fields: Fields<'a>,
}

pub type Fields<'a> = Vec<Field<'a>>;

#[derive(Debug, PartialEq)]
pub struct Field<'a> {
    pub name: Name<'a>,
    pub definition: Name<'a>,
    pub count: Count,
}

#[derive(Debug, PartialEq)]
pub enum Count {
    One,
    ZeroOrOne,
    ZeroOrMany,
}
