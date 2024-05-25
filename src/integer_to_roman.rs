/*
# Integer to Roman

Seven different symbols represent Roman numerals with the following values:
- I -> 1
- V -> 5
- X -> 10
- L -> 50
- C -> 100
- D -> 500
- M -> 1000

Roman numerals are formed by appending the conversions of decimal place values from highest to lowest.
Converting a decimal place value into a Roman numeral has the following rules:

If the value does not start with 4 or 9, select the symbol of the maximal value that can be subtracted from the input, append that symbol to the result, subtract its value, and convert the remainder to a Roman numeral.
If the value starts with 4 or 9 use the subtractive form representing one symbol subtracted from the following symbol, for example, 4 is 1 (I) less than 5 (V): IV and 9 is 1 (I) less than 10 (X): IX. Only the following subtractive forms are used: 4 (IV), 9 (IX), 40 (XL), 90 (XC), 400 (CD) and 900 (CM).
Only powers of 10 (I, X, C, M) can be appended consecutively at most 3 times to represent multiples of 10. You cannot append 5 (V), 50 (L), or 500 (D) multiple times. If you need to append a symbol 4 times use the subtractive form.
Given an integer, convert it to a Roman numeral.
*/

pub fn solution(num: i32) -> String {
    let mut result = String::new();
    result.push_str(match num / 1000 {
        0 => "",
        1 => "M",
        2 => "MM",
        3 => "MMM",
        _ => "",
    });
    result.push_str(match num % 1000 / 100 {
        0 => "",
        1 => "C",
        2 => "CC",
        3 => "CCC",
        4 => "CD",
        5 => "D",
        6 => "DC",
        7 => "DCC",
        8 => "DCCC",
        9 => "CM",
        _ => "",
    });
    result.push_str(match num % 100 / 10 {
        0 => "",
        1 => "X",
        2 => "XX",
        3 => "XXX",
        4 => "XL",
        5 => "L",
        6 => "LX",
        7 => "LXX",
        8 => "LXXX",
        9 => "XC",
        _ => "",
    });
    result.push_str(match num % 10 {
        0 => "",
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "IX",
        _ => "",
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solution(3), "III".to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(solution(3749), "MMMDCCXLIX".to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(solution(58), "LVIII".to_string());
    }

    #[test]
    fn example_4() {
        assert_eq!(solution(1994), "MCMXCIV".to_string());
    }
}
