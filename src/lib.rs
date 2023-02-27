mod variables;
mod data_types;
mod functions;
mod command_line_args;

pub fn run_examples() {
    variables::variable_examples();
    data_types::scalar_data_type_examples();
    data_types::compound_data_type_examples();

    //functions:private_function(); // the below won't compile
    functions::function_examples();
}






