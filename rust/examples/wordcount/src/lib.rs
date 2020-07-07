//! wordcount provides simple functions for counting frequencies of chars, words or lines.
//! For more detail, see docs for the count func[`count`](fn.count.html).
#![warn(missing_docs)]

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// An option for [`count`](fn.count.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// By chars
    Char,
    /// By words
    Word,
    /// By lines
    Line,
}

/// Default option is [`Word`](enum.CountOption.html#variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// read lines from input, and count frequency of them.
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): Unicode chars
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word): regexp
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line): \n or \r\n
///
/// # Examples
/// example for counting words in input.
///
/// ```
/// use std::io::Cursor;
/// use wordcount::{count, CountOption};
///
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
///
/// assert_eq!(freq["aa"], 1);
/// assert_eq!(freq["bb"], 2);
/// assert_eq!(freq["cc"], 1);
/// ```
///
/// # Panics
/// When input is not UTF-8
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CountOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }
    freqs
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    use std::io::Cursor;

    #[test]
    fn word_count_works() {
        let mut exp = HashMap::new();
        exp.insert("aa".to_string(), 1);
        exp.insert("bb".to_string(), 2);
        exp.insert("cc".to_string(), 1);

        assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
    }

    #[test]
    fn word_count_works2() {
        let mut exp = HashMap::new();
        exp.insert("aa".to_string(), 1);
        exp.insert("cc".to_string(), 1);
        exp.insert("dd".to_string(), 1);

        assert_eq!(count(Cursor::new("aa cc dd"), CountOption::Word), exp);
    }

    #[test]
    fn assert_test() {
        let a = 3;
        let b = 27;
        assert!(a + b == 30, "a = {}, b = {}", a, b);
    }

    #[test]
    fn result_test() -> io::Result<()> {
        use std::fs::{read_to_string, remove_file, write};
        write("test.txt", "message")?;
        let message = read_to_string("test.txt")?;
        remove_file("test.txt")?;
        assert_eq!(message, "message");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn word_count_do_not_contain_unknown_words() {
        use std::io::Cursor;

        count(
            Cursor::new([b'a', 0xf0, 0x90, 0x80, 0xe3, 0x81, 0x82]),
            CountOption::Word,
        );
    }

    #[test]
    #[ignore]
    fn large_test() {}

    #[test]
    fn word_count_works3() {
        use std::io::Cursor;

        let freqs = count(Cursor::new("aa cc dd"), CountOption::Word);

        assert_eq!(freqs.len(), 3);
        assert_eq!(freqs["aa"], 1);
        assert_eq!(freqs["cc"], 1);
        assert_eq!(freqs["dd"], 1);
    }

    macro_rules! assert_map {
    ($expr: expr, {$($key: expr => $value:expr), *}) => {
        $(assert_eq!($expr[$key], $value));*
        };
    }

    #[test]
    fn word_count_works4() {
        use std::io::Cursor;

        let freqs = count(Cursor::new("aa cc dd"), CountOption::Word);

        assert_eq!(freqs.len(), 3);

        assert_map!(freqs, {"aa" => 1, "cc" => 1, "dd" => 1});
    }
}
