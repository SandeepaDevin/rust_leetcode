impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let input_vec: Vec<&str> = s.split_whitespace().collect();
        let length: i32 = input_vec[ input_vec.len() -1 ].len() as i32;
        length
    }
}
