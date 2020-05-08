use super::*;

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
                    execute = Some(Execute::parse(i));
                }
                Rule::output_declaration => {
                    output = Some(Output::parse(i));
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

impl Compilable for MainNode {
    fn link(&self, data: &LanguageData) -> Self {
        self.clone()
    }

    fn compile(&self, target: TargetLanguage, data: &LanguageData) -> String {
        let mut generator = StringGenerator::new();

        generator.append("Main Node".to_string());

        return generator.to_string();
    }
    fn validate(&self, data: &LanguageData) {}
}
