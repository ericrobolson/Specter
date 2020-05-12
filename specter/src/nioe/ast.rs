use super::*;
use core::fmt::Display;

fn unhandled_parse(scenario: &'static str, pair: &pest::iterators::Pair<'_, Rule>) {
    unimplemented!("Unimplemented scenario in '{}' parsing: {}", scenario, pair);
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub file: String,
    pub start: usize,
    pub end: usize,
}

impl Display for Metadata {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "File: {}, position: {}", self.file, self.start)
    }
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
    pub fn rust(&self) -> String {
        return format!("{}", self.id);
    }

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
            id: id.unwrap().to_string().to_lowercase(),
            metadata: Metadata::new(path, &rule),
        });
    }
}

#[derive(Debug, Clone)]
pub enum Inputs {
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
pub enum PrimitiveTypes {
    Number(f32),
    Boolean(bool),
    Str(String),
}

#[derive(Debug, Clone)]
pub struct Primitive {
    pub value: PrimitiveTypes,
    pub metadata: Metadata,
}

impl Primitive {
    pub fn rust(&self) -> String {
        return match &self.value {
            PrimitiveTypes::Number(n) => format!("{}", n),
            PrimitiveTypes::Boolean(b) => format!("{}", b),
            PrimitiveTypes::Str(s) => s.clone(),
        };
    }
    pub fn parse(
        path: &String,
        rule: &pest::iterators::Pair<'_, Rule>,
    ) -> Result<Self, pest::error::Error<Rule>> {
        let mut value = None;

        if rule.as_rule() != Rule::primitive {
            unimplemented!("Attempting to parse a primitive from a non-primitive rule!");
        } else {
            for i in rule.clone().into_inner() {
                match i.as_rule() {
                    Rule::number => {
                        println!("TODO numbers: value: {}", i.as_str());
                        unhandled_parse("primitive number", &i);
                    }
                    Rule::string => {
                        value = Some(PrimitiveTypes::Str(i.as_str().to_string()));
                    }
                    Rule::bool => {
                        match i.as_str().to_lowercase().as_str() {
                            "true" => {
                                value = Some(PrimitiveTypes::Boolean(true));
                            }
                            "false" => {
                                value = Some(PrimitiveTypes::Boolean(false));
                            }
                            _ => {
                                unhandled_parse("primitive bool", &i);
                            }
                        }
                        println!("TODO bools: value: {}", i.as_str());
                    }
                    _ => {
                        unhandled_parse("primitive", &i);
                    }
                }
            }
        }

        return Ok(Self {
            value: value.unwrap(),
            metadata: Metadata::new(path, &rule),
        });
    }
}

#[derive(Debug, Clone)]
pub struct Signal {
    pub id: Identifier,
    pub message_value: Primitive,
    pub metadata: Metadata,
}

impl Signal {
    pub fn rust(&self) -> String {
        return format!("{}", self.id.rust());
    }

    pub fn parse(
        path: &String,
        rule: &pest::iterators::Pair<'_, Rule>,
    ) -> Result<Self, pest::error::Error<Rule>> {
        let mut id = None;
        let mut value = None;

        if rule.as_rule() != Rule::signal {
            unimplemented!("Attempting to parse a Signal from a non-Signal rule!");
        } else {
            for i in rule.clone().into_inner() {
                match i.as_rule() {
                    Rule::signal_type => {}
                    Rule::signal_output => {
                        for i2 in i.into_inner() {
                            match i2.as_rule() {
                                Rule::output_alias => {
                                    id = Some(Identifier::parse(&path, &i2).unwrap());
                                }
                                Rule::primitive => {
                                    value = Some(Primitive::parse(&path, &i2).unwrap());
                                }
                                _ => unhandled_parse("signal_output", &i2),
                            }
                        }
                    }
                    _ => {
                        unhandled_parse("identifier", &i);
                    }
                }
            }
        }

        return Ok(Self {
            id: id.unwrap(),
            message_value: value.unwrap(),
            metadata: Metadata::new(path, &rule),
        });
    }
}

#[derive(Debug, Clone)]
pub struct Execute {
    pub metadata: Metadata,
    pub statements: Vec<ExecuteStatements>,
}

#[derive(Debug, Clone)]
pub enum ExecuteStatements {
    Expr,
    Signal(Signal),
}

impl Execute {
    pub fn parse(
        path: &String,
        rule: &pest::iterators::Pair<'_, Rule>,
    ) -> Result<Self, pest::error::Error<Rule>> {
        let mut statements = vec![];
        match rule.as_rule() {
            Rule::execute_declaration => {
                for inner in rule.clone().into_inner() {
                    match inner.as_rule() {
                        Rule::execute_type => {}
                        Rule::statement => {
                            for inner2 in inner.into_inner() {
                                match inner2.as_rule() {
                                    Rule::expr => {
                                        unhandled_parse("execute: expr", &inner2);
                                    }
                                    Rule::signal => {
                                        statements.push(ExecuteStatements::Signal(
                                            Signal::parse(&path, &inner2).unwrap(),
                                        ));
                                    }
                                    Rule::end_statement => {}
                                    _ => unhandled_parse("execute statement", &inner2),
                                }
                            }
                        }
                        _ => unhandled_parse("execute_declaration", &inner),
                    }
                }
            }
            _ => unhandled_parse("execute", rule),
        }

        return Ok(Self {
            metadata: Metadata::new(path, &rule),
            statements: statements,
        });
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Identifier,
    pub input: Option<Inputs>,
    pub output: Outputs,
    pub execute: Execute,
    pub metadata: Metadata,
    pub injected_include: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Ast {
    Program(Vec<Ast>),
    Node(Node),
}

impl Ast {
    /// Given a collection of Asts, merge them
    pub fn merge(data: Vec<Ast>) -> Ast {
        let mut nodes = vec![];

        for ast in data {
            match ast.clone() {
                Self::Node(n) => {
                    nodes.push(ast);
                }
                Self::Program(_) => {
                    nodes.append(&mut Self::expand_nodes(&ast));
                }
            }
        }

        return Ast::Program(nodes);
    }

    /// Given an ast, expand it
    pub fn expand_nodes(ast: &Self) -> Vec<Self> {
        let mut expanded = vec![];

        match ast.clone() {
            Self::Node(n) => {
                expanded.push(ast.clone());
            }
            Self::Program(trees) => {
                for tree in trees {
                    let mut e = Self::expand_nodes(&tree);
                    expanded.append(&mut e);
                }
            }
        }

        return expanded;
    }

    pub fn build(
        path: &String,
        data: pest::iterators::Pairs<'_, Rule>,
    ) -> Result<Ast, pest::error::Error<Rule>> {
        let mut program_data = Vec::<Ast>::new();

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
                                let mut injected_include = None;
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
                                                    //Rule::injected_include => {
                                                    //    injected_include =
                                                    //        Some(i3.as_str().to_string());
                                                    //}
                                                    _ => unhandled_parse("node_body", &i3),
                                                }
                                            }
                                        }
                                        _ => unhandled_parse("node", &i2),
                                    }
                                }

                                let node = Ast::Node(Node {
                                    id: id.unwrap(),
                                    input: inputs,
                                    output: outputs.unwrap(),
                                    execute: execute.unwrap(),
                                    metadata: metadata,
                                    injected_include: injected_include,
                                });

                                program_data.push(node);
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
