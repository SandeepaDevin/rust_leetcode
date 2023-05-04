impl Solution {
    pub fn reverse(x: i32) -> i32 {

        if x < 0 {
            
            let signed_string = x.to_string();
            let (sign, unsigned_string) = signed_string.split_at(1);
            let mut unsigned_chars: Vec<char> = unsigned_string.chars().collect();
            unsigned_chars.reverse();
            let reversed_string = unsigned_chars.iter().collect::<String>();
            let result_string = format!("{}{}", sign, reversed_string);
            let result = std::panic::catch_unwind(|| {
                result_string.parse::<i32>().unwrap()
            });
            match result {
                Ok(num) => num,
                Err(error)=> 0
            }

        } else {
            
            let unsigned_string = x.to_string();
            let mut unsigned_chars: Vec<char> = unsigned_string.chars().collect();
            unsigned_chars.reverse();
            let reversed_string = unsigned_chars.iter().collect::<String>();
            let result = std::panic::catch_unwind(|| {
                reversed_string.parse::<i32>().unwrap()
            });
            match result {
                Ok(num) => num,
                Err(error)=> 0
            }
        }
    }
}
