pub fn inputless_executions() -> Vec<String> {
    vec![]
}

pub fn executions() -> Vec<String> {
    vec![std_cout_node_ref.to_string(), std_cin_node_ref.to_string()]
}

const std_cin_node_ref: &'static str = "node_std_cin";
const std_cin_struct_ref: &'static str = "std_cin";

const std_cout_node_ref: &'static str = "node_std_cout";
const std_cout_struct_ref: &'static str = "std_cout";

pub fn node_declarations() -> Vec<(String, String)> {
    return vec![
        (
            String::from(std_cin_node_ref),
            String::from(std_cin_struct_ref),
        ),
        (
            String::from(std_cout_node_ref),
            String::from(std_cout_struct_ref),
        ),
    ];
}
