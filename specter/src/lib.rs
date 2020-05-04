extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

mod object_locator;

use std::fs;

#[derive(Parser)]
#[grammar = "specter.pest"]
struct MyParser;

/// Build the Specter files
pub fn build() {
    let objects = object_locator::locate_objects();

    for object in objects {
        if object.path.is_none() {
            continue;
        }

        let path = object.path.unwrap();

        println!("???????????????????????????????????????????????????");
        println!("Reading file: {}", path);
        println!("???????????????????????????????????????????????????");

        let contents = fs::read_to_string(path).unwrap();

        println!("Contents: {}", contents);

        let res = parse_specter(contents);

        if res.is_err() {
            println!("{:?}", res);
        }
    }
}

fn parse_specter(contents: String) -> Result<(), pest::error::Error<Rule>> {
    let pairs = MyParser::parse(Rule::specter, &contents)?;

    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {:?}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::component => println!("Component: {}", inner_pair.as_str()),
                Rule::system => println!("System: {}", inner_pair.as_str()),
                Rule::identifier => println!("Identifier:  {}", inner_pair.as_str()),
                Rule::stype => println!("STYPE:   {}", inner_pair.as_str()),
                _ => println!("dafu? {:?}", inner_pair.as_str()),
            };
        }
    }

    Ok(())
}
