pub struct Deposits {
    pub checks: Vec<String>,
    pub output_strings: Vec<Vec<(String, String, String)>>,
    pub prices: Vec<Vec<f64>>,
    pub count: usize,
}

impl Deposits {
    pub fn get_sum(&self, index: usize) -> f64 {
        if index > self.count {
            return 0.0;
        }

        let mut sum: f64 = 0.0;
        for i in 0..self.prices[index].len() {
            sum += self.prices[index][i];
        }

        sum
    }

    pub fn push_tuple (&mut self, check: String, date: String, unit: String, amount: String) -> () {
        let mut index: Option<usize> = None;

        for i in 0..self.count {
            if self.checks.get(i).unwrap() == &check {
                index = Some(i);
                break;
            }
        }

        let outer_index = match index {
            Some(i) => i,
            None => {
                self.push_check(check);
                self.count - 1
            }
        };

        let number: f64 = amount.parse().unwrap();
        let outer_vector = &mut self.output_strings[outer_index];
        let prices = &mut self.prices[outer_index];

        outer_vector.push((date, unit, amount));
        prices.push(number);
    }

    pub fn push_check (&mut self, check: String) -> () {
        self.checks.push(check);
        self.output_strings.push(Vec::new());
        self.prices.push(Vec::new());
        self.count += 1;
    }
}