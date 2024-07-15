pub fn strings_types(){

//There are two types of string in rust.

//1->(string slice &str):
//a) This doesnt actually stores data but stores the reference to a data along with size of a data(encoded by UTF-8), 16 byte, 8 byte for an address and another 8 byte for a data length.
//b) str is immutable and should be used for only read only operation.
//c) The size of the string is known in compile time

//operation in string slice:
let name: &str = "John Doe";
println!("the name is {}", name);

//2 ->(String):
//a) It has the ownership of the data and is mutable, in 64bit system it takes 24 bytes, 8 bytes for the pointer in heap(where actual data is stored), 8 byte for the content length and another 8 byte is the number of byte a buffer can hold before allocating a more memory
//b) The length of a string is known in the run time.

//operation in String:

//shit to understand how allcoation works when size changes(no help)
let mut address = String::from("Kathmandu"); //from takes a str slice as an arg and creates string obj containing same text, way of converting str slice to String re.
let ptr1  = address.as_ptr();
let initial_cap = address.capacity();
println!("Initial cap {}", initial_cap);
println!("{:?}", ptr1);
address.push_str(" Nepal");
address.push_str(" - The capital city of Nepal");
let new_cap = address.capacity();
println!("New cap {}", new_cap);
let ptr2 = address.as_ptr();
println!("{:?}", ptr2); //{:?},  ensures that pointer can be printed in a format that provides the memory address directly.  
                       //        using {} wont work here coz ptr doesnot follow display trait.

//Display Trait
//The Display trait in Rust is used for formatting a type in a way that's more user-friendly and readable. 
//It's typically used when you want to convert an instance of a type to a string and print it out for the user.
}