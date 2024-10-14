//match is control flow construct that allows to compare the value against series of patterns and execute the code on the basis of matched patterns
//simple analogy would be to think match as a coin sorting machine, where coins of fidd sizes passes through a machine and gets sorted based on their sizes

pub fn match_basics(){
    #[derive(Debug)]
    enum Coin {
        Ekrupiya(u8), 
        Duirupiya,
        Pachrupiya,
        Dusrupiya,
    }
    
    fn sort_coins(coin: Coin) -> u8 {
        match coin {
            Coin::Ekrupiya(val) => {
                println!("u got ek rupiya {}", val) ; 
                println!("checking coin {coin:?}"); //here u get a instance of ekrupiya as learned in enums.
                1
            },
            Coin::Duirupiya => 2,
            Coin::Pachrupiya => 5,
            Coin::Dusrupiya => 10,
        }
    }
    //sort coins fn will recieve a coin  which could be one of the 4 coin types, we match coin with given types and if  matched we are simply returning a value here
    //cant we do this with if else ladder? yes but if else is limited to boolean type but here it can be any type, remember switch statement? match is kinda simila
   sort_coins(Coin::Ekrupiya(1)); //sort_coins takes Coin arg of type ekrupiya since that one takes value we provided a value

   //MATCHING WITH OPTION TYPE
   //we can match certain value with option type, this basically does two operation, if some value do smthng, if none do smthng.
   //lets take an example where a function takes an arG OF OPTION type and matches some and none
   //for one case we provide a value and for another we donot provide
   fn plus_one(x:Option<i32>) {
    match x {
        Some(val) => {
        println!("added one to the val {}", val + 1)
        }
        None =>{
            println!("Value not found situation handled")
        }
    }
   }

plus_one(Some(1));
plus_one(None);

//INTERESTING NOTE: rust knows if we missed any case to handle and even knows the pattern we forgot.

//using other to handle all possible scenarios
//just an eg: when u roll a number 1 on the dice u get to move ur piece from home but for other number it skips yr turn


enum DiceRoll{
    One(u8), 
    Two(u8), //lets check with only two for now
    Three, 
    Four, 
    Five, 
    Six
}

//to make this simple make dice to accept u8 arg and remove enum, i did it like this for a practice.
fn roll_a_dice(dice: DiceRoll){
    match dice {
        DiceRoll::One(val) => println!("move outta your house u got {}", val),

        // underscore on prefix to tell rust that, yes im really sure i want to use other here
        // _other => {
        //     println!("turn skipped")
        // }

        //using _ is also valid
        _ => {
            println!("turn skipped")
        }

    }
}
roll_a_dice(DiceRoll::One(1));
roll_a_dice(DiceRoll::Two(2));


//CONCISE CONTROL FLOW WITH IF LET

// it a way of matching one pattern while ignoring other
//let say we have a situation where we want to execute our code only if one pattern is matched
//using prev approach we will have to match atleast 2 pattern.

let config_matrix = Some(3u8); //number 3 with u8

//matching some pattern only, if config_matrix is none we will not get err, the code will go on without executing the if block
if let Some(max)  = config_matrix{
    println!("The maximum is configured to be {max}");
}

   //some error were made
 
    // ************************************Control Flow Construct(match)*********************************************
    //match :allows to compare a value against series of pattern and then execute code based on
    //pattern matches,yes u can do this using if else.match is good to use with enums.
    #[derive(Debug)] //to inspect state
    enum UsState {
        Alabama,
        Alaska,
    }
    enum Coins {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    //if return boolean to check however match can return any kind of data
    fn value_in_cents(coin: Coins) -> u8 {
        match coin {
            Coins::Penny => {
                println!("lucky coin");
                1
            }
            Coins::Nickel => 5,
            Coins::Dime => 10,
            Coins::Quarter(state) => {
                println!("State quarter from{:?}!", state);
                25
            }
        }
    }
    value_in_cents(Coins::Quarter(UsState::Alaska));
    //**************************Matching with Option<T>

    fn pluss_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, //must specify all the condition or else it will show error
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = pluss_one(five);
    let none = pluss_one(None);

    //*************************************Catch-All patterns and _the placeholder*************|
    //using enums we can also define a way in which certain values cause special action while all
    //other values cause a default action
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), //here '_' is a special pattern that matches any value and doesnot bind to
        //that value,meaning if it is not 3 or 7,the dice will keep rolling until it gets that.
        // other => move_player(other), //it is the default value if the received value is not 3 or
        // 7
        _ => (), //nothing happens if it isnt 3 or 7
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}
    // *****************************Consise Control Flow with if let*****
    //if let syntax lets you combine if and let to handle values that match one pattern while
    //ignoring the rest.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be{max}"),
        _ => (),
    }
    //Same code using if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max is{max}");
    }
    //u can add else with if let
    // //Example of match using above code
    // let mut count = 0;
    // fn value_cents(coin: Coins) -> u8 {
    //     match coin {
    //         Coins::Quarter(state) => println!("State{:?}!", state),
    //         _ => count += 1,
    //     }
    // }
    // using if let and else
    // let mut count=0;
    // if let Coin:Quarter(state)=coin{
    // println!("{:?}!,state");
    // }
    // else{
    // count+=1;
    // }

}