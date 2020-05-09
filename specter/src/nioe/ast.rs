use super::*;

use pest::Parser;

use inflector::Inflector;

fn unhandled_parse(scenario: &'static str, pair: &pest::iterators::Pair<'_, Rule>) {
    unimplemented!("Unimplemented scenario in '{}' parsing: {}", scenario, pair);
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub file: String,
    pub start: usize,
    pub end: usize,
}

impl Metadata {
    pub fn new(file: &String, rule: &pest::iterators::Pair<'_, Rule>) -> Self {
        let span = rule.as_span();
        return Self {
            file: file.clone(),
            start: span.start(),
            end: span.end(),
        };
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub id: String,
    pub metadata: Metadata,
}

impl Identifier {
    pub fn parse(
        path: &String,
        rule: &pest::iterators::Pair<'_, Rule>,
    ) -> Result<Self, pest::error::Error<Rule>> {
        let mut id = None;

        if rule.as_rule() == Rule::identifier {
            id = Some(rule.as_str());
        } else {
            for i in rule.clone().into_inner() {
                match i.as_rule() {
                    Rule::identifier => {
                        id = Some(i.as_str());
                    }
                    _ => {
                        unhandled_parse("identifier", &i);
                    }
                }
            }
        }

        return Ok(Self {
            id: id.unwrap().to_string(),
            metadata: Metadata::new(path, &rule),
        });
    }
}

#[derive(Debug, Clone)]
pub enum Inputs {
    Silent(Metadata),
    References(Metadata, Vec<Identifier>),
}

impl Inputs {
    pub fn parse(
        path: &String,
        rule: &pest::iterators::Pair<'_, Rule>,
    ) -> Result<Self, pest::error::Error<Rule>> {
        let mut references = vec![];

        match rule.as_rule() {
            Rule::input_declaration => {
                for inner in rule.clone().into_inner() {
                    match inner.as_rule() {
                        Rule::input_type => {}
                        Rule::silent_type => {
                            return Ok(Self::Silent(Metadata::new(path, &inner)));
                        }
                        Rule::input => {
                            for inner2 in inner.into_inner() {
                                let id = Identifier::parse(path, &inner2)?;
                                references.push(id);
                            }
                        }
                        _ => unhandled_parse("input_declaration", &inner),
                    }
                }
            }
            _ => unhandled_parse("inputs", rule),
        }

        return Ok(Self::References(Metadata::new(path, &rule), references));
    }
}

#[derive(Debug, Clone)]
pub enum Outputs {
    Silent(Metadata),
    References(Metadata, Vec<Identifier>),
}

impl Outputs {
    pub fn parse(
        path: &String,
        rule: &pest::iterators::Pair<'_, Rule>,
    ) -> Result<Self, pest::error::Error<Rule>> {
        let mut references = vec![];

        match rule.as_rule() {
            Rule::output_declaration => {
                for inner in rule.clone().into_inner() {
                    match inner.as_rule() {
                        Rule::output_type => {}
                        Rule::silent_type => {
                            return Ok(Self::Silent(Metadata::new(path, &inner)));
                        }
                        Rule::output_alias => {
                            for inner2 in inner.into_inner() {
                                match inner2.as_rule() {
                                    Rule::identifier => {
                                        let id = Identifier::parse(path, &inner2)?;
                                        references.push(id);
                                    }
                                    _ => unhandled_parse("output_alias", &inner2),
                                }
                            }
                        }
                        _ => unhandled_parse("output_declaration", &inner),
                    }
                }
            }
            _ => unhandled_parse("outputs", rule),
        }

        return Ok(Self::References(Metadata::new(path, &rule), references));
    }
}

#[derive(Debug, Clone)]
pub struct Execute {
    pub metadata: Metadata,
}

impl Execute {
    pub fn parse(
        path: &String,
        rule: &pest::iterators::Pair<'_, Rule>,
    ) -> Result<Self, pest::error::Error<Rule>> {
        println!("TODO: Execute parsing");

        return Ok(Self {
            metadata: Metadata::new(path, &rule),
        });
    }
}

#[derive(Debug, Clone)]
pub enum Ast {
    Program(Vec<Box<Ast>>),
    Node {
        id: Identifier,
        input: Inputs,
        output: Outputs,
        execute: Execute,
        metadata: Metadata,
    },
}

impl Ast {
    pub fn build(
        path: &String,
        data: pest::iterators::Pairs<'_, Rule>,
    ) -> Result<Ast, pest::error::Error<Rule>> {
        let mut program_data = Vec::<Box<Ast>>::new();

        for pair in data {
            match pair.as_rule() {
                Rule::program => {
                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::node => {
                                let mut id = None;
                                let mut inputs = None;
                                let mut outputs = None;
                                let mut execute = None;
                                let metadata = Metadata::new(path, &inner);

                                for i2 in inner.into_inner() {
                                    match i2.as_rule() {
                                        Rule::node_declaration => {
                                            id = Some(Identifier::parse(path, &i2)?);
                                        }
                                        Rule::node_body => {
                                            for i3 in i2.into_inner() {
                                                match i3.as_rule() {
                                                    Rule::input_declaration => {
                                                        inputs = Some(Inputs::parse(path, &i3)?);
                                                    }
                                                    Rule::output_declaration => {
                                                        outputs = Some(Outputs::parse(path, &i3)?);
                                                    }
                                                    Rule::execute_declaration => {
                                                        execute = Some(Execute::parse(path, &i3)?);
                                                    }
                                                    _ => unhandled_parse("node_body", &i3),
                                                }
                                            }
                                        }
                                        _ => unhandled_parse("node", &i2),
                                    }
                                }

                                let node = Ast::Node {
                                    id: id.unwrap(),
                                    input: inputs.unwrap(),
                                    output: outputs.unwrap(),
                                    execute: execute.unwrap(),
                                    metadata: metadata,
                                };

                                program_data.push(Box::new(node));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        return Ok(Ast::Program(program_data));
    }
}
