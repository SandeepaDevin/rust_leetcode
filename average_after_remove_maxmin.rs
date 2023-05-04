impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut min= 0;
        let mut max= 0;

        for (i, &x) in salary.iter().enumerate() {
            if x < salary[min] {
                min = i;
            }
            if x > salary[max] {
                max = i;
            }
        }

        
        let mut salary_mut = salary.clone();
        salary_mut.retain(|&x| x != salary[max]);
        salary_mut.retain(|&x| x != salary[min]);

        let mut sum: f64 = 0.0;
        for (i, &x) in salary_mut.iter().enumerate() {
            sum += x as f64 ;
               
        }

        sum/salary_mut.len() as f64

    }
}
