use super::*;

use super::rule_parser::{Execute, Input, Output};

#[derive(Debug, Clone)]
pub struct Node {
    pub identifier: String,
    pub input: Input,
    pub execute: Execute,
    pub output: Output,
}

impl Parsable for Node {
    fn parse(inner_pair: pest::iterators::Pair<'_, Rule>) -> Self {
        let mut identifier = String::new();

        let mut output = None;
        let mut input = None;
        let mut execute = None;

        if inner_pair.as_rule() != Rule::node {
            panic!("Not a node!");
        }

        let inner_pair = inner_pair.into_inner();
        for inner_pair in inner_pair {
            match inner_pair.as_rule() {
                Rule::node_declaration => {
                    let inner = inner_pair.into_inner();

                    let mut id = None;

                    for i in inner {
                        if i.as_rule() == Rule::identifier {
                            id = Some(i.as_str());
                        }
                    }

                    if id.is_none() {
                        panic!("Unable to get identifier for node!");
                    }
                    identifier = id.unwrap().to_string();
                }
                Rule::node_body => {
                    let body = inner_pair.into_inner();
                    for i in body {
                        let rule = i.as_rule();
                        match rule {
                            Rule::input_declaration => {
                                input = rule_parser::input_declaration(i);
                            }
                            Rule::execute => {
                                execute = rule_parser::execute_stmt(i);
                            }
                            Rule::output_declaration => {
                                output = rule_parser::output_declaration(i);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if input.is_none() {
            unimplemented!();
        }

        if output.is_none() {
            unimplemented!();
        }

        if execute.is_none() {
            unimplemented!();
        }

        return Self {
            identifier: identifier,
            input: input.unwrap(),
            execute: execute.unwrap(),
            output: output.unwrap(),
        };
    }
}
