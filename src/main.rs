
mod ch1_basics;
mod ch2_datatypes;
mod ch3_control_flow;
mod ch4_ownership;
mod ch5_struct;
mod ch6_enum;
mod ch8_collections;
mod ch9_errors;
mod ch10_generics;

mod gotchas;

fn main() {
    
    ch1_basics::variable();
    ch1_basics::scope();

    ch2_datatypes::numeric();
    ch2_datatypes::tuple();
    ch2_datatypes::array();

    ch3_control_flow::function_call();
    ch3_control_flow::if_syntax();
    ch3_control_flow::loop_syntax();
    ch3_control_flow::for_and_while();

    ch4_ownership::shallow_transfer();
    ch4_ownership::deep_copy();
    ch4_ownership::stack_copy();
    ch4_ownership::ownership_through_function();
    ch4_ownership::immutable_references();
    ch4_ownership::mutable_references();
    ch4_ownership::switch_between_references();
    ch4_ownership::slice();
    ch4_ownership::slice_ownership();

    ch5_struct::struct_syntax();
    ch5_struct::special_struct();
    ch5_struct::struct_method();

    ch6_enum::enum_syntax();
    ch6_enum::match_syntax();
    ch6_enum::iflet_syntax();

    ch8_collections::vector_syntax();
    ch8_collections::string_syntax();
    ch8_collections::hashmap_syntax();

    ch9_errors::panic_syntax();
    ch9_errors::match_on_error();
    ch9_errors::unwrap_expect();
    ch9_errors::short_match();
    ch9_errors::err_propagation();

    ch10_generics::generic_type();
    ch10_generics::trait_for_generic();
    ch10_generics::lifetime();

}
