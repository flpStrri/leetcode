// # Roman to Integer
// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
//
// | Symbol | Value |
// |--------|-------|
// | I      | 1     |
// | V      | 5     |
// | X      | 10    |
// | L      | 50    |
// | C      | 100   |
// | D      | 500   |
// | M      | 1000  |
//
// For example, 2 is written as II in Roman numeral, just two ones added together.
// 12 is written as XII, which is simply X + II.
// The number 27 is written as XXVII, which is XX + V + II.
//
// Roman numerals are usually written largest to smallest from left to right.
// However, the numeral for four is not IIII.
// Instead, the number four is written as IV.
// Because the one is before the five we subtract it making four.
// The same principle applies to the number nine, which is written as IX.
// There are six instances where subtraction is used:
//
// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.
// Given a roman numeral, convert it to an integer.

pub fn solution(s: String) -> i32 {
    let mut result = 0;
    let mut previous_numeral = ' ';

    for char in s.chars() {
        match (char, previous_numeral) {
            ('V', 'I') => result += 3,
            ('X', 'I') => result += 8,
            ('L', 'X') => result += 30,
            ('C', 'X') => result += 80,
            ('D', 'C') => result += 300,
            ('M', 'C') => result += 800,
            ('I', _) => result += 1,
            ('V', _) => result += 5,
            ('X', _) => result += 10,
            ('L', _) => result += 50,
            ('C', _) => result += 100,
            ('D', _) => result += 500,
            ('M', _) => result += 1000,
            _ => (),
        }
        previous_numeral = char;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "III".to_string();
        assert_eq!(solution(s), 3);
    }
    #[test]
    fn example_2() {
        let s = "LVIII".to_string();
        assert_eq!(solution(s), 58);
    }

    #[test]
    fn example_3() {
        let s = "MCMXCIV".to_string();
        assert_eq!(solution(s), 1994);
    }

    #[test]
    fn example_4() {
        let s = "IV".to_string();
        assert_eq!(solution(s), 4);
    }

    #[test]
    fn example_5() {
        let s = "IX".to_string();
        assert_eq!(solution(s), 9);
    }
}
