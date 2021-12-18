use std::str::FromStr;
use std::collections::BTreeSet;

const INPUT: &str = include_str!("../data/8/input");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Segment { A,B,C,D,E,F,G }

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Segments (
    BTreeSet::<Segment>,
);

#[derive(Debug, PartialEq, Eq)]
struct Notes {
    patterns: Vec<Segments>,
    outputs: Vec<Segments>,
}

#[derive(Debug, PartialEq, Eq)]
struct DigitKey(
    [Option<Segments>; 10]
);

impl Segments {
    fn char_to_segment(c: char) -> Segment {
        use Segment::*;
        match c {
            'a' => {A},
            'b' => {B},
            'c' => {C},
            'd' => {D},
            'e' => {E},
            'f' => {F},
            'g' => {G},
            _ => panic!("Unknown segment {}", c),
        }
    }

    fn is_zero(self: &Self, key: &DigitKey) -> bool {
        self.count() == 6 &&
        self.overlap(&key.encode(1).unwrap()) == 2 &&
        self.overlap(&key.encode(7).unwrap()) == 3 &&
        self.overlap(&key.encode(4).unwrap()) == 3 
    }

    fn is_one(self: &Self) -> bool {
        self.0.len() == 2
    }

    fn is_two(self: &Self, key: &DigitKey) -> bool {
        self.count() == 5 &&
        self != &key.encode(3).unwrap() &&
        self != &key.encode(5).unwrap()
    }

    fn is_three(self: &Self, key: &DigitKey) -> bool {
        self.count() == 5 &&
        self.overlap(&key.encode(1).unwrap()) == 2
    }

    fn is_four(self: &Self) -> bool {
        self.0.len() == 4
    }

    fn is_five(self: &Self, key: &DigitKey) -> bool {
        self.count() == 5 && 
        self.overlap(&key.encode(6).unwrap()) == 5
    }

    fn is_six(self: &Self, key: &DigitKey) -> bool {
        self.count() == 6 && 
        self.overlap(&key.encode(1).unwrap()) == 1
    }

    fn is_seven(self: &Self) -> bool {
        self.0.len() == 3
    }

    fn is_eight(self: &Self) -> bool {
        self.0.len() == 7
    }

    fn is_nine(self: &Self, key: &DigitKey) -> bool {
        self.count() == 6 && 
        self.overlap(&key.encode(4).unwrap()) == 4
    }

    fn is_unique_segments_digit(self: &Self) -> bool {
        self.is_one() || self.is_four() || self.is_seven() || self.is_eight()
    }
    
    fn count(self: &Self) -> usize {
        self.0.len()
    }

    fn overlap(self: &Self, other: &Segments) -> usize {
        self.0.intersection(&other.0).count()
    }
}

impl FromIterator<Segment> for Segments {
    fn from_iter<T> (iter: T) -> Self 
        where T: IntoIterator<Item=Segment>
    {
        Segments(iter.into_iter().collect::<BTreeSet<Segment>>())
    }
}

#[derive(Debug)]
struct ParseSegmentsErr{}
impl FromStr for Segments {
    type Err = ParseSegmentsErr;
    fn from_str(s: &str) -> Result<Segments, Self::Err> {
        Ok(s.chars().map(Segments::char_to_segment).collect())
    }
}

impl DigitKey {
    fn new() -> Self {
        Self(Default::default())
    }

    fn update(self: &mut Self, number: u8, pattern: Segments) {
        self.0[number as usize] = Some(pattern);
    }

    fn decode(self: &Self, pattern: &Segments) -> Option<u8> {
        self.0.iter().position(|p| p == &Some(pattern).cloned()).map(|v| v as u8)
    }

    fn encode(self: &Self, number: u8) -> Option<Segments> {
        self.0
            .iter()
            .enumerate()
            .find(|(n, _)| *n == number as usize)
            .map(|(_, seg)| seg.clone())
            .unwrap()
    }
}

fn decode_patterns(patterns: &Vec<Segments>) -> DigitKey {
    let mut key = DigitKey::new();
    key.update(1, patterns.iter().find(|p| p.is_one()).expect("One pattern not found").clone());
    key.update(4, patterns.iter().find(|p| p.is_four()).expect("Four pattern not found").clone());
    key.update(7, patterns.iter().find(|p| p.is_seven()).expect("Seven pattern not found").clone());
    key.update(8, patterns.iter().find(|p| p.is_eight()).expect("Eight pattern not found").clone());
    key.update(0, patterns.iter().find(|p| p.is_zero(&key)).expect("Zero pattern not found").clone());
    key.update(6, patterns.iter().find(|p| p.is_six(&key)).expect("Six pattern not found").clone());
    key.update(9, patterns.iter().find(|p| p.is_nine(&key)).expect("Nine pattern not found").clone());
    key.update(3, patterns.iter().find(|p| p.is_three(&key)).expect("Three pattern not found").clone());
    key.update(5, patterns.iter().find(|p| p.is_five(&key)).expect("Five pattern not found").clone());
    key.update(2, patterns.iter().find(|p| p.is_two(&key)).expect("Two pattern not found").clone());
    key
}

fn decode_digits(digits: &Vec<Segments>, key: &DigitKey) -> Vec<u8> {
    digits.iter().map(|d| key.decode(d).expect(&format!("No such digit: {:?}", d))).collect()
}

fn sum_digits(digits: Vec<u8>) -> u32 {
    digits
        .iter().rev().enumerate()
        .map(
            |(i, &v)| u32::pow(10, i as u32)*v as u32
        )
        .sum::<u32>()
}

fn parse_words(s: &str) -> Vec<Segments> {
    s
        .split(char::is_whitespace)
        .map(str::trim)
        .map(|s| s.parse::<Segments>().expect("Unparsable word"))
        .collect()
}

fn parse_line(s: &str) -> Notes {
    let mut sections = s.split('|').map(str::trim);
    Notes {
        patterns: parse_words(sections.next().expect("Empty data line")),
        outputs: parse_words(sections.next().expect("No section separator")),
    }
}

fn parse_input(s: &str) -> Vec<Notes> {
    s.trim()
        .split('\n')
        .map(str::trim)
        .map(parse_line)
        .collect()
}

fn count_uniqe_segment_output_digits(notes: Vec<Notes>) -> usize {
    notes
        .into_iter()
        .flat_map(|n| n.outputs)
        .filter(|seg| seg.is_unique_segments_digit())
        .count()
}

fn sum_decoded_outputs(notes: &Vec<Notes>) -> usize {
    let mut total_sum: usize = 0;
    for n in notes.iter() {
        let key  = decode_patterns(&n.patterns);
        let digits = decode_digits(&n.outputs, &key);
        let digit_sum = sum_digits(digits);
        total_sum += digit_sum as usize;
    }
    total_sum
}

pub fn count_digits_1478() -> usize {
    let notes = parse_input(INPUT);
    count_uniqe_segment_output_digits(notes)
}

pub fn sum_all_decoded_outputs() -> usize {
    let notes = parse_input(INPUT);
    sum_decoded_outputs(&notes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_segment_word() {
        let seg: Segments = "fbgad".parse().unwrap();
        assert!(seg.0.contains(&Segment::A));
        assert!(seg.0.contains(&Segment::B));
        assert!(!seg.0.contains(&Segment::C));
        assert!(seg.0.contains(&Segment::D));
        assert!(!seg.0.contains(&Segment::E));
        assert!(seg.0.contains(&Segment::F));
        assert!(seg.0.contains(&Segment::G));
    }

    #[test]
    fn parse_notes_or_outputs() {
        let data = "abcd ef ab cde fg defg";
        let words = parse_words(data);
        assert_eq!(6, words.len());
        assert_eq!("abcd".parse::<Segments>().unwrap(), words[0]);
        assert_eq!("defg".parse::<Segments>().unwrap(), words[5]);
    }

    #[test]
    fn parse_data_line() {
        let data = "abcd ef ab cde | fg defg";
        let notes = parse_line(data);
        assert_eq!(4, notes.patterns.len());
        assert_eq!(2, notes.outputs.len());
    }

    const EXAMPLE: &str = include_str!("../data/8/example");

    #[test]
    fn example_count_1478() {
        let notes = parse_input(EXAMPLE);
        let count = count_uniqe_segment_output_digits(notes);
        assert_eq!(26, count);
    }

    ///  dddd
    /// e    a
    /// e    a
    ///  ffff
    /// g    b
    /// g    b
    ///  cccc
    impl From<u8> for Segments {
        fn from(n: u8) -> Segments {
            match n {
                0 => "abcdeg",
                1 => "ab",
                2 => "adfgc",
                3 => "abcdf",
                4 => "abef",
                5 => "bcdef",
                6 => "bcdefg",
                7 => "abd",
                8 => "abcdefg",
                9 => "abcdef",
                _ => panic!("Number cannot be represented in seven segments"),
            }.parse().unwrap()
        }
    }

    /// Five has 5 segments that all overlap with six
    #[test]
    fn deduce_five_pattern() {
        let five = Segments::from(5);
        let six = Segments::from(6);
        assert_eq!(5, five.overlap(&six));
    }

    /// Three has 5 segments and shares the 2 segments of one
    /// Two has 5 segments, deduce three and five first and you have two
    #[test]
    fn deduce_three_and_two_pattern() {
        let one = Segments::from(1);
        let three = Segments::from(3);
        assert_eq!(2, one.overlap(&three));
    }

    /// Nine has 6 segments and shares the 2 segments of one and is not zero
    /// Six has 6 segments, deduce nine and zero first and you have six
    #[test]
    fn deduce_nine_and_six_pattern() {
        let one = Segments::from(1);
        let nine = Segments::from(9);
        assert_eq!(2, nine.overlap(&one));
    }

    /// Zero has 6 segments that all overlap with 8
    #[test]
    fn deduce_zero_pattern() {
        let zero = Segments::from(0);
        let eight = Segments::from(8);
        assert_eq!(6, zero.overlap(&eight));
    }

    const EXAMPLE_LINE: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    #[test]
    fn sum_digits_to_number() {
        assert_eq!(0, sum_digits(vec![]));
        assert_eq!(123, sum_digits(vec![1,2,3]));
        assert_eq!(1234, sum_digits(vec![1,2,3,4]));
        assert_eq!(234, sum_digits(vec![2,3,4]));
    }


    #[test]
    fn test_encode_decode() {
        let mut key = DigitKey::new();
        for i in 0..=9 {
            let seg = Segments::from(i);
            key.update(i, seg.clone());
            assert_eq!(Some(seg.clone()), key.encode(i));
            assert_eq!(Some(i as u8), key.decode(&seg));
        }
    }

    #[test]
    fn decode_example_patterns() {
        let patterns = parse_line(EXAMPLE_LINE).patterns;
        let key  = decode_patterns(&patterns);
        let mut expected_key = DigitKey::new();
        for i in 0..=9 {
            expected_key.update(i, Segments::from(i));
        }
        assert_eq!(expected_key, key);
        assert_eq!(1, key.decode(&Segments::from(1)).unwrap());
    }

    #[test]
    fn decode_example_line() {
        let notes = parse_line(EXAMPLE_LINE);
        let key  = decode_patterns(&notes.patterns);
        let digits = decode_digits(&notes.outputs, &key);
        let digit_sum = sum_digits(digits);
        assert_eq!(5353, digit_sum);
    }

    #[test]
    fn decode_full_example() {
        let notes = parse_input(EXAMPLE);
        let total_sum = sum_decoded_outputs(&notes);
        assert_eq!(61229, total_sum);
    }

    #[test]
    fn decode_full_puzzle() {
        let notes = parse_input(INPUT);
        let total_sum = sum_decoded_outputs(&notes);
        assert_eq!(936117, total_sum);
    }
}
