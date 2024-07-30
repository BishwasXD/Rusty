pub fn array_basics(){
    //declaring array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers); // Normal formatter `{}` is not working because it is used for more basic types. `{:?}` is used for Debug formatting, which works with more complex types like arrays.


    // Looping to display individual elements
    for number in numbers {
        println!("{number}"); // Works because `{}` is for displaying a single value at a time.
    }

    let _fruits = ["apple", "orange"]; // Can be declared without type as Rust can infer the type
    let food: [&str; 2] = ["pizza", "burger"];
    println!("{:?}", food);

    // Usually, arrays are homogenous, meaning you cannot have different data types in the same array.
    // This would not be valid:
    // let _hetero = ["apple", 1]; // It will expect the second element to be of the same type as the first one, which is `&str`.

    // Basic operations with arrays:

    // .len() to find the length
    println!("Length of numbers array: {}", numbers.len());

    // Mutability with arrays:
    // Arrays in Rust have a fixed size, but the elements can be changed if the array is mutable.
    let mut countries: [&str; 2] = ["Lesotho", "Botswana"];
    countries[1] = "Chad";
    println!("{:?}", countries);

    // Iterating over arrays:
    // There are two ways of iterating over an array: using a normal for loop or using the `iter` method.
    // For simple array operations where you only need to access data, a normal for loop is just fine.
    // However, if you need more methods and control, use the `iter` method.
    // `iter` comes with a bunch of other functions like map, filter, etc.

    // In direct iteration, the array elements are copied, but with `iter`, only a reference to the elements is copied (borrowed).
    // Using `iter_mut` allows you to change the array data.
    
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr.iter()); // Returns an iterator

    // Memory with arrays:
    // Arrays are stored in the stack due to their fixed size, which makes accessing array elements very fast.
    // For an array with 5 elements of `i32`, it takes 32 * 5 bits (or 4 bytes * 5 elements = 20 bytes).
    // Using pointers, we can get the address of the first element and add to the pointer to get the addresses of other elements.

    let ptr: *const i32 = arr.as_ptr();
    unsafe {
        for i in 0..arr.len() {
            println!("Address of element {}: {:?}", i, ptr.add(i));
            println!("Value at that address: {}", *ptr.add(i));
        }
    }
}
