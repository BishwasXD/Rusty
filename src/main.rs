
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

fn main() {
    data_types::strings::strings_types();
    data_types::numbers::integer_types();
    ownership::ownership::ownership();
}