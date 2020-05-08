use super::*;

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
                                input = Some(Input::parse(i));
                            }
                            Rule::execute => {
                                execute = Some(Execute::parse(i));
                            }
                            Rule::output_declaration => {
                                output = Some(Output::parse(i));
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
            identifier: identifier.to_lowercase(),
            input: input.unwrap(),
            execute: execute.unwrap(),
            output: output.unwrap(),
        };
    }
}

impl Compilable for Node {
    fn link(&self, data: &LanguageData) -> Self {
        let mut linked = self.clone();

        linked.input = linked.input.link(data);

        return linked;
    }

    fn compile(&self, target: TargetLanguage, data: &LanguageData) -> String {
        let mut generator = StringGenerator::new();

        generator.append(format!("node: {}", self.identifier));

        return generator.to_string();
    }

    fn validate(&self, data: &LanguageData) {
        // TODO: validate self + all inputs/fields
    }
}
