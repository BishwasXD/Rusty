pub mod vector_basics {
    pub fn vector() {
        // A vector is a collection of data of same type
        //initializing an empty vector:
        let mut v: Vec<i32> = Vec::new(); //Vec is a structure
        //Vec structure has various methods like push pop, new
        //called as associated function, push pop are methods and new is a constructor that returns a instance of a Vec object.

        //we can also create a vector using a vector macro:
        let mut new_vec: Vec<&str> = vec!["Apple", "Banana"]; // automatically infers a type.

        let type_vec: Vec<i32> = vec![1, 2, 3];

        //now that we have initialized the v; we can use various methods provided by a Vec module

        //PUSH METHOD:
        v.push(3);
        println!("{:?}", v);
        new_vec.push("Pineapple");
        println!("{:?}", new_vec);
        //Note: to use push method var should be mutable(common mistake)

        //READING ELEMENTS FROM A VECTOR:
        //normal way
        let third: &str = new_vec[2];
        println!("{}", third);
        println!("{:?}", new_vec); //checking if value is moved or not(it doesn't)

        //using get method
        let get_third: Option<&i32> = type_vec.get(100);

        println!("{:?}", get_third);
        //NOTE: if we use get method to access elements by index, it returs the Option type with some / none
        //this feature seems very useful in error handling and condition where index goes out of scope.
        match get_third {
            Some(item) => println!("u got the item {}", item),
            None => println!("Index not found in vector"),
        }

        //some reference concepts in vector:
        let mut ref_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        let fourth: &i32 = &ref_vec[3]; //accessing element using reference of a ref_vector.
        //now the fourth contains the reference of a ref_vector[3].
        //if & was  not used then fourth would copy the value from ref_vec[3] to itself.

        //ref_vec.push(5); // remove & from above line to make this work

        println!("{:?}", ref_vec);
        println!("fourth element is: {fourth}")

        //the code above looks like it should work but it doesn't
        //the reason is related to how vector works, a vector puts value next to each other in memory.
        //adding value at the end of a vector might require allocating a new memory and copying it's value to the new space.
        //if that happens fourth will be referencing to empty or garbage space, since fourth has a reference to ref_vec[3] not value.
        //to prevent that from happening we cannot access fourth after pushing item to the vector, glad error happens at compile time.
    }

    pub fn iterating_vector() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        //iterating using for loop.
        for num in &v {
            //println!("{num}");
        }

        //to make changes while iterating we will have to make mutable refernce while iterating
        for num in &mut v {
            println!("{}", *num * 2);

            //see the * before num, it a dereference operator(more in later chap).
        }

        // v.push(20); THIS CANNOT BE DONE IF WE ARE USING VALUE TO LOOP

        //can we insert a item into a vector while in loop?
        //-> while using reference NO, we know pushing val in vector can cause new memory allocation and if that happens we will be referencing to a null while iterating.
        //-> even if we loop using value instead of refernce we cannot push in loop because ownwership of v is moved into the loop and no changes can be made to v.

        //USING ENUMS TO STORE MULTIPLE TYPE IN VECTOR
        //earlier we only had vector of same type, however we can use enums to have a multiple type vector.
        #[derive(Debug)]
        enum SpreadSheetCell{
            Int(i32),
            Float(f32),
            Text(String)
        }

        let row = vec![
            SpreadSheetCell::Int(3),
            SpreadSheetCell::Float(2.9),
            SpreadSheetCell::Text(String::from("Hello World!"))
        ];
        //lets use match while iterating
        for item in &row{
            match item{
                SpreadSheetCell::Int(val) => println!("DO INT OPERATION WITH {}", val),
                _ => println!("WE ARE FLOAT AND STRING TYPE")
            }
        }
        //WHY VECTOR NEEDS TO KNOW THE TYPE?
        //->it is because rust needs to know exactly how much memory to allocate in the heap for the item
        //-> if we allow vector to hold any type then there is a chance that operation in one of the type can cause error on other type. we used enums for this particular reason.

    }
}
