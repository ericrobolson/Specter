use super::*;

#[derive(Debug, Clone)]
pub struct Output {
    pub identifiers: Vec<Identifier>,
}

impl Output {
    pub fn new(identifiers: Vec<Identifier>) -> Self {
        return Self {
            identifiers: identifiers,
        };
    }
}

impl Parsable for Output {
    fn parse(pair: pest::iterators::Pair<'_, Rule>) -> Self {
        if pair.as_rule() != Rule::output_declaration {
            panic!("Not an Output!");
        }

        let mut identifiers = vec![];

        for i in pair.into_inner() {
            match i.as_rule() {
                Rule::output_assignment => {
                    for i2 in i.into_inner() {
                        match i2.as_rule() {
                            Rule::output_definition => {
                                println!("o: {}", i2.as_str());
                            }
                            Rule::identifier => {
                                //TODO: handle assignment; for now just populating the output type as the assignment for debugging
                            }
                            _ => {}
                        }

                        //match i2.as_rule() {
                        //    Rule::output_alias => {
                        //        identifiers.push(Identifier::new(i2.as_str().to_string()));
                        //    }
                        //    _ => {}
                        //}
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

impl MetaDatable for Output {
    fn metadata(&self) -> MetaData {
        todo!()
    }
}

impl Compilable for Output {
    fn link(&self, data: &LanguageData) -> Self {
        self.clone()
    }

    fn validate(&self, _: &LanguageData) {
        todo!()
    }
    fn compile(&self, _: TargetLanguage, _: &LanguageData) -> std::string::String {
        todo!()
    }
}
