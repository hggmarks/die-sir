#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Num(i128),
    Die,
    EOF,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    Power,
    DieRoll,
    Negative,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::Token::*;
        use self::OperPrec::*;
        match self {
            Add | Subtract => AddSub,
            Multiply | Divide => MulDiv,
            Caret => Power,
            Die => DieRoll,
            _ => DefaultZero
        }
    }

    
}
