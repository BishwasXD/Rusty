pub fn integer_types() {
    //making x mutable as it is immutable by default;meaning u can change its values now
    let mut x = 4;
    //can change the value now
    //if u use 'let' again and reassign the value then it will think it is another variable in which case it is shadowing the above "x"

    //constant
    //naming convention for rust constant:USE ALL UPPERCASE SEPARATED BY UNDERSCORE
    const _THIS_IS_A_CONSTANT: u32 = 10;

    //shadowing
    x = x - 1; //if var is declared with mut it has to be modified somewhere
    println!("This is x:{}", x);
    let x = 7; //if u dont use let then it is reassigning the value to previous x,so should give error.Use let to make new variable named "x" altogether which is shadowing.

    {
        let x = 5;
        println!("{}", x); //should print 5 ,as it is assigned to 5 only on this scope
    }
    println!("{}", x + 1); //should print 7,

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

    let float: f64 = 22.0 / 7.0; //one bit for sign, 8 for exponent, 23 for mantissa, 32 provides 7 decimal digits of precision, 15 for 64, doint just 22/7 cuts the decimal and treats the value as interger 3 in this case
    println!("displaying float {}", float);

    //*Character type
    //Rust’s char type is four bytes in size and
    //represents a Unicode scalar value, which means it can represent a lot more
    //than just ASCII. Accented letters; Chinese, Japanese, and Korean
    //characters; emoji; and zero-width spaces are all valid char values in Rust.
    //Unicode scalar values range from U+0000 to U+D7FF and U+E000 to
    //U+10FFFF inclusive.
    let _char = 'a'; //use single literals

    //compound types
    //compound types group multiple values into one type.Types:Tuple Type and Array Type
    //*Tuples have fixed size and cant grow once they are declared
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //to get individual value
    let _tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is:{y}");

    //u can also use (.) to directly get the value
    let third = tup.2;
    println!("{third}");
    //The tuple without any values has a special name, unit.

    //*The Arrays */
    let _norm_array = [1, 2, 3, 4];
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; //with manual data type and size of array
    let _b = [3; 5]; //array[3,3,3,3,3].  what is this a Matlab?
                    //if u try to access out of bound index from array it will not show garbage value.

    //tip: u cant write x=y=4 ,kind of statements
    //statements don't return a value
    //most functions return the last expression implicitly.
    let k = {
        let l = 5;
        l + 5 //if u use ; here ,it will be a statement and will throw an error(without ; ,it is an expression and will return a value)
    };
    println!("The value is {k}");

    //*Functions
    fn five()-> i32{  //equivalent of fn name(int)
        5
    }
    let result = five();
    println!("{result}");


    //NOTE:
    //use _ in a prefix of a variable name, if u declare it but never use it.
}
