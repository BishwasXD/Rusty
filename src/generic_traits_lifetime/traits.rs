use std::fmt::Display;

//This file is not running if u can/want u can fix it,no errors showing either
//Traits define the functionality a particular type has and can share with other types
pub fn traits() {
    println!("This is working");
    pub trait Summary {
        //defining trait
        fn summarize(&self) -> String;
    }
    //here in this case Tweet and NewsArticle share the same trait named Summary
    pub struct Tweet {
        pub username: String,
        pub content: String,
    }
    //using traits : impl Trait for type
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}:{}", self.username, self.content)
        }
    }
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
    }
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{},{}", self.headline, self.location)
        }
    }
    //using tweet and summarize trait
    let tweet = Tweet {
        username: String::from("title"),
        content: String::from("there be the content"),
    };

    println!("Tweet is:{}", tweet.summarize());
    //Traits as parameters
    //we can call any method on item that can come from Summary trait,such as summarize .We can
    //call notify and pass in any instant of NewsArticle or Tweet.Any type that dont implement
    //Summary type with will cause error.
    pub fn notify(item: &impl Summary) {
        println!("Breaking news!{}", item.summarize());
    }
    //Trait Bound Syntax
    //doing this is as same as above replacing &impl summary
    pub fn notifi<T: Summary>(item: &T) {
        println!("Breaking news!{}", item.summarize());
    }
    //Trait bounds with where clauses
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

    //we can write above as
    // fn some_functions<T, U>(t: &T, u: &U) -> i32
    // where
    // T: Display + Clone,
    // U: Clone + Debug,
    // {};
    // ****************Remaining:Returning Types that implement traits
}
