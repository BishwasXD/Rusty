
pub fn smart_pointers() {
    //they are the data structures which has the functionality of a pointer and has additional features such as metadata
    //pointers allows data to have multiple owners by keeping tracks of the owners and when there is no owner it cleans the data

    //strings and vec<T> are also an example of smart pointers
    //smart pointers are implemented using structures , they implement deref and drop traits

    //TYPES OF SMART POINTERS:

    //BOX<T> : to point the data in the heap
    //BOX allows us to store data in the heap while the pointer to the data is stored in the stack.
    //mostly complex data which size are not known in compile time are stored in heap
    //however using BOX we can force integer type also to be stored in the heap while its addr resides on the stack
    //storing integer in the heap makes sense while dealing with linked list:



 

}
