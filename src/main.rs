use std::io::{stdin, stdout, Write, prelude::*, BufReader};
use std::fs::File;
use std::path::Path;
use eval::{Expr, to_value, eval};
use crate::model::expression::Expression;
use crate::model::expression_item::ExpressionItem;
use std::collections::HashMap;
use std::ops::Add;

mod model;

const OPERATOR_NOT: &str = "￢";
const OPERATOR_AND: &str = "∧";
const OPERATOR_OR: &str = "∨";
const OPERATOR_THEN: &str = "⇒";

fn main()
{
    let (expression_list, mut knowledge) = init();

    println!("\n\x1b[0;32m{}\x1b[0m\n", "Saraksti ir veiksmīgi izveidoti .");

    print_knowledge(&knowledge);

    println!("Izvelne.");
    println!("1 - Annulēt ZB");
    println!("2 - Tieša spriedumu ķēdīte");
    println!("3 - Apgriezta spriedumu ķēdīte");
    println!("4 - Pabeigt darbu");

    let action_options = ["1", "2", "3"];

    loop {
        println!("---------");
        print!("Ievadiet opciju: ");

        let mut s= String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string!");

        if action_options.contains(&s.trim()) {
            if s.trim() == "1" {
                knowledge = annul_knowledge();
            }
            else if s.trim() == "2" {
                knowledge = direct_judgment_chain(&expression_list, knowledge);
            }
            else if s.trim() == "3" {
                knowledge = reversed_judgment_chain(&expression_list, knowledge);
            }

            print_knowledge(&knowledge);
        }
        else if s.trim() == "4" {
            break;
        }
    }
}

fn init() -> (Vec<Expression>, HashMap<String, ExpressionItem>)
{
    println!("Ievadiet ZB faila ceļu.");
    println!("Piemērs: C:\\Users\\user\\sample_zb.txt");
    print!("Ceļa: ");

    let mut s= String::new();
    let _ = stdout().flush();

    stdin().read_line(&mut s).expect("Did not enter a correct string!");

    let path = Path::new(s.trim());
    let file = File::open(path).expect("File not found.");
    let reader = BufReader::new(file);

    let mut expression_list: Vec<Expression> = Vec::new();
    //let mut knowledge: HashMap<String, ExpressionItem> = HashMap::new();

    // C:\Users\shyri\OneDrive\Documents\zb.txt
    for line in reader.lines() {
        // because of 'line' borrow must be used 'l' variable
        let mut l = line.unwrap();

        l = l.replace(OPERATOR_NOT, "!")
            .replace(OPERATOR_AND, "&&")
            .replace(OPERATOR_OR, "||")
            .replace(OPERATOR_THEN, "==")
        ;

        if l.contains("==") {
            let mut split = l.split("==");
            let start = (split.next().unwrap(), split.next().unwrap());

            let expression = Expression::new(
                l.to_owned(),
                start.0.to_owned(),
                start.1.to_owned()
            );

            //knowledge = expression.insert_knowledge(knowledge);

            expression_list.push(expression);
        }
        else {
            println!("Nepareiza izteiksmes rindiņa failā : {}", l);
        }
    }

    (expression_list, HashMap::new())
}

fn print_knowledge(knowledge: &HashMap<String, ExpressionItem>)
{
    println!("\n--------------ZB---------------");

    if knowledge.len() > 0 {
        for (_, item) in knowledge.iter() {
            println!("{}: {}", item.desc, item.val.unwrap());
        }
    }
    else {
        println!("ZB ir tukša");
    }

    println!("-------------------------------\n");
}

fn annul_knowledge() -> HashMap<String, ExpressionItem>
{
    HashMap::new()
}

fn direct_judgment_chain(
    expression_list: &Vec<Expression>,
    mut knowledge: HashMap<String, ExpressionItem>
) -> HashMap<String, ExpressionItem>
{
    knowledge = input_knowledge(knowledge);

    // store identified expressions id from vector
    let mut identified_expressions: Vec<usize> = vec![];

    let len = expression_list.len();
    let mut current: usize = 0;

    while current < len {

        if identified_expressions.contains(&current) {
            current += 1;
            continue;
        }

        let expression = expression_list.get(current).unwrap();

        let mut split = expression.sentence.split("==");

        let mut l = split.next().unwrap().to_string();

        for val in expression.variables.iter() {
            if knowledge.get(val).is_some() {
                l = l.replace(val, knowledge.get(val).unwrap().val.unwrap().to_string().as_str());
            }
        }

        // evaluate result
        let res = eval(l.as_str());

        // if result is not error and as bool value is 'true'
        if res.is_ok() && res.unwrap().as_bool().unwrap() {
            let then_clause_variable =
                knowledge.get_mut(&expression.then_clause.desc.to_owned());

            if then_clause_variable.is_none() {
                // add new knowledge if not exists
                knowledge.insert(
                    expression.then_clause.desc.to_owned(),
                    ExpressionItem::new(
                        Some(expression.then_clause.val.unwrap()),
                        expression.then_clause.desc.to_owned()
                    )
                );
            }
            else {
                // change knowledge if exists
                then_clause_variable.unwrap().val = expression.then_clause.val;
            }

            identified_expressions.push(current);
            // annul current because of possibility to execute new expression with new knowledge
            current = 0;
        }
        else {
            current += 1;
        }
    }

    print_identified_expressions(expression_list, identified_expressions, false);

    knowledge
}

fn reversed_judgment_chain(
    expression_list: &Vec<Expression>,
    mut knowledge: HashMap<String, ExpressionItem>
) -> HashMap<String, ExpressionItem>
{
    knowledge = input_knowledge(knowledge);

    // store identified expressions id from vector
    let mut identified_expressions: Vec<usize> = vec![];

    let len = expression_list.len();
    let mut current: usize = 0;

    while current < len {

        if identified_expressions.contains(&current) {
            current += 1;
            continue;
        }

        let expression = expression_list.get(current).unwrap();

        let known_then = knowledge.get(expression.then_clause.desc.to_owned().as_str());

        // if exists then clause value in knowledge
        if known_then.is_some() {

            // if values equals
            if expression.then_clause.val.unwrap() == known_then.unwrap().val.unwrap() {

                // set all variables into knowledge
                for val in expression.variables.iter() {

                    let expression_item =
                        expression.logic_clause.iter().find(|&r| r.desc == val.to_owned());

                    // because variables also contains values from 'then' clause
                    if expression_item.is_none() {
                        continue;
                    }

                    let item = knowledge.get_mut(&val.to_owned());

                    if item.is_some() {
                        // change knowledge value that exists in map
                        item.unwrap().val = expression_item.unwrap().val;
                    }
                    else {
                        // insert new into knowledge map
                        knowledge.insert(
                            val.to_owned(),
                            ExpressionItem::new(
                                Some(expression_item.unwrap().val.unwrap()),
                                val.to_owned()
                            )
                        );
                    }
                }

                identified_expressions.push(current);
                // annul current because of possibility to execute new expression with new knowledge
                current = 0;
                continue;
            }
        }

        current += 1;
    }

    print_identified_expressions(expression_list, identified_expressions, true);

    knowledge
}

fn print_identified_expressions(
    expression_list: &Vec<Expression>,
    identified_expressions: Vec<usize>,
    reverse: bool
)
{
    println!();
    for ie in identified_expressions.into_iter() {

        let expression = expression_list.get(ie).unwrap();

        let mut result = expression.sentence.clone();

        if !reverse {
            result = result
                .replace("&&", "UN")
                .replace("!", "NĒ ")
                .replace("||", "VAI")
                .replace("==", "TAD")
                .replace("(", "")
                .replace(")", "")
            ;

            println!("#{}: JĀ {}", (ie + 1), result.to_owned());
        }
        else {
            result = result
                .replace("&&", "UN")
                .replace("!", "NĒ ")
                .replace("||", "VAI")
                .replace("(", "")
                .replace(")", "")
            ;

            let mut split = result.split("==");
            let logic_clause = split.next().unwrap().to_string();
            let then_clause = split.next().unwrap().to_string();

            println!("#{}: JĀ {} TAD {}", (ie + 1), then_clause, logic_clause);
        }

    }
}

fn input_knowledge(mut knowledge: HashMap<String, ExpressionItem>) -> HashMap<String, ExpressionItem>
{
    println!("Ievadiet zināšanas.");
    println!("Piemērs: val_1:true, var_2:false");
    print!("Ievade: ");

    let mut s= String::new();
    let _ = stdout().flush();

    stdin().read_line(&mut s).expect("Did not enter a correct string!");

    if s.trim() != "" {
        let input = s.split(",");

        for l in input.into_iter() {
            let mut split = l.split(":");
            let name = split.next().unwrap().trim().to_string();
            let val = split.next().unwrap().trim().to_string() == "true";

            if knowledge.contains_key(name.as_str()) {
                // change knowledge if exists
                let item = knowledge.get_mut(name.as_str());
                item.unwrap().val = Some(val);
            }
            else {
                // insert new knowledge if not exists
                knowledge.insert(
                    name.to_owned(),
                    ExpressionItem::new(Some(val), name.to_owned())
                );
            }
        }
    }

    knowledge
}
