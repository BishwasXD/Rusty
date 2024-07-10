mod first;

fn main() {
    let mut y = 5;
    y = y + 1;
    
    // let positive: u32 = -34; // cant assign negative number with unsigned integer
    
    let positive: i128 = -342345678987654345678; // the number is bit
    
    //in signed integers one bit is used to sign int(to determine positive or negative), so in case of i8 only 7bits can be used for data rep. in unsigned its 8bit
    let limit: u8 = 240;  //i8 throws error
    
    let float: f64 = 22.0 / 7.0; //one bit for sign, 8 for exponent, 23 for mantissa, 32 provides 7 decimal digits of precision, 15 for 64, doint just 22/7 cuts the decimal and treats the value as interger 3 in this case
    
    println!("checking floating number {}", float);
    println!("checking limit of signed and unsigned {}", limit);
    println!("not a positive number {}", positive);
    println!("the var decalred is {}", y);
    println!("Hello, world!");
    
    first::new_fn();
    
    for number in 0..6 {
        if number == 4 {
            println!(" checking if condition : {}", number);
            break;
        }
        println!("{} is the number", number);
    }
}
