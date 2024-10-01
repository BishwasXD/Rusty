pub mod UnrecoverableError{
    pub fn panic(){
        //there are two way of causing panic, one by doing an action that causes panic such as accessing an array past the end
        //two by calling the panic macro.
        //by default the panic will print an error message, unwind, clean up the stack and quit.
        //we can also track the point of panic in the stack via an env variable.

        //when panic occurs the program starts to unwinding, which means rust walks to the stack and clean up the code from a function that is present in the stack.
        //this is pain for rust, walking cleaning so we have a option to quit a program without cleaning up.
        //and memory that the program was using is released by the OS.

        //lets call panic here(the second way of causing panic in the program)

        // panic!("die");
        // println!("will it work after panic?"); //program wont work after panic is called.

        //what we get in terminal:
        //thread 'main' panicked at src/error_handling/unrecoverable_errors.rs:13:9:
        //die
        //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        //we get panic message alongsie with the file name where panic occured.

        //USING PANIC BACKTRACE:
        //lets us take a look at another example where the panic occurs due to some lib.
        let vec = vec![1,2,3];
        vec[99];

        //in C if u do this this is a undefined behavior, instead of panicking and immediatley stopping the exec
        //it will simply returns whatever there is in the memory that corresponds to vec[99] memory address.
        //it  will return grabage value, which is not good from a security point of view.
        //the attacker can mauipulate the index in such a way that, it would allow them to access data which they donot have access to.

        //so to prevent that rust will immediately stops the execution.
        //the error message is similar to when we called panic macro explicitly.

        //BACKTRACE:
        //in terminal it says to run with backtrace
        //a backtrace is the list of functions that has been called to get to this point
        //running RUST_BACKTRACE=1 cargo run will results in list of function that has been called.
        //we get a long list of output that appears to be a stack.
        //it also shows where panic occured(it labels it).

        
        


   



    } 
}