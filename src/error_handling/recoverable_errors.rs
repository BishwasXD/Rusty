
pub mod RecoverableErrors{
    use std::{fs::File, io::ErrorKind};
    pub fn recoverable_errors(){
        //most errors are not serious enough to stop the execution of the entire program 
        //for eg: if u want to access a file and that cannot happen because there is no file, u might want to create a file instead of terminating a program.

        //for that we use Result enum
        enum Result<T, E>{
            OK(T),//success of type T
            Err(E),
        }
        //Result has a generic type parameter so we can use it for all kinds of operation
        let greeting_file_result: std::result::Result<File, std::io::Error> = File::open("hello.txt");
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

        match greeting_file_result {
            Ok(file) => println!("File opened successfully"),
            Err(error) => match error.kind() { //error.kind() returns NotFound
                //ErrorKind is an enum
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => {
                        println!("File created successfully");
                        
                    },
                    Err(err) => panic!("Problem creating a file: {:?}", err),
                },
                other_error => panic!("Problem opening a file: {:?}", other_error),
            },
        }
        

    }
}