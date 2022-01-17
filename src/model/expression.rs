use crate::model::expression_item::ExpressionItem;
use std::collections::HashMap;
use std::option::Option::Some;

#[derive(Debug)]
pub struct Expression {
    pub sentence: String,
    pub variables: Vec<String>,
    pub logic_clause: Vec<ExpressionItem>,
    pub then_clause: ExpressionItem
}

impl Expression {

    pub fn new(sentence: String, logic_clause_val: String, then_clause_val: String) -> Self
    {
        let mut variables: Vec<String> = Vec::new();
        let mut logic_clause: Vec<ExpressionItem> = Vec::new();

        let ignore_items = ["(", ")", "&&", "||"];

        // resolve logic clause and all variables
        for item in logic_clause_val.split_whitespace() {

            let add_to_variables = !ignore_items.contains(&item.trim());

            let mut desc = item.trim().to_owned();
            let mut val = None;

            if add_to_variables {
                desc = item.replace("!", "").replace(" ", "");
                val = Some(!item.contains("!"));

                if !variables.contains(&desc.to_owned()) {
                    variables.push(desc.to_owned());
                }
            }

            logic_clause.push(
                ExpressionItem::new(
                    val,
                    desc.to_owned()
                )
            );
        }

        let desc = then_clause_val.replace("!", "").replace(" ", "").trim().to_owned();

        let then_clause = ExpressionItem::new(
            Some(!then_clause_val.contains("!")),
            desc.to_owned()
        );

        if !variables.contains(&desc.to_owned()) {
            variables.push(desc.to_owned());
        }

        Self { sentence, logic_clause, variables, then_clause }
    }
}
