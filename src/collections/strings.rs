//string is a collection of bytes, it is encoded with UTF-8
//many of the operations available with vector is still available with string as string is implemented as a wrapper to the vector, 
//with some extra restriction & features.

pub mod string{
    pub fn string_basics(){
        //initializing a string:
        let mut s = String::new();
        //this will create a new empty string s, where we can load data.


        //often we have to create a string with data still on:
        let initia_data = "Hello world!";
        let mut loded_String: String = initia_data.to_string();
        //type changed from slice to string hmm
 

        //using .to_string method to initialize a loaded string is similar to using string::from
        let mut from_string = String::from("loaded string with initial data");
        let hello = String::from("Здравствуйте");
        println!("{hello}");

        from_string.push_str("pushed data");
        //the pus_str takes string slice as an argument, cause we donot want to take the ownnership of the parameter.
        //eg showing why it is necessary.
        let mut s1 = String::from("initial string");
        let s2 = "pushed string";
        s1.push_str(s2); //this can use char also as a arg hai.

        //here if push_str were to take value insteas of reference, the parameter will take the ownership. and we will not be able to access s2 after that.
        println!("checking if s2 is still valid: {s2}") ;//it is.

        //concatentaion with + operator.
        let str_1: String = String::from("Hello ");
        let str_2: String = String::from("World");
        let concat: String = str_1 + &str_2;
        //s1 is moved here and is no longer valid afterwards.

        //the reason we had to do str_1 as val and reference of str_2 is because of the method that will be called because of + operator.
        //fn add(self, s: &str) -> String {} //s1 is the self here and &str_2 is a reference to s.
        //str_1 will no longer be valid.
        //if we need to concatenate three string later two will be reference.
        //this way it is more effecient while concatenating 1 copied val two reference
        
        //we can also use format macro to concatenate strings:
        //fomrat macro doesn't takes any values of the string, just a reference for all.
        let form = format!("{str_2} {str_2}"); //kinda similar to how println macro works.
        println!("concatentaing using format {form}");

        println!("printing str_2 after passing to format macro: {str_2}"); //valid

        //INDEXING INTO STRINGS:

        let new_string = String::from("New String");

        //this results in error sayin type str cannot be indexed by integer
        //basically string doesnt supports indexing, the why is how rust stores string in the memory.
        // let second = new_string[1];

        //A String is a wrapper aroung vec<u8>, 8 bit unsigned integer
        let spanish_hello = String::from("Hola");
        //the length here will be 4 because each char is represented by a 1byte
        //so the vector will prob be like [111000, 100101,1010101, 0001010]


       //(Note that this string begins with the capital Cyrillic letter Ze, not the number 3.)
        let russian_hello = String::from("Здравствуйте");
        //applying above logic the length of a string is 12, but actually it is 24, because each char in that string takes 2 bytes. 
        //so indexing will not give a appropirate value
        //lets see why is that 
        // to accesss 3 from the string 
        // let access_first = russian_hello[0];
        //but when encoded the 3 is [208], [151], 2 8 bit digits as we can see
        //the index will point at [208], which doesn't corresponds to any character and user probably wouldn't want a 8 bit digit returned while looking for a char.


        //SLICING STRINGS
        //we now know why indexing is a bad idea with rust.
        //however we still want to access the first data from a russian_hello
        //for that we slice the string to get our desired data.

        let access_first = &russian_hello[0..2];
        //we cannot slice with [0..1] here
        println!("first letter is: {access_first}");

        let spanish_first = &spanish_hello[0..1];
        println!("first spanish hello is {spanish_first}");


        //METHODS FOR ITERATING OVER STRING:
        for char in russian_hello.chars(){
            println!("{char}");
        }
        //alternating if we want corresponding bytes returned.
        for bt in russian_hello.bytes(){
            println!("{bt}")
        }
        
//some methods: contains for searching in string and replace for substituting in string    

//END
 
    }
}
