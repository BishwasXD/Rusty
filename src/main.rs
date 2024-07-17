
pub mod data_types{
    pub mod strings; 
    pub mod numbers; //approach without declaring in mod.rs, now the files are not treated as module.
}


fn main() {
    data_types::strings::strings_types();
    data_types::numbers::integer_types();
}