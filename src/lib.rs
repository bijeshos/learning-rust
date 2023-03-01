mod variables;
mod data_types;
mod functions;
mod control_flows;
mod memory_management;
mod slices;
mod structs;
mod enums;
mod pattern_matching;
mod collections;
mod error_handling;
mod generics;
mod io;
mod smart_pointers;
mod concurrency;
mod unsafe_pkg;
mod command_line_args;
mod common;
mod vectors;
mod sub_module_a;

pub fn run_examples() {
    variables::run_variable_examples();
    data_types::run_scalar_data_type_examples();
    data_types::run_compound_data_type_examples();

    // the below won't compile
    //functions:private_function();

    functions::run_function_examples();
    control_flows::run_control_flow_examples();
    vectors::run_vector_examples();

    memory_management::run_memory_management_examples();
    slices::run_slices_examples();
    structs::run_structs_examples();
    enums::run_enums_examples();
    pattern_matching::run_pattern_matching_examples();
    collections::run_collections_examples();
    error_handling::run_error_handling_examples();
    generics::run_generics_examples();
    io::run_io_examples();
    smart_pointers::run_smart_pointers_examples();
    concurrency::run_concurrency_examples();
    unsafe_pkg::run_unsafe_pkg_examples();

    sub_module_a::run_sub_module_a_examples();
    sub_module_a::sub_module_aa::run_sub_module_aa_examples();
    //sub_module_a::sub_module_ab::run_sub_module_ab_examples(); //This is not accessible since sub_module_ab is private
}






