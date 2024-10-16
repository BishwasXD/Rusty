//generics in function definition
//Note:rust type parameters names in Rust are short usually 1 letter.
pub fn generics() {
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest<T>(list: &[T]) -> &T {
        //declaring generics
        let mut largest = &list[0];
        for item in list {
            // if item > largest {
            //     //use std::cmp::PartialOrd or else error
            //     largest = item;
            // }
        }
        largest
        //since the code is same for largest_char and largest_i32,this generic function should work for
        //both,reduces code redundancy.
    }
    //In struct definition
    struct Point<T> {
        x: T,
        y: T,
    }
   // let wont_work: Point<i32> = Point { x: 5, y: 4.0 }; //mismatched data types
                                            //if u want to use two types of data in generics use different names
    struct Double<T, U> {
        x: T,
        y: U,
    }

    //enum definition
    enum Option<T> {
        Some(T),
        None,
    }
    //multiple generics in enums
    enum Result<T, E> {
        OK(T),
        Err(E),
    }
    // In method definition
    struct Points<T> {
        x: T,
        y: T,
    }
    impl<T> Points<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    //another one ,here x and y are values passed from Pointss of type x1 and y1
    struct Pointss<X1, Y1> {
        x: X1,
        y: Y1,
    }
    impl<X1, Y1> Pointss<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Pointss<X2, Y2>) -> Pointss<X1, Y2> {
            Pointss {
                x: self.x,
                y: other.y,
            }
        }
    }
}
