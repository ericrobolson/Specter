use super::*;

#[derive(Debug, Clone)]
pub struct Input {
    pub identifiers: Vec<Identifier>,
}

impl Input {
    pub fn new(identifiers: Vec<Identifier>) -> Self {
        return Self {
            identifiers: identifiers,
        };
    }
}

impl Parsable for Input {
    fn parse(pair: pest::iterators::Pair<'_, Rule>) -> Self {
        if pair.as_rule() != Rule::input_declaration {
            panic!("Not an input!");
        }

        let mut identifiers = vec![];

        for i in pair.into_inner() {
            match i.as_rule() {
                Rule::input => {
                    for i2 in i.into_inner() {
                        match i2.as_rule() {
                            Rule::output_alias => {
                                identifiers.push(Identifier::new(i2.as_str().to_string()));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {
                    panic!("Not handled!");
                }
            }
        }

        return Self::new(identifiers);
    }
}

impl Compilable for Input {
    fn link(&self, data: &LanguageData) -> Self {
        let mut linked = self.clone();

        // For each input, link it to the proper type
        let outputs = data.outputs();

        for identifier in linked.identifiers.iter_mut() {
            let found_output = outputs.get(&identifier.id);
            if found_output.is_none() {
                panic!("Unable to link input '{}' to output!", identifier.id);
            }

            identifier.itype = found_output.unwrap().itype.clone();
        }

        return linked;
    }

    fn validate(&self, data: &LanguageData) {
        for input in self.identifiers.iter() {
            if input.itype.is_none() {
                panic!("Input '{}': Unable to associate type!", input.id);
            }

            // Check to see that the output exists
        }
    }

    fn compile(&self, _: TargetLanguage, _: &LanguageData) -> std::string::String {
        todo!()
    }
}
