//Structs

//A struct or structure is a custom data type that packages multiple data structures.
struct _user3 {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn _structures() {
    //calling a struct
    let user1 = _user3 {
        active: true,
        username: String::from("Nabin"),
        email: String::from("dontmessageme@gmail.com"),
        sign_in_count: 47,
    };

    //mutable structs
    //entire struct must be mutable and not just a data type we want to be mutable.
    let mut user2 = _user3 {
        active: true,
        username: String::from("userName1"),
        email: String::from("dontmessageme@gmail.com"),
        sign_in_count: 47,
    };
    user2.username = String::from("anotherUserName2");

    //function with struct use
    //writing user3 works but others names gives error?this is supposed to be new struct,idk why?
    fn build_user(email: String, username: String) -> _user3 {
        _user3 {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }

    //using init shorthand
    fn build_users(email: String, username: String) -> _user3 {
        _user3 { active: true, username, email, sign_in_count: 1 }
    }

    //creating instances from other instances with strut update syntax
    let _user5 = _user3 {
        active: user2.active,
        username: user2.username,
        email: String::from("another@hotmail.com"),
        sign_in_count: user1.sign_in_count,
    };

    //struct update syntax
    let _user6 = _user3 {
        username: String::from("Nabim"),
        //must come last to fill in all the remaining fields not used
        ..user2 //take everything else from user2
    };
    //148

    //accessing user2 will now be invalid as we have moved the email from use2 to user6 as we know email is a string and string doesnt folllows a copy trait
    //however if active and sign in count were to be moved to user6 and keeping new value for username and email user2 would still be valid as bool and u32 follows copy trait.
    // let uname:&str = &user2.username;
    // println!("{uname} check")

    //we can also define a structure wihtout a name :
    struct Color(u32, u32, u32); //known as tuple structure.
    let black = Color(2, 3, 4);
}

pub fn associated_fns() {
    //simply they are the functions associated with the structures
    //there are two types of assoc fns on where instance is passed(self keyword) these fns should be called with the valid instace of that struct and that fns are called methods
    //another one where self is not passed returns the instance instead using a Self keyword
    #[derive(Debug)] //this is necessary to print structs.
    struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        fn calc_area(&self) -> u32 {
            self.width * self.length
        }
        fn check_square(&self) -> bool {
            self.width == self.length
        }
        //just to check the ownership concepts here ref is not passed and same value is returned, this should move the ownership to variable receiving the method return.
        fn return_self(self) -> Self {
            return self;
        }
        //without self
        //Self is nothing but an alias used for Rectangle  using Rectangle instead of Self also works fine
        fn square(size: u32) -> Self {
            Self {
                length: size,
                width: size,
            }
        }
    }

    let rect_1 = Rectangle {
        length: 20,
        width: 10,
    };
    let area: u32 = rect_1.calc_area();
    let is_square: bool = rect_1.check_square();
    println!("{area}");
    println!("{is_square}");

    let square: Rectangle = Rectangle::square(5); //calling associated fn which is not a method
    let area_sq: u32 = square.calc_area();
    println!("{area_sq}");

    //lets check some ownership concepts here
    //we have passed the reference of a instance using &self to the methods, lets see if rect_1 is still accessible
    println!("rect is {:?}", rect_1);
    //however if we choose to pass the value instead of ref to one of the methods the value of rec_1 will be moved and dropped after the method is finished.

    let rect_2: Rectangle = rect_1.return_self();
    println!("rect moved to rect_2 is {:?}", rect_2); //works as expected
}
