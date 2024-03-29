mod ownership;
mod borrowing;
mod lifetime;
mod lifetime_in_structure_implementation;
mod reference_counted_variables;
mod atomic_reference_counted_variables;
mod mutex_thread_safe;

fn main() {
    // ownership::main();
    // borrowing::main();
    // lifetime::main();
    // lifetime_in_structure_implementation::main();
    // reference_counted_variables::main();
    // atomic_reference_counted_variables::main();
    mutex_thread_safe::main();
}
