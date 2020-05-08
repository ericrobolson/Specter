use super::*;

#[derive(Debug, Clone)]
pub struct Execute {}
impl Execute {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Parsable for Execute {
    fn parse(pair: pest::iterators::Pair<'_, Rule>) -> Self {
        if pair.as_rule() != Rule::execute {
            panic!("Not an Execute!");
        }

        return Self::new();
    }
}

impl Compilable for Execute {
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
