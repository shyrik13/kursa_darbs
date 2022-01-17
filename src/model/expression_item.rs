
#[derive(Debug)]
pub struct ExpressionItem {
    pub val: Option<bool>,
    pub desc: String
}

impl ExpressionItem {
    pub fn new(val: Option<bool>, desc: String) -> Self
    {
        Self { val, desc }
    }
}