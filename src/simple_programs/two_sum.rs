use std::collections::HashMap;

pub fn two_sum(target: i32, nums: [i32; 4]) -> Vec<i32> {
    let mut map: HashMap<i32, usize> =  HashMap::new();
    let mut res: Vec<i32> = Vec::new();

    for i in 0..nums.len() {
        map.insert(nums[i], i);
    }

    for i in 0..nums.len() {
        let difference: i32 = target - nums[i];

        // let Some(&index)= map.get(&difference); 
        // println!("{}", index);  // cant do this without else codn need to handle when val not found case

        if let Some(&index)= map.get(&difference) {
            if index != i {
                res.push(i as i32);
                res.push(index as i32);
                println!("{:?}", res);
                return res;
            }
        }
    }

    res
}


//usize represents the unsigned integer, here we are using usize to represent indices which are non negative.
//also usize is architecture dependent, in 64bit system it takes 64bit and 32 in 32 bit system.
// It is typically used for array indices, lengths, and sizes because it matches the maximum addressable memory size of the system.
//isize also


//Some -> it is enum that comes with the option type, option type is used to rep a value which might be present or absent(similar to our case where the value might not be in map)
          //in our case, map.get return as Some(&val) when the value exists for a given key and return none when no val exists.
          //using let Some(&index) we are actually destructuring the Some and index will take the value inside Some(T), where T is the type.

//OPtion type -> in rust option type is used to represent a value that is some value Some(T) or nothing None.
                  //helps to handle the situation where there could be absence of a value.
                  // consider a scenario where u try to access a database but not sure the value exists or not.