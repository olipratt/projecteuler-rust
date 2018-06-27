use std::cmp::max;
use std::collections::HashMap;

fn sum_of_integers_1_to_n(n: u64) -> u64 {
    ((1 + n) * n) / 2
}

fn sum_of_multiples_of_x_less_than_n(n: u64, x: u64) -> u64 {
    sum_of_integers_1_to_n((n - 1) / x) * x
}

pub fn sum_of_mults_of_3_and_5_lt_n(n: u64) -> u64 {
    let mults_of_3 = sum_of_multiples_of_x_less_than_n(n, 3);
    let mults_of_5 = sum_of_multiples_of_x_less_than_n(n, 5);
    let mults_of_15 = sum_of_multiples_of_x_less_than_n(n, 15);

    let result = mults_of_3 + mults_of_5 - mults_of_15;
    result
}

struct Fib {
    prev: u64,
    prev_prev: u64,
}

impl Fib {
    pub fn new(first_term: u64, second_term: u64) -> Self {
        Self {
            prev_prev: first_term,
            prev: second_term,
        }
    }

    pub fn next(&mut self) -> u64 {
        let new_term = self.prev + self.prev_prev;
        self.prev_prev = self.prev;
        self.prev = new_term;

        new_term
    }
}

pub fn sum_even_fib_terms_less_than(n: u64) -> u64 {
    let mut fib = Fib::new(1, 2);
    let mut sum = 2;
    let mut next_term = fib.next();

    while next_term < n {
        if next_term % 2 == 0 {
            sum += next_term;
        }
        next_term = fib.next();
    }

    sum
}

fn prime_factors(n: u64) -> HashMap<u64, u64> {
    let mut result = HashMap::new();
    let mut possible_factor = 2;
    let mut remainder = n;

    while possible_factor <= (remainder / 2) {
        while remainder % possible_factor == 0 {
            let count = result.entry(possible_factor).or_insert(0);
            *count += 1;

            remainder /= possible_factor;
        }
        possible_factor += 1;
    }

    {
        let count = result.entry(remainder).or_insert(0);
        *count += 1;
    }

    result
}

pub fn largest_prime_factor(n: u64) -> u64 {
    prime_factors(n).keys().cloned().max().unwrap()
}

fn int_as_digit_vec(mut num: u64) -> Vec<u8> {
    let mut result = Vec::new();

    while num > 0 {
        result.push((num % 10) as u8);
        num = num / 10;
    }

    result
}

pub fn multiples_contain_same_digits(num_multiples: u64) -> u64 {
    let mut num = 0;
    'outer: loop {
        num += 1;
        let mut first_vec = int_as_digit_vec(num);
        first_vec.sort();
        for multiple in 2..(num_multiples + 1) {
            let mut next_vec = int_as_digit_vec(num * multiple);
            next_vec.sort();
            if first_vec != next_vec {
                continue 'outer;
            }
        }
        break;
    }

    num
}

fn is_product_of_3_two_digit_nums(n: u64) -> bool {
    for x in (100..1000).rev() {
        if n % x == 0 {
            let y = n / x;
            if y >= 100 && y <= 999 {
                return true;
            }
        }
        if n / x > 1000 {
            break;
        }
    }

    false
}

pub fn largest_6_digit_palindrome_prod_of_2_three_dig_nums() -> u64 {
    for x in (100..999).rev() {
        let mut palindrome = x;
        palindrome = palindrome * 10 + (x % 10);
        palindrome = palindrome * 10 + ((x / 10) % 10);
        palindrome = palindrome * 10 + ((x / 100) % 10);

        if is_product_of_3_two_digit_nums(palindrome) {
            return palindrome;
        }
    }

    panic!("Failed to find palindrome!")
}

pub fn evenly_divisible_by_nums_up_to_n(n: u64) -> u64 {
    let mut result = 1;
    let mut factors = HashMap::new();

    for x in 2..(n + 1) {
        let new_factors = prime_factors(x);
        for (factor, count) in &new_factors {
            let curr_count = factors.entry(*factor).or_insert(*count);
            *curr_count = max(*curr_count, *count);
        }
    }

    for (factor, count) in &factors {
        result *= factor.pow(*count as u32);
    }

    result
}

pub fn score_names<'a, I>(names: I) -> u64
where
    I: IntoIterator<Item = &'a str>,
{
    let mut total = 0;

    for (i, name) in names.into_iter().enumerate() {
        let mut name_total = 0;
        for letter in name.bytes() {
            name_total += (letter - b'A' + 1) as u64;
        }
        total += name_total * ((i + 1) as u64);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn problem1() {
        assert_eq!(233168, sum_of_mults_of_3_and_5_lt_n(1000));
    }

    #[test]
    fn problem2() {
        assert_eq!(4613732, sum_even_fib_terms_less_than(4000001));
    }

    #[test]
    fn problem3() {
        assert_eq!(6857, largest_prime_factor(600851475143));
    }

    #[test]
    fn problem4() {
        assert_eq!(
            906609,
            largest_6_digit_palindrome_prod_of_2_three_dig_nums()
        );
    }

    #[test]
    fn problem5() {
        assert_eq!(232792560, evenly_divisible_by_nums_up_to_n(20));
    }

    #[test]
    fn problem22() {
        assert_eq!(53, score_names(vec!["COLIN"]));
        let mut f = File::open("p022_names.txt").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Failed to read file");
        let mut names = contents
            .split(',')
            .map(|x| x.trim_matches('"'))
            .collect::<Vec<_>>();
        names.sort();
        assert_eq!(871198282, score_names(names));
    }

    #[test]
    fn problem52() {
        assert_eq!(1, multiples_contain_same_digits(1));
        assert_eq!(125874, multiples_contain_same_digits(2));
        assert_eq!(142857, multiples_contain_same_digits(3));
        assert_eq!(142857, multiples_contain_same_digits(4));
        assert_eq!(142857, multiples_contain_same_digits(5));
        assert_eq!(142857, multiples_contain_same_digits(6));
    }
}
