use std::collections::HashMap; //hashMap is not as popular as compared to string and vectors so it is not brought into scope automatically.
pub mod hashmap {
    use std::collections::HashMap;

    pub fn hashmap() {
       //stores value in key value pair, hashMap<K, V> key of type K is mapped to value of type V.
       //it uses hashing function to determine where to place key val pair in memory.

        //creating a new hashMap
        let mut scores: HashMap<String, u8> = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("{:?}", scores);

        //accessing value from hashmap
        //one key thing to remember while trying to access, the value might be present or not, so the value will be either some or none

        let search_key = String::from("Blue");
        let returned  = scores.get(&search_key);
        match returned{
            Some(val) => println!("value is present and value is : {val}"),
            None => println!("Val not found")
        }

        //such hassle for such a simple operation
        //it handles the option by calling copied to get Option<u8> rather than Option<&u8> 
        //then it uses un_wrap_or to set score to zero if the key is not found, hence handling the case for both some and none.
        let score = scores.get(&search_key).copied().unwrap_or(0);
        println!("{score}");

        //iterating over hashMap
        for (key , val) in &scores{
            println!("{key}: {val}");
        }


       //hashMap and ownership
       //while inserting into a hashMap, we have to consider whether inserting the value copies the val in hashMap or moved
       //the ans is simple the datatype with copy trait are copied like integers, and string will be moved(obv).
       //for eg:
       let team_name = String::from("Green");
       scores.insert(team_name, 20);

        //println!("accessing team_name that wa just moved: {team_name}");//can't to this
        //we can use reference though.

        //updating in hashmap
        //we might want to update the value associated with certain key.

        //overwriting a value.
        //if we insert again with the key that is already in the map, then old value is overwritten.


        //Adding a Key and Value Only If a Key Isn’t Present
        //here we check if the key is present if yes we do nothing 
        //if no then we add key with new value

        scores.entry(String::from("Blue")).or_insert(50);
        scores.entry(String::from("Black")).or_insert(50);
        println!("{scores:?}");

        //for that we used entry, which takes a key as an arg and checks if the key is already present in the map or not
        //The return value of the entry method is an enum called Entry that represents a value that might or might not exist.
        //or_insert is the method from a Entry enum, it returns the mutable reference for the  key if the key exists in the map 
        //and if the key doesn't exists it inserts the param as the new value for the key.


        //UPDATING A VALUE BASED ON THE OLD VALUE(LIKE COUNTING FREQUENCY)
        let text = "hello world wonderful world";
        let mut freq_map: HashMap<&str, i32>  = HashMap::new();
        for word in text.split_whitespace()
        {
            let count: &mut i32 = freq_map.entry(word).or_insert(0);
            //if the key exists it returns the mutable ref to the value, that val is stored in count
            //after that we increase count simply to count its occurance.
           *count   += 1;
           //The * (dereference operator) is used in Rust to access the value that a reference points to.

        }
        println!("{freq_map:?}")

//RUST USES SipHash hashing function, this is not one of the fastest hashing algo but it provides better security
//however we can can switch to different hash function suiting our purpose.

        
    }
}
