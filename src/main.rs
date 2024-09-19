
 pub mod data_types{
//     pub mod strings; 
//     pub mod numbers; 
       pub mod array;
}
mod loops{
    pub mod loops;
   // pub mod fibonacci_loops;
}
mod ownership{
 pub mod ownership;
}

mod structs{
    pub mod structures;

}
mod enums{
    pub mod enums;
}

mod simple_programs{
    // pub mod two_sum;
    // pub mod factorial;
    // pub mod leetcode11;
    pub mod build_pattern;
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
structs::structures::_structures();
structs::structures::associated_fns();
enums::enums::enums_basics();

    
}