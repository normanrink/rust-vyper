pub type Name = String;
pub type Type = String;

pub struct Module {
    pub body: Vec<ModuleStmt>,
}

pub enum ModuleStmt {
    EventDef {
        name: Name,
        fields: Vec<EventField>,
    },
    InterfaceDef {
        name: Name,
        fields: Vec<InterfaceField>,
        methods: Vec<InterfaceMethodDef>,
    },
    ContractDef {
        name: Name,
        fields: Vec<ContractField>,
        methods: Vec<ContractMethodDef>,
    },
}

pub struct EventField {
    pub name: Name,
    pub typ: Type,
    pub indexed: bool,
}

pub struct InterfaceField {
    pub name: Name,
    pub typ: Type,
}

pub struct ContractField {
    pub name: Name,
    pub typ: Type,
    pub public: bool,
}

pub struct InterfaceMethodDef {
    pub name: Name,
    pub args: Vec<Arg>,
    pub returns: Option<Type>,
}

pub struct ContractMethodDef {
    pub name: Name,
    pub args: Vec<Arg>,
    pub return_type: Option<Type>,
    pub body: Option<Stmt>,
}

pub struct Arg {
    pub name: Name,
    pub typ: Type,
}

pub enum MethodProps {
    Constant,
    Modifying,
    Public,
    Private,
}

pub enum Stmt {
    EmitEvent {
        event_name: Name,
        args: Vec<Expr>,
    },
}

pub enum Expr {

}
