use super::*;

pub fn execute() -> Ast {
    let relevant_files = locate_objects(FILE_TYPE);

    let mut program_data = vec![];
    for p in relevant_files {
        let path = p;
        let contents = fs::read_to_string(path.clone()).unwrap();
        program_data.push(lexer::lex_nioe(path, contents).unwrap());
    }

    return Ast::merge(program_data);
}

fn lex_nioe(path: String, contents: String) -> Result<Ast, pest::error::Error<Rule>> {
    let pairs = NioeParser::parse(Rule::program, &contents).unwrap_or_else(|e| panic!("{}", e));

    let ast = Ast::build(&path, pairs)?;

    return Ok(ast);
}
