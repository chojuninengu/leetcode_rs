pub struct Solution;

impl Solution {
  pub fn my_atoi(str: String) -> i32 {
    let mut s = str.chars().skip_while(|c| c.is_whitespace()).peekable();
    let mut sign = 1;
    match s.peek() {
      Some(&'+') => {
        s.next();
      },
      Some(&'-') => {
        sign = -1;
        s.next();
      },
      None => {
        return 0;
      }
      _ => {},
    }
    
    match s.peek() {
      Some(&c) => {
        if !c.is_ascii_digit() {
          return 0;
        }
      },
      None => {
        return 0;
      }
    }
    
    s
      .take_while(|c| c.is_ascii_digit())
      .collect::<String>()
      .parse::<i32>()
      .map(|n| n * sign)
      .unwrap_or(if sign == 1 {
        std::i32::MAX
      } else {
        std::i32::MIN
      })
  }
}

pub fn main() {
  assert_eq!(Solution::my_atoi("-42".to_string()), -42);
  assert_eq!(Solution::my_atoi("".to_string()), 0);
  assert_eq!(Solution::my_atoi("   4193 with words".to_string()), 4193);
  assert_eq!(Solution::my_atoi("words and 4193".to_string()), 0);
  assert_eq!(Solution::my_atoi("-91283472332".to_string()), std::i32::MIN);
  assert_eq!(
    Solution::my_atoi("9223372036854775808".to_string()),
    std::i32::MAX
  );
}

mod test {
  #[test]
  fn basics() {}
}
