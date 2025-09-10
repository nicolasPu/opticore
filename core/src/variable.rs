pub enum VarType {
    Integer,
    Continuous,
    Binary, // needed?
}

pub struct Var {
    var_type: VarType,
}

impl Var {
    pub fn new(var_type: VarType) -> Var {
        Self { var_type }
    }
}
