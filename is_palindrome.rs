impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let first_string = x.to_string();
        let mut pali_vect: Vec<char> = x.to_string().chars().collect();
        pali_vect.reverse();
        let mut second_string = pali_vect.iter().collect::<String>();
        if first_string == second_string{
            true
        }
        else{
            false
        }
    }
}
