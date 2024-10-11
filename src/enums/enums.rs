//enum is the way of saying a value is among one of the set of values
//for eg: a rectangle can be one of the shape val among the set of shapes like circle triangle.
//its like enumerating all possible values

//ip address can be of version 4 or versison 6  and it can only be one

pub fn enums_basics() {
    //now _IpAddrkind is a custom datatype where data can be either of type v4 or v6
    enum _IpAddrkind {
        //v4(String), //can define type as well, now the v4 becomes the function and we cann dreate a instance of it, now v4 is a function which takes string as arg and returns the instance of v4.
        //not just string we can also padd number, struct as an arg
        v4,
        v6,
    }

    //creating an instance of enum
    let old_ip = _IpAddrkind::v4;
    let new_ip = _IpAddrkind::v6;

    handle_route(old_ip);
    handle_route(new_ip);

    //now we can define a custom function which can take two types
    fn handle_route(ip_addr: _IpAddrkind) {
        //do stuffs
    }

    //we can define a structure where the version can be of type _IpAddrkind and other common fields like address, data etc.., and create a instance of IpAddr.
    struct IpAddress {
        version: _IpAddrkind,
        address: String,
        data: String,
    }
    impl IpAddress {
        fn handle_routing(&self) -> bool {
            return true;
        }
    }
    let v4_example = IpAddress {
        version: _IpAddrkind::v4,
        address: String::from("127.0.0.0"),
        data: String::from("text_message"),
    };

    let v6_example = IpAddress {
        version: _IpAddrkind::v6,
        address: String::from("::1"),
        data: String::from("text_message"),
    };

    let status: bool = v4_example.handle_routing();
    println!("routing success: {status}");

    //let v4_addr = _IpAddrkind::v4(String::from("127.0.0.1"));

    //lets take a another practical example where enums might be necessary
    //suppose we are getting a response from a web server there could be be many possibilites like getting desired data, getting error, getting timeout
    //using enums we can make response to be a value among these three values

    //its not like we cannot check reponse status with if else blocks it is possible but not just scalable and is prone to typos
    enum ApiResponse {
        Success(String),
        Error(u16),
        Timeout,
    }

    fn handle_response(response: ApiResponse) {
        //about match in later chap
        match response {
            ApiResponse::Success(data) => {
                println!("Data received: {}", data);
            }
            ApiResponse::Error(code) => {
                println!("Error occurred. Status code: {}", code);
            }
            ApiResponse::Timeout => {
                println!("Request timed out.");
            }
        }
    }

    //another example of enum
    #[derive(Debug)]
    enum Message {
        Quit,
        Write(String),
        Move { x: u32, y: u32 },
        ChangeColor(u8, u8, u8),
    }

    impl Message {
        fn apply_color(&self) -> bool {
            return true;
        }
    }

    let new_color = Message::ChangeColor(12, 34, 56);
    println!("{new_color:?}");
    let color_status: bool = new_color.apply_color();
    println!("color apply status: {color_status}");

    //just like struct we can define a impl functions, create an instance, so enums are quite handy

    //OPTION ENUMS

    //the option enums handles the situation where the value can be something or nothing(null).
    //a scenario while accessing a data from a hashmap initially there could be no data stored(eg in two_sum.rs)
    // a null is a value that is not present at the moment accessing null value results in error,
    //rust donot have null type unlike ts but it has enum that handles the concept of value being available or empty.

    //T is a generic type parameter similar to one from typescript
    //T can be any type using this approach makes option enum compatible with any types(more about generics later)

    // enum Option<T>{
    //     None,
    //     Some(T),
    // }

    let _x: i8 = 5;
    //option enum is already built in and comes with various methods no need to declare
    let y: Option<i8> = Some(10); //even though we have used 10 here which will never be null, this option enum is typically used for vals like server response, accessing empty obj etc..
                                  // let sum = x + y;
                                  // println!("sum is {sum}");
                                  //even thought x and y are of same type we cannot really add them
                                  //because when we use option enum for y theres a possibility that the value might not be present, which ensures there will not be any run time error

    //checking whether the value null or not
    assert_eq!(y.is_some(), true);
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

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, //must specify all the condition or else it will show error
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //*************************************Catch-All patterns and _the placeholder*************|
    //using enums we can also define a way in which certain values cause special action while all
    //other values cause a default action
    let dire_roll = 9;
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
