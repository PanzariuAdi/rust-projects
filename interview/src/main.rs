use std::collections::HashMap;

#[warn(dead_code)]
struct Solution {}

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut temp: Vec<i32> = Vec::new();
        let mut max_number: i32 = -1;

        for num in arr.iter().rev() {
            temp.push(max_number);
            if num > &max_number {
                max_number = *num;
            }
        }

        temp.reverse();

        temp
    }

    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_idx = 0;

        for t_char in t.chars() {
            match s.chars().nth(s_idx) {
                Some(s_char) => {
                    if t_char == s_char {
                        s_idx += 1;
                    }
                }
                None => return true,
            }
        }

        s_idx == s.len()
    }

    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.trim();

        let mut size = 0;

        for c in s.chars().rev() {
            if c == ' ' {
                return size;
            }

            size += 1;
        }

        size
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result = Vec::new();

        for (idx, num) in nums.iter().enumerate() {
            let complement: i32 = target - num;

            if let Some(&complement_idx) = map.get(&complement) {
                result.push(complement_idx);
                result.push(idx as i32);
                break;
            }

            map.insert(num, idx as i32);
        }

        result
    }

    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs.first().unwrap().to_string();
        }

        strs.sort();

        if strs.first().unwrap() == "" {
            return String::from("");
        }

        let mut idx = 0;

        loop {
            let first_word = strs.first().unwrap();
            let last_word = strs.last().unwrap();

            if first_word.len() <= idx || last_word.len() <= idx {
                break;
            }

            if first_word.chars().nth(idx) != last_word.chars().nth(idx) {
                break;
            }

            idx += 1;
        }

        if idx == 0 {
            return String::from("");
        }

        return strs.first().unwrap()[0..idx].to_string();
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();

        for str in strs.iter() {
            let mut freq_array = vec![0; 28];

            for ch in str.chars() {
                freq_array[ch as usize - 'a' as usize] += 1;
            }

            map.entry(freq_array)
                .or_insert_with(Vec::new)
                .push(str.to_string());
        }

        for (_, vector) in map {
            result.push(vector);
        }

        return result;
    }

    pub fn generate_pascal_triangle(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);

        for row_idx in 0..num_rows {
            let mut row = vec![1; row_idx as usize + 1];

            for col_idx in 1..row_idx {
                row[col_idx as usize] = triangle[row_idx as usize - 1][col_idx as usize - 1]
                    + triangle[row_idx as usize - 1][col_idx as usize];
            }

            triangle.push(row);
        }

        triangle
    }

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[idx] = nums[i];
                idx += 1;
            }
        }

        idx as i32
    }

    pub fn num_unique_emails(email: Vec<String>) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_elements_case_1() {
        let input: Vec<i32> = vec![17, 18, 5, 4, 6, 1];
        let output: Vec<i32> = vec![18, 6, 6, 6, 1, -1];
        assert_eq!(Solution::replace_elements(input), output);
    }

    #[test]
    fn is_subsequence_case_1() {
        let s: String = "abc".to_string();
        let t: String = "ahbgdc".to_string();
        assert_eq!(Solution::is_subsequence(s, t), true);
    }

    #[test]
    fn is_subsequence_case_2() {
        let s: String = "axc".to_string();
        let t: String = "ahbgdc".to_string();
        assert_eq!(Solution::is_subsequence(s, t), false);
    }

    #[test]
    fn is_subsequence_case_3() {
        let s: String = "".to_string();
        let t: String = "ahbgdc".to_string();
        assert_eq!(Solution::is_subsequence(s, t), true);
    }

    #[test]
    fn length_of_last_word() {
        let word1: String = "Hello World".to_string();
        let word2: String = "    fly me   to     the   moon   ".to_string();
        assert_eq!(Solution::length_of_last_word(word1), 5);
        assert_eq!(Solution::length_of_last_word(word2), 4);
    }

    #[test]
    fn two_sum() {
        let input = vec![2, 7, 11, 15];
        let expected = vec![0, 1];

        assert_eq!(Solution::two_sum(input, 9), expected);
    }

    #[test]
    fn longest_common_prefix() {
        let words1 = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let words2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

        assert_eq!(Solution::longest_common_prefix(words1), "fl");
        assert_eq!(Solution::longest_common_prefix(words2), "");
    }

    #[test]
    fn pascal_triangle() {
        let expected = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];

        assert_eq!(Solution::generate_pascal_triangle(5), expected);
    }
}

fn main() {}
