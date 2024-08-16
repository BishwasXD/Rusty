pub fn build_pattern(blueprint: &str) -> String {
    let mut pattern = String::new();
    let mut i = 0;

    while i < blueprint.len() {
        let character = blueprint.chars().nth(i).expect("Failed to get character");
        i += 1;

        if i < blueprint.len() {
            let range_str = blueprint
                .chars()
                .nth(i)
                .expect("Failed to get character for range")
                .to_string();
            let mut range: u32 = range_str.parse().expect("Failed to parse string to integer");

            while range > 0 {
                pattern.push(character);
                range -= 1;
            }
        }

        i += 1;
    }

    println!("{}", pattern);
    pattern
}
