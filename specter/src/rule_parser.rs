use super::*;

#[derive(Debug, Clone)]
pub struct Input {}
pub fn input_declaration(pair: pest::iterators::Pair<'_, Rule>) -> Option<Input> {
    if pair.as_rule() != Rule::input_declaration {
        return None;
    }

    println!("Input decl: {}", pair);
    return None;
}

#[derive(Debug, Clone)]
pub struct Execute {}
pub fn execute_stmt(pair: pest::iterators::Pair<'_, Rule>) -> Option<Execute> {
    if pair.as_rule() != Rule::execute {
        return None;
    }

    println!("Execute decl: {}", pair);
    return None;
}

#[derive(Debug, Clone)]
pub struct Output {}
pub fn output_declaration(pair: pest::iterators::Pair<'_, Rule>) -> Option<Output> {
    if pair.as_rule() != Rule::output_declaration {
        return None;
    }
    println!("Output decl: {}", pair);
    return None;
}
