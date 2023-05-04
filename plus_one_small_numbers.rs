impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {

        fn get_digits(mut number: u128) -> Vec<u128> {
            let mut digits = Vec::new();
            while number > 0 {
                digits.push(number % 10);
                number /= 10;
            }
            digits.reverse();
            digits
        }
       
        let concatenated = digits.iter().fold(0, |acc, &x| acc * 10 + ( x as u128));
        let incremented = concatenated + 1;
        let digits = get_digits(incremented);
        let i32_vec = digits.iter().map(|&x| x as i32).collect::<Vec<i32>>();
        i32_vec
    }
}
