use super::*;

fn already_defined(
    obj_type: &'static str,
    id: &String,
    current_metadata: &ast::Metadata,
    existing_metadata: &ast::Metadata,
) {
    panic!(
        "{} with id '{}' at {} has already been defined at {}!",
        obj_type, id, current_metadata, existing_metadata
    );
}

pub fn execute(ast: &Ast) {
    let mut node_map: HashMap<String, ast::Node> = HashMap::new();
    let mut output_map: HashMap<String, ast::Identifier> = HashMap::new();

    let mut inputs = vec![];

    // First pass validation for nodes + outputs
    for node in Ast::expand_nodes(ast) {
        match node {
            Ast::Node(n) => {
                // Verify node ids
                {
                    let id = &n.id.id;
                    let existing = node_map.get(id);

                    let id = id.to_string();

                    if existing.is_some() {
                        let existing = existing.unwrap();
                        already_defined("node", &id, &n.metadata, &existing.metadata);
                    }

                    node_map.insert(id.clone(), n.clone());
                }

                // Save inputs for second pass to validate after all outputs
                if n.input.is_some() {
                    inputs.push(n.input.unwrap());
                }
                // Verify that there's no duplicate outputs
                match n.output {
                    ast::Outputs::References(metadata, references) => {
                        for reference in &references {
                            let existing = output_map.get(&reference.id);
                            if existing.is_some() {
                                let existing = existing.unwrap();
                                //TODO: verify that the types are the same
                                break;
                                already_defined(
                                    "output",
                                    &reference.id,
                                    &reference.metadata,
                                    &existing.metadata,
                                );
                            }

                            output_map.insert(reference.id.clone(), reference.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Second pass validation for inputs
    for input in inputs {
        match input {
            ast::Inputs::References(metadata, references) => {
                if references.is_empty() {
                    panic!(
                        "At least one input is required! Otherwise use the silent '_' input type."
                    )
                }

                for reference in references {
                    let matching_output = output_map.get(&reference.id);
                    if matching_output.is_none() {
                        panic!(
                            "Unable to properly link input with id '{}' to output at {}!",
                            reference.id, reference.metadata
                        );
                    }
                }
            }
        }
    }
}
