 pub mod ErrorHandling{
    pub fn error_handling(){

      //with error handling we look at the codn when error is likely to arise and take some action before the code is compiled.
      //with proper err handling our program  becomes more robust.

      //with rust there are two types of error:
      //1. Recoverable Error:
      //eg of recoverable error is file not found error, here if file is not found we simply want to notify the user to retry the operation with correct file path.

      //2.Unrecoverable Error:
      //unrecoverable error stops the entire program, error arise during trying to access location beyond the end of an array, bugs etc..

      //A file not found error is recoverable because the issue can often be addressed by the user or by altering the runtime environment 
      //(e.g., providing the correct file path or creating the file). These types of errors are expected and can be handled without terminating the program. 
      //In contrast, unrecoverable errors are symptoms of bugs in the program logic, where immediate termination is usually the safest course of action.

      //most language donot divide error in two category and call every error exception
      //rust donot have exception but i t have type Result<T,E> for recoverable error and panic!() for unrecoverable error.
      //panic macro stops the execution of a program 

    }
}