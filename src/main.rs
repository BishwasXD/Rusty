pub mod data_types{
    pub mod strings; 
    pub mod numbers; //approach without declaring in mod.rs, now the files are not treated as module.
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

mod simple_programs{
    pub mod fibonacii;
}

fn main() {
    simple_programs::fibonacii::fibonacci();
    
}