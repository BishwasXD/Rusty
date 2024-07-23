pub fn _inner_loops(){
    let mut counter = 0;
    let result= loop{
       counter+=1;

       if counter == 10{
        break counter*2; //*returns counter*2 after break */
       }
    };
    println!("The result is {result}");
}

//Labeling the loops
pub fn _label_loops() {
 let mut count = 0;
 //label the loop using ('name)
 'counting_up: loop {
 println!("count = {count}");
 let mut remaining = 10;
 loop {
 println!("remaining = {remaining}");
 if remaining == 9 {
 break;
 }
 if count == 2 {
 break 'counting_up; //break counting_up loop
 }
 remaining -= 1;
 }
 count += 1;
 }
 println!("End count = {count}");
}

//For loop
pub fn _for_loops(){
    let a=[10,20,30,40,50];
 for element in a {
    println!("The value is: {element}");
 }
 //reversing the range using rev
 //decreases from n-1
 for number in (1..10).rev(){
    println!("{number}");
    println!("Decreasing");
 }
}

