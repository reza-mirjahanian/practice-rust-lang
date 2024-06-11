impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
      let mut n = x;
        if n < 0 || (n != 0 && n % 10 == 0) {
            return false;
        } else {
            let mut tmp = 0;
            while n > 0 {
                tmp = tmp * 10 + (n % 10);
                n /= 10;
            }
            return tmp == x;
        }

    }
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        return x.to_string().chars().rev().eq(x.to_string().chars())
    }
}