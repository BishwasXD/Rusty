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

// mod structs{
//     pub mod structures;
// }

mod simple_programs{
    pub mod two_sum;
}

fn main() {
    // simple_programs::fibonacii::fibonacci();
//    data_types::array::array_basics();
const NUMS: [i32; 4] = [2,7,11,15];
const TARGET: i32 = 9;
simple_programs::two_sum::two_sum(TARGET, NUMS);
    
}