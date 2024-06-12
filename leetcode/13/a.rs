impl Solution {
    pub fn value(s: char) -> i32 {
        match s {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
    pub fn roman_to_int(s: String) -> i32 {
        let mut num: i32 = 0;
        for (i, ch) in s.chars().enumerate() {
            if (i+1)!=s.len() && (Self::value(s.as_bytes()[i+1] as char)>Self::value(ch)) {
                num -= Self::value(ch)
            } else {
            num += Self::value(ch);
            }
        }
        num
    }
}