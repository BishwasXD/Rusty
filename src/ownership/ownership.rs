//ownership(rule to manage memory) is the feature to guarantee memory safety without garbage collector.
//rust has no garbage collector.
//ownership related features: borrowing,slices,Rust
//ownership is rust exclusive concept
//mostly to manage heap data.

//Rules for ownership
//1.Each value in rust has owner.
//2.There can only be one owner at a time.
//3.When the owner goes out of scope,the value will be dropped.

pub fn _ownership(){
    
    {
        //s is valid in this scope.
        let _s ="hello";
        //one memory allocator needs to have one free memory assigned.
        //rust calls "drop"(free memory) automatically at closing of curly brackets. 
        
    }
    //s is not valid in this scope.
    //string literal:where string value is hardcoded.
    //we can create a string from string literal using from function
    //memory is requested at runtime from memory allocator using "from"
    let mut s_a = String::from("hello"); 
    s_a.push_str(",world");//push_str appends literal to string
    println!("{s_a}");

    let x = 5;
    let _y =x; //basically means x=y=5

    //in case of strings
    //string are stored in the heap,while 3 parts are stored on stack??not sure (fact check this)
    //string is made of 3 parts:pointer to mem,length(amt of mem.) and capacity.Imagine kinda a like linked list.
   {
     let s1 = String::from("hello");
    let _s2 = s1;
    //println!("{s1},this") //doing this will throw error though.
}
//here in case of string s2 ,it copies all three parts of string but it does not copy the data on heap which pointer referes to.in detail fig:4-3
//here if s1 and s2 go out of scope and drop is used we will be freeing same heap twice,causing double free error(not shown on program but it is a bad practice)
//To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
// (shallow copy and deep copy) in rust shallow copy is called move.Above s2=s1 is shallowcopy/move as actual data is not copied.

//rust does'not automatically create a deep copy.
{
    let s1 = String::from("hello");
    let _s2 = s1.clone(); //.clone method to deep copy the heap data as well.
}

//integers are entirely stored on the stack.
{
    //thats why this works for int but not for strings.
    let x = 5;
    let _y =x;
}
//121
//Remaining Still a lot of actual juice.

}