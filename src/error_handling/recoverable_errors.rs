
pub mod RecoverableErrors{
    use std::{fs::File, io::{ErrorKind, Write}};
    use std::process;
    pub fn recoverable_errors(){
        //most errors are not serious enough to stop the execution of the entire program 
        //for eg: if u want to access a file and that cannot happen because there is no file, u might want to create a file instead of terminating a program.

        //for that we use Result enum
        enum Result<T, E>{
            OK(T),//success of type T
            Err(E),
        }
        //Result has a generic type parameter so we can use it for all kinds of operation
        //file::open opens file in read only mode, while file::create file in write only mode.
        let greeting_file_result: std::result::Result<File, std::io::Error> = File::create("example_file.txt");
        //this is why generic type is important, here when operation is success we are accessing a file type, but in other operation this could be i8, string etc..

        //however we need to handle the error ourselves:
        //using match

        // match greeting_file_result {
        //     Ok(file) => println!("File opened successfully"),
        //     Err(error) => panic!("Problem opening the file: {error:?}"),
            
        // }

        //the code above doesn't looks recoverable, but we can make it.
        //there could be various reasons that causes error, one of them is file not existing
        //lets recover form that, for that we have to match error kind
        //Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
        //this is the erro we got from error message, see there is the error kind, if error kind is NotFound we can try creaing a file.
       let x =[240, 159, 166, 128];

        match greeting_file_result {
            Ok(mut file) => {
               let res =  file.write(&x); //b is a rep for byte string literal where the string will be rep as a string of bytes.
               match  res {
                Ok(h) => println!("write operation completed"),
                Err(e) => println!("Error occured {e}")
                   
               }
             
            }
            Err(error) => match error.kind() { //error.kind() returns NotFound
                //ErrorKind is an enum
                ErrorKind::NotFound => match File::create("example_file.txt") {
                    Ok(fc) => {
                        println!("File created successfully");
                        
                    },
                    Err(err) => panic!("Problem creating a file: {:?}", err),
                },
                other_error => panic!("Problem opening a file: {:?}", other_error),
            },
        }
        
    //using unwrap_or_else instead of using match, match is primitive and we have to write more code with it.
    //we use unwrap_or_else to handle the result type values 
    //what this do is if any error is caught it runs the anon function provided in unwrap_or_else arg
    //|error| { ... } this specific is the way we write closure in rust  error is passed as arg to closure.
    //A closure is an anonymous function that can capture variables from its environment. 
    //The |...| syntax is used to define the parameters for the closure, similar to the (...) syntax used in regular function definition

    let new_file = File::open("new_file.txt").unwrap_or_else(|error| {
        println!("the error is: {error}");
        //here we can use if else to do stuffs acc to error
        if error.kind() == ErrorKind::NotFound {
            File::create("rescue_file.txt").unwrap_or_else(|error|{
                panic!("error creating a file {error}");
            })
        }
        else{
            panic!("different error {error}");
        }
    
    });
     //if file do exists we can continue the code from here
     println!("File exists do stuffs ");


//USING EXPECT
//with this if file is not found we can directly provide a message for a dev for debugging
let e_file = File::open("e.txt").expect("e.txt File should exist before executing further");//terminates the execution



    }
}