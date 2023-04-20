pub struct Solution;

impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut common_prefix: Vec<String> = vec![];
    for index in 0..Solution::find_shortest(&strs) {
      let mut result: Vec<String> = vec![];
      for item in &strs {
        result.push(item[index..index+1].to_string())
      }

      if !Solution::check_same_element(&result) {
        break;
      }

      common_prefix.push(result[0].clone())
    }

    return common_prefix.join("");
  }

  fn find_shortest(strs: &Vec<String>) -> usize {
    let mut length = strs[0].len();
    for str in strs {
      if str.len() < length {
        length = str.len();
      }
    }

    return length;
  }

  fn check_same_element(strs: &Vec<String>) -> bool {
    let initial = &strs[0];
    for item in strs {
      if item != initial {
        return false
      }
    }
    return true;
  }
}