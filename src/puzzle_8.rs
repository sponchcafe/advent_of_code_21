use std::str::FromStr;
use std::collections::HashSet;

const INPUT: &str = include_str!("../data/8/input");

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Segment { A,B,C,D,E,F,G }

#[derive(Debug, PartialEq, Eq)]
struct Segments (
    HashSet::<Segment>,
);

#[derive(Debug, PartialEq, Eq)]
struct Notes {
    patterns: Vec<Segments>,
    outputs: Vec<Segments>,
}

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

    fn is_one(self: &Self) -> bool {
        self.0.len() == 2
    }

    fn is_four(self: &Self) -> bool {
        self.0.len() == 4
    }

    fn is_seven(self: &Self) -> bool {
        self.0.len() == 3
    }

    fn is_eight(self: &Self) -> bool {
        self.0.len() == 7
    }

    fn is_unique_segments_digit(self: &Self) -> bool {
        self.is_one() || self.is_four() || self.is_seven() || self.is_eight()
    }
}

impl FromIterator<Segment> for Segments {
    fn from_iter<T> (iter: T) -> Self 
        where T: IntoIterator<Item=Segment>
    {
        Segments(iter.into_iter().collect::<HashSet<Segment>>())
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

pub fn count_digits_1478() -> usize {
    let notes = parse_input(INPUT);
    count_uniqe_segment_output_digits(notes)
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

}
