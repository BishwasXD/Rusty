//all in brief
//Just a fast track,none gone in detail
//none tested :Probably lots of errors
//only has basic things

use std::collection::Hashmaps;
fn main(){
    let x:i32 = -42; //signed 32bit integer
    let y: u64 = 100; //unsigned  64-bit integer
    //value range for i32 is (- or +) and range for u64 is only (+).

    //floats
    //f32,f64
    let pi:f64 = 3.14;

    //boolean values
    let is_snowing:bool  = true;

    //character char
    let letter:char = 'a';

    //Compound Data types
    //arrays,tuples,slices and string(slice string)

    //Arrays
   let numbers:[i32;4] = [1,2,3,4];
     //  println !("{}",numbers); //error due to formatter
     // println !("{:?}",numbers); //doing this works
     let fruits:[&str;2] = ["apple","Banana"];

    //Tuples
    let human:(String,i32,bool) =("Alice",30,false); //error:Alice is string slice
    //let human =("Alice",30,false); //works
    let human:(String,i32,bool) =("Alice".to_String(),30,false); 

    let my_mix_tuple = ("kratos",23,true,[1,2,3,4]);

    //Slices
    let number_slices:&[i32] = &[1,2,3,4];
    let animal:&[&str]=&["monkey","donkey"]; 

    //Strings vs string slices(&str)
    //Strings[growable,mutable,owned string type]
    let mut stone_cold:String = String::from("Hell");
    stone_cold.push_str("Yeah!");

    //B- &str (String Slice)
    //immutable,reference to already existing string
    let string :String = String::from("Hello,World");
    let slice:&str=&string; //references to string

    //Ownership,Borrowing And References
    //ownership rules
    //each value has a owner
    //only one owner at time
    //when owner is out of scope value is dropped
    fn test(){
        let s1 = String::from("RUST");
        let s2 =s1; //transferred the ownership of string to s2,now cant print s1 output
        let len = calculate_length(&s1);

    }
    //s1 does not exist here as it is out of scope
    fn calculate_length(s: &String)->usize{
    s.len
}  

//Borrowing and References
//Safety and Performance
//References can be mutable and immutable
//just & to reference,does'nt transfer ownership
fn references(){
    let mut  _x = 5;
    let _s = &mut_x;

    let   _y = 5;
    let _r = &y;
    *_r+=1;
    
}

//Structures
struct BankAccount {
    owner:String,
    balance:f64,
}
impl BankAccount{
    fn withdraw(&mut self,amount:f64){
        self.balance -= amount;
    }
    fn check_balance(&self){
        println!("{}",self.balance)
    }
}
//fn main(){
//let mut account = BankAccount{
  //  owner:"Alice".to_string(),
    //balance : 150.00,
//};
//u can have only one immutable borrow and mutable borrow
//account.check_balance();//immutable borrow
//account.withdraw(22.00);//mutable borrow
//}


//Variable and Mutability
//everything is immutable by default
let immut_var = 10;//immutable variable
let mut mut_var  = 12;//mutable variable

//Constants
//cant use mut variable to make it mutable
//MUST provide type for constant and Must be capital letters
const CONSTANT:i32= 10;

//Shadowing
//not as same as making variable mutable
//lets use same variable name having different types
let x =5;
let x = x+1;//this x shadows the first x.
{
    x =x*2;//shadowing in this scope only (shadows x=x+!)
}


//Control flow
//IF.........ELSE
//same as other languages
let condition = true;
let number = if condition{5} else{6};

//loop
//unconditional ,infinite loop
loop{
//condition to end loop can be provided 
}

//while 
while(condition){
    //code
}

//for loop
let a =[1,2,3,4,5,6];
 for element in a{
    println!{"{element"}
 }

 //ENUMS
 //A versatile tool used to represent a type that can take on one of several possible variants
 //unlike in structures each can hold different kinds of data types
 enum IpAddrKind{
    V4,
    V6,
 } 
let four = IpAddrKind::V4;
let six= IpAddrKind::V6;
fn route (ip_kind:IpAddrKind){}
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
//using struct for enum
    struct IpAddr {
        kind:IpAddrKind,
        address: String
    }
    let home=IpAddr{
        kind:IpAddrKind::V4,
        address:String::from("128.0.1.1"),
    };

    //using enums
     enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String),
 } 
    
    let home33= IpAddr::V6(String::from("128.0.3.1"));
    //enhanced enums
    let home3= IpAddr::V4(String::from(128,0,4,1));

    //Error Handling
    //Approach 1
    enum Option<T>{ //Define generic option type
        Some(T),
        None,
    }
     //Approach 2
    enum Option<T,E>{ //Define generic result type
        Ok(T),//Represents value
        Error(E),//Presents an error
    }
 //Example:
 fn divide(num:f64,denom:f64)->Option<f64>{
    if denom == 0.0{
        None
    }
    else{
        Some(num/denom)
    }
 }
 let result = divide(10.0,0.0);
 match result{
    Some(xy)=>println!("Result:{}",xy),
    None=>println!("Cannot divide by zero"),
 }

 //Collection Types
 //vectors,hashmaps and utf-8
 //Vector
 let _v:Vec<i32>=Vec::new();
let _the_Vec:vec<i32> = vec![1,2,3];
//add element to vector
let mut _numbers_vec:Vec<i32>=Vec::new();
_the_numbers_vec.push(5);

let third:&i32 = &_v[2];//Access number of vector,uses index 
let another_third = _v.get(2);//Same thing

//UTF-8 encoding
//string are utf-8 encoded
//just about  strings

//hashmaps
//use std::collection::Hashmaps; //use library to use hashmaps
let mut scores = Hashmaps::new();

scores.insert(k::from("Blue"),10);
scores.insert(k::from("Yellow"),50);

let team_name=String::from("Blue");
let score = score.get(&team_name).copied().unwrap_or(0);

for(key,value)in &scores{
    println!("{key}:{value}");
}
}   