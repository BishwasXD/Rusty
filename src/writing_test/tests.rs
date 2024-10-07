//To change function into test function add #[test] before fn
//run tests with "cargo test" all tests on the project

//every 
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    //auto generated test
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//Test to fail

#[cfg(test)]
mod testers{
    #[test]
    fn exploration(){
        assert_eq!(2+2,4);
    }
    #[test]
    fn another(){
        panic!("Makes it go boom")
    }
}