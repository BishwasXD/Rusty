pub mod collections {
    pub mod hashmap;
    pub mod strings;
    pub mod vectors;
}
pub mod error_handling {
    pub mod Errors;
    pub mod recoverable_errors;
    pub mod unrecoverable_errors;
}
pub mod smart_pointers{
    pub mod smart_pointers;
    pub mod box_pointers;
    pub mod reference_counted;
}
pub mod data_types {
    //     pub mod strings;
    //     pub mod numbers;
    pub mod array;
}
mod loops {
    pub mod loops;
    // pub mod fibonacci_loops;
}
mod ownership {
    pub mod ownership;
}
mod macros{
    pub mod macros;
}
mod structs {
    pub mod structures;
}
mod asynchronous{
    pub mod asynchronous;
}
mod enums {
    pub mod enums;
}
mod match_basics {
    pub mod match_basics;
}
mod simple_programs {
    // pub mod two_sum;
    // pub mod factorial;
    // pub mod leetcode11;
    pub mod build_pattern;
    pub mod compression;
}
mod check_mod {
    pub fn check() {
        println!("checking the working of a inline module.");
    }
}

mod writing_test {
    pub mod tests;
    pub mod tests2;
}
mod generic_traits_lifetime {
    pub mod generics;
}
mod concurrencies{
    pub mod concurrency;
}
fn main() {
    // simple_programs::fibonacii::fibonacci();
    //    data_types::array::array_basics();
    // const NUMS: [i32; 4] = [2,7,11,15];
    // const TARGET: i32 = 9;
    // simple_programs::two_sum::two_sum(TARGET, NUMS);
    // const NUM: usize = 10;
    // simple_programs::factorial::calc_factorial(NUM);
    // const HEIGHT: [usize;9] = [1,8,6,2,5,4,8,3,7];
    // simple_programs::leetcode11::container_with_most_water(HEIGHT);
    // const BLUEPRINT: &str = "b3c6d4";
    // simple_programs::build_pattern::build_pattern(BLUEPRINT);
    // structs::structures::_structures();
    // structs::structures::associated_fns();
    // enums::enums::enums_basics();
    // match_basics::match_basics::match_basics();
    // check_mod::check();
    // collections::vectors::vector_basics::vector();
    // collections::vectors::vector_basics::iterating_vector();
    // collections::strings::string::string_basics();
    // collections::hashmap::hashmap::hashmap();
    // error_handling::Errors::ErrorHandling::error_handling();
    // error_handling::unrecoverable_errors::UnrecoverableError::panic();
   // error_handling::recoverable_errors::RecoverableErrors::recoverable_errors();
    // writing_test::tests::add(3, 4);
    // simple_programs::compression::compression();
    // concurrencies::concurrency::concurrency();
  // smart_pointers::smart_pointers::smart_pointers();
  // smart_pointers::box_pointers::box_pointers();
  // smart_pointers::reference_counted::reference_counted_smart_pointers();
asynchronous::asynchronous::asynchronous();
  // macros::macros::macros();
}
