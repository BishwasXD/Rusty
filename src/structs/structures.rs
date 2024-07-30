//Structs

//A struct or structure is a custom data type that packages multiple data structures.
struct _user3{
    active:bool,
    username:String,
    email:String,
    sign_in_count: u64,
}

pub fn _structures(){
    //calling a struct
    let user1 = _user3{
        active:true,
        username:String::from("Nabin"),
        email:String::from("dontmessageme@gmail.com"),
        sign_in_count:47,
    };

    //mutable structs
    //entire struct must be mutable and not just a data type we want to be mutable.
    let mut user2 = _user3{
        active:true,
        username:String::from("userName1"),
        email:String::from("dontmessageme@gmail.com"),
        sign_in_count:47,
    };
    user2.username=String::from("anotherUserName2");

    //function with struct use
    //writing user3 works but others names gives error?this is supposed to be new struct,idk why?
    fn build_user(email:String,username:String)->_user3{
        _user3{
            active:true,
            username:username,
            email:email,
            sign_in_count:1,
        }
    }

    //using init shorthand
    fn build_users(email:String,username:String)->_user3{
        _user3 { active: true,
             username, 
             email,
              sign_in_count:1 }
    }

    //creating instances from other instances with strut update syntax
    let _user5 = _user3{
        active:user2.active,
        username:user2.username,
        email:String::from("another@hotmail.com"),
        sign_in_count:user1.sign_in_count,
    };

    //struct update syntax
    let _user6 = _user3{
        username:String::from("Nabim"),
        //must come last to fill in all the remaining fields not used
       ..user2 //take everything else from user2
    };
    //148
}