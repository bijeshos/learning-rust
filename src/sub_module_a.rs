pub mod sub_module_aa; //declaring as pub sub module; definition is inside sub_module_a directory
mod sub_module_ab; //declaring as private sub module; definition is inside sub_module_a directory

use crate::common;

pub fn run_sub_module_a_examples() {
    common::print_topic_start_message("sub_module_a");
    sub_module_aa::run_sub_module_aa_examples();
    sub_module_ab::run_sub_module_ab_examples();
    common::print_topic_end_message("sub_module_a");
}