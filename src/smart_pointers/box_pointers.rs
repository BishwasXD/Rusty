use std::ops::Deref;

pub fn box_pointers(){
    let b = Box::new(5);
    println!("b is {b}");

    //enabling recursive types with the BOX
    //when u want to store a data but it size can't be known at the comp time but u are using that data in the context that requires size known at comp time.
    //example with enum

    enum List {
        //Cons(i32, List), //this we cant really tell about size because this can go up to infinite
        Cons(i32, Box<List>), //now the cons has to store i32 + size of the box pointer data
        Nil, //this has size zero
    }
    //cons is the datastructure which is similar to LinkedList.
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil)))))); //1- > 2 -> 3 -> Nil

    // match list {
    //     List::Cons(value, next) => println!("{value}"),
    //     List::Nil => println!("encountered nil"),
    // }
    //this will only print one coz we are ignoring a whole chain after one.
    
    show_list(&list);
    fn show_list(list: &List) {
        match list {
            List::Cons(value, next) => {
                println!("{value}");
                show_list(next);
            }
            List::Nil => println!("encountered nil"),
        }
    }
    //when BOX<T> goes out of scope the heap it is pointing to is cleaned up

    //Treating smart pointers like regular references with deref trait
    //implementing deref trait allows us to customize the behavior of dereference operator *
    //by customizing deref in that way it enables the smart pointer to be treated as regular reference

    //eg of deference operator for a regular reference
   { let x = 5;
    let y = &x ;//ref of x
    assert_eq!(5, x);
    //assert_eq!(5, y); //cant compare number and reference, they are of different types

    assert_eq!(5,*y)
    } //after dereferencing 

    //using BOX<T>
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, *y); //5 is in heap and addr is in stack which is y so deref is needed
    //however while printing deref is not needed can directly print y
    //let z =x + y; //this is not valid use deref on y


    //Defining our own smart pointer
    #[derive(Debug)]
    struct MyBox<T>(T);
    impl <T> MyBox<T> {
      fn new(x :T) -> MyBox<T> {
        MyBox(x)
      }
    }
    let m: MyBox<i32> = MyBox::new(5);
    println!("{m:?}"); //instance of mybox does nothing

    //implementing deref trait on mybox

    impl <T> Deref for MyBox<T> {
      type Target = T;//defining associate type TODO: look into associate type
      fn deref(&self) -> &Self::Target {
          &self.0 //returns ref to first value of tuple structure
      }
        
    }
    let n = MyBox::new(20);
    assert_eq!(20, *n); //*n is actually running *(n.deref()) */

    //When we have a reference, like &i32, we can use the * operator to dereference it and access the value it points to
    //but in case of custom types like MyBox tuple Without some additional mechanism, the compiler wouldn’t know how to automatically convert Box<T> to a reference &T.



    //DEREF COERCIONS:
    //Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type. 
    //For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str

    let name: MyBox<String> = MyBox::new(String::from("XYZ"));
   //let name = String::from("XYZ"); //this bydefault follows deref trait

    //when this function is called rust uses deref corecion to convert &String to &str
    //since MyBox implements the deref trait rust will call deref method for this type conversion 
    //what happens under the hood: let temp: &MyBox<String> = &name;
    //why &String to &str ?
    //-> If a function only needs the character data (not the whole String object), it should take a &str.
    //&str points directly to the string content whereas &String points to the string object which contains the metadata as well.
    //however if we wish to pass to mutate we need to pass &String

     hello(&name);
   

    fn hello(name: &str) {
      println!("Hello, {name}!");
  }

}