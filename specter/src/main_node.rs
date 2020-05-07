use super::*;

use super::rule_parser::{Execute, Output};

#[derive(Debug, Clone)]
pub struct MainNode {
    pub execute: Execute,
    pub output: Output,
}

impl Parsable for MainNode {
    fn parse(inner_pair: pest::iterators::Pair<'_, Rule>) -> Self {
        let mut output = None;
        let mut execute = None;

        if inner_pair.as_rule() != Rule::main {
            panic!("Not a main!");
        }

        let inner_pair = inner_pair.into_inner();
        for i in inner_pair {
            match i.as_rule() {
                Rule::execute => {
                    execute = rule_parser::execute_stmt(i);
                }
                Rule::output_declaration => {
                    output = rule_parser::output_declaration(i);
                }
                _ => {}
            }
        }

        if output.is_none() {
            unimplemented!();
        }

        if execute.is_none() {
            unimplemented!();
        }

        return Self {
            execute: execute.unwrap(),
            output: output.unwrap(),
        };
    }
}
