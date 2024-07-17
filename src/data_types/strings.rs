pub fn strings_types(){

//There are two types of string in rust.

//1->(string slice &str):
//a) This doesnt actually stores data but stores the reference to a data along with size of a data(encoded by UTF-8), 16 byte, 8 byte for an address and another 8 byte for a data length.
//b) str is immutable and should be used for only read only operation.
//c) The size of the string is known in compile time

//operation in string slice:
let name: &str = "John";
// name.push_str("Doe"); // cant be done.
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

//check palindrome:
let  pattern: &str = "ababax";
let mut result: String = String::from(""); //creating a instance of string from String struct. from converts one type into another in this case &str to String.
let reverse: String = pattern.chars().rev().collect(); //chars convert string in array of char, rev reverses the arr, collect collects the chars from a iterator and converts to a str.(similar to ''.joined(array) method for this case, collect might be more powerful.)
//now checking logic is easy.


//some ooperation in string:
//1. concatenation -> push_str(arg); arg can be string or a char, both are valid.
let mut s = String::from("Hello");
s.push_str("w");
println!("{s}");

//using + operator -> The  + operator combines String with a &str to concatenate string, it requires left handed to be string and right handed to be &str

let hello:String = String::from("Hello");
let world:String = String::from("world");
let mut concat: String = hello + " " + &world; //here hello is moved, now we cannot access hello, but world is borrowed, which means the type of world is now in &str(in concat variable only).
// println!("{hello}"); //cant be done
println!("{world}"); //world has the org data so this is still valid.
println!("{concat}") ;

//such weird rule exists with + operator, due to efficiency concerns the first str is owned while. the second one is borrowed whoch prevents the unnecessary copying of data. other reasons also exists.

//2. Basic fns related to strings
let a: &str = "Hello";
println!("lenght : {}", a.len()); //outputs length(in byte);
//more about length:
//for a here the output is 5 and 5 bytes are used to display it(the variable a has all the ascii characters, so 1 byte representation for each char).
//when a is rep as a array of characters ['h','e','l','l','o'], every char is 4 byte so the length with be 20 byte.
//when there is a string of emoji(which is not an ascii character, the size of each char will be > than 1byte).


println!("bytes : {:?}", a.bytes()); //gives a corresponding byte value assigned to a character, basically gives the UTF-8 encoding of a characters
// println!("{}", a[0..2]); // this throws err coz the compiler has to know the exact size of the result before printing. the error is prob coz the result was directly used inside of a print statment. 
let sliced = &a[0..2];
println!("slices string: {}", sliced);


}