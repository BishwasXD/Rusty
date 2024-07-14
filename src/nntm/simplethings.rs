//Common programming concepts


pub fn data_types(){
    //making x mutable as it is immutable by default;meaning u can change its values now
    let mut x = 4;
    //can change the value now
    //if u use 'let' again and reassign the value then it will think it is another variable in which case it is shadowing the above "x" 
   

    //constant
    //naming convention for rust constant:USE ALL UPPERCASE SEPARATED BY UNDERSCORE
    const THIS_IS_A_CONSTANT:u32 = 10;

    //shadowing
     x= x - 1; //if var is declared with mut it has to be modified somewhere
    println!("This is x:{}",x);
    let x = 7; //if u dont use let then it is reassigning the value to previous x,so should give error.Use let to make new variable named "x" altogether which is shadowing.
    
    {
        let x =5;
    println!("{}",x); //should print 5 ,as it is assigned to 5 only on this scope
}
println!("{}",x+1); //should print 7,

//scalar types
//There are four types of scalar data types
// integer ,floating points, boolean and character

//signed and unsigned integers
//signed meaning it can be positive or negative
// signed interger stores value between –(2^(n – 1)) to (2^(n – 1)) – 1
// let x:i32 =12; //signed 32-bit integer  ,if u need 8 bit use i8,128bit use i128 and so on so forth

//unsigned:can and will only ever be positive
//unsigned will store between  0 to 2^(n)-1
// let x:u64=54;// unsigned 64-bit integer

//there are also usize and isize ,i dont know much abt them :(

}