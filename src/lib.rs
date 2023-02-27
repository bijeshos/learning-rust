mod variables;
mod data_types;
mod functions;
mod command_line_args;

pub fn run_examples() {
    variables::basic_variables();
    data_types::scalar_data_types();
    data_types::compound_data_types();
    //functions:private_function(); // this won't compile
    functions::function_examples();
}






