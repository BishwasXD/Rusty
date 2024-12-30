pub fn macros(){
//macros are the way of writing code that writes other code
//macros reduce the code u have to write also macros defination are complex to understand
//they arepowerful compared to normal function as they can take any number of arg of any type
//one such example is vec! macro we use to initilize a vec with value withouT proper defination of
//that vector when we write vec![] what happens under the hood is:

// /#[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }
//The #[macro_export] annotation indicates that this macro should be made available whenever the crate in which 
//the macro is defined is brought into scope
//macro is defined using macro_rules! followed by the name of macro without ! mark
//if pattern is matched first arm is executed, complex macros got more than one arm 
//dollar sign is used to declare  variable that will contain the rust code 
//dollar sign distinguishes normal and macro variable:
}
