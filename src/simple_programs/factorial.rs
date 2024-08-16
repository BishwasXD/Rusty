pub fn calc_factorial(number: usize) -> usize{
    let mut num: usize = number;
    let mut fact: usize= 1;
    loop{
          fact = fact * (num);
          num= num - 1;
          if num == 1{
            println!("{fact}");
            return fact;
          }


    }

}