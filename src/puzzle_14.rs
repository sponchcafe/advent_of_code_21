use std::str::FromStr;
use std::collections::HashMap;
use std::ops::Index;

const POLYMER: &str = include_str!("../data/14/polymer");
const RULES: &str = include_str!("../data/14/rules");

type Pair = (char, char);

#[derive(Debug, PartialEq)]
enum Applied { Pair(char, char), Triple(char, char, char)}

#[derive(Debug, PartialEq)]
struct Rule {
    pair: Pair,
    insert: char
}

#[derive(Debug)]
struct Rules ( HashMap<Pair, char>);

impl Rule {
    fn new(pair: (char, char), insert: char) -> Self {
        Rule{pair, insert}
    }
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Rule, Self::Err> {
        let mut parts = s
            .trim()
            .split("->")
            .map(str::trim);

        match (parts.next(), parts.next(), parts.next()) {
            (Some(pair), Some(insert), None) => {
                let mut p = pair.chars();
                let pair = match (p.next(), p.next(), p.next()) {
                    (Some(a), Some(b), None) =>  { Ok( (a, b) ) }
                    _ => { Err(()) }
                };
                let mut i = insert.chars();
                let insert = match (i.next(), i.next()) {
                    (Some(i),  None) =>  { Ok( i ) }
                    _ => { Err(()) }
                };
                Ok(Rule::new(pair?, insert?))
            },
            _ => { Err(()) } 
        }
    }
}

impl FromIterator<Rule> for Rules {
    fn from_iter<T: IntoIterator<Item=Rule>>(iter: T) -> Rules {
        Rules(
            iter.into_iter()
                .map(|rule| (rule.pair, rule.insert))
                .collect::<HashMap<Pair, char>>()
        )
    }
}

fn parse_rules(s: &str) -> Rules {
    s
        .trim()
        .lines()
        .map(str::trim)
        .map(Rule::from_str)
        .map(Result::unwrap)
        .collect()
}

impl Rules {
    fn apply(self: &Self, pair: Pair) -> Applied {
        match self.0.get(&pair) {
            Some(insert) => { Applied::Triple(pair.0, *insert, pair.1) }
            None => { Applied::Pair(pair.0, pair.1) }
        }
    }
}

fn push_applied(s: &mut String, app: Applied) {
    match app {
        Applied::Pair(a, b) if s.len() == 0 => { s.push(a); s.push(b); }
        Applied::Triple(a, b, c) if s.len() == 0 => { s.push(a); s.push(b); s.push(c); }
        Applied::Pair(_, b) => { 
            s.push(b); 
        }
        Applied::Triple(_, b, c) => { 
            s.push(b); s.push(c); 
        }
    }
}

struct Pairs<'a> ( std::iter::Peekable<std::str::Chars<'a>> );

impl<'a> Iterator for Pairs<'a> {
    type Item = Pair;
    fn next(self: &mut Self) -> Option<Self::Item> {
        match (self.0.next(), self.0.peek()) {
            (Some(a), Some(&b)) => { Some((a, b)) },
            _ => None
        }
    }
}

fn pairs<'a> (s: &'a str) -> Pairs<'a> {
    Pairs(s.chars().peekable())
}

fn polymerize(s: String, r: &Rules) -> String {
    let mut polymerized = String::with_capacity(s.len()*2);
    pairs(&s)
        .map(|pair| r.apply(pair))
        .for_each(|app| push_applied(&mut polymerized, app));
    polymerized
}

#[derive(Debug)]
struct ElementCounts(HashMap<char, usize>);

impl Index<char> for ElementCounts {
    type Output = usize;
    fn index(self: &Self, c: char) -> &Self::Output {
        match self.0.get(&c) {
            Some(count) => &count,
            None => &0
        }
    }
}

fn count_elements(s: &str) -> ElementCounts {
    let mut map = HashMap::<char, usize>::new();
    for c in s.chars() {
        match map.get_mut(&c) {
            Some(count) => { *count += 1; }
            None => { map.insert(c, 1); }
        }
    }
    ElementCounts(map)
}

impl ElementCounts{
    fn max(self: &Self) -> usize {
        *self.0.iter().map(|(_, v)| v).max().unwrap_or(&0)
    }
    fn min(self: &Self) -> usize {
        *self.0.iter().map(|(_, v)| v).min().unwrap_or(&0)
    }
}

pub fn polymer_index() -> u64 {
    let mut s = String::from(POLYMER.trim());
    let r = parse_rules(RULES);
    for _ in 0..10 { s = polymerize(s, &r); }
    let counts = count_elements(&s);
    (counts.max()-counts.min()) as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_rules() {
        assert_eq!(Rule::new(('A', 'B'), 'C'), "AB -> C".parse::<Rule>().unwrap());
        assert!("".parse::<Rule>().is_err());
        assert!(" -> ".parse::<Rule>().is_err());
        assert!("AB -> ".parse::<Rule>().is_err());
        assert!("ABC -> D".parse::<Rule>().is_err());
        assert!("AB -> CD".parse::<Rule>().is_err());
        assert!("AB -> C -> D".parse::<Rule>().is_err());
    }
    
    #[test]
    fn apply_rules() {
        let rules: Rules = [Rule::new(('A', 'B'), 'C')].into_iter().collect();
        assert_eq!(Applied::Triple('A', 'C', 'B'), rules.apply(('A', 'B')));
        assert_eq!(Applied::Pair('A', 'C'), rules.apply(('A', 'C')));
    }
    
    #[test]
    fn push_applied_to_string() {
        let mut s = String::with_capacity(10);
        push_applied(&mut s, Applied::Triple('A', 'B', 'C'));
        push_applied(&mut s, Applied::Pair('C', 'D'));
        assert_eq!("ABCD".to_string(), s);
    }

    #[test]
    fn generate_pairs() {
        let s = "ABCDE";
        let pairs: Vec<Pair> = pairs(s).collect();
        assert_eq!(4, pairs.len());
        assert_eq!(vec![('A','B'),('B','C'),('C','D'),('D','E')], pairs);
    }

    #[test]
    fn test_polymerize() {
        let s = String::from("ABC");
        let rules: Rules = [Rule::new(('A', 'B'), 'C')].into_iter().collect();
        let s = polymerize(s, &rules);
        assert_eq!(String::from("ACBC"), s);
    }

    #[test]
    fn test_count_elements() {
        let s = String::from("ABCDEFABC");
        let counts = count_elements(&s);
        assert_eq!(counts['A'], 2);
        assert_eq!(counts['B'], 2);
        assert_eq!(counts['C'], 2);
        assert_eq!(counts['D'], 1);
        assert_eq!(counts['E'], 1);
        assert_eq!(counts['F'], 1);
        assert_eq!(counts['G'], 0);
    }

    const EX_RULES: &str = include_str!("../data/14/example_rules");

    #[test]
    fn test_example() {
        let mut s = String::from("NNCB");
        let r = parse_rules(EX_RULES);
        for _ in 0..10 { s = polymerize(s, &r); }
        let counts = count_elements(&s);
        assert_eq!(1588, counts.max()-counts.min());
    }

}
