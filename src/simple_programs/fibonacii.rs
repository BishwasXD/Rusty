

// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610
pub fn fibonacci(){
    //change this number
    let max =30;
    let mut first_num = 0;
    let mut second_num = 1;
   let mut count = 0;
   println!("Input max fib series numbers,2+N");
   println!("{first_num}");
    loop{
         if (count>max) {
            break;
        }
     let next_num = first_num+second_num;
        println!("{next_num}");
       second_num = first_num;
        first_num = next_num;
        count+=1;
}
}