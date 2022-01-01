use std::str::FromStr;
use std::collections::HashMap;
use std::ops::Index;

const POLYMER: &str = include_str!("../data/14/polymer");
const RULES: &str = include_str!("../data/14/rules");

type Pair = (char, char);

#[derive(Debug, PartialEq)]
struct Rule {
    input: Pair,
    output: (Pair, Pair),
}

struct Rules ( HashMap<Pair, (Pair, Pair)> );

#[derive(Debug, Default)]
struct Polymer{ 
    initial: String,
    pair_counts: HashMap<Pair, usize>,
}

impl Rule {
    fn new(input: (char, char), insert: char) -> Self {
        Rule{
            input,
            output: ((input.0, insert), (insert, input.1))
        }
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
                let input = match (p.next(), p.next(), p.next()) {
                    (Some(a), Some(b), None) =>  { Ok( (a, b) ) }
                    _ => { Err(()) }
                };
                let mut i = insert.chars();
                let insert = match (i.next(), i.next()) {
                    (Some(i),  None) =>  { Ok( i ) }
                    _ => { Err(()) }
                };
                Ok(Rule::new(input?, insert?))
            },
            _ => { Err(()) } 
        }
    }
}

impl FromIterator<Rule> for Rules {
    fn from_iter<T: IntoIterator<Item=Rule>>(iter: T) -> Rules {
        Rules(
            iter.into_iter()
                .map(|rule| (rule.input, rule.output)) 
                .collect::<HashMap<Pair, (Pair, Pair)>>()
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

impl Polymer {

    fn new(s: &str) -> Self {
        let mut pair_counts = HashMap::<Pair, usize>::new();
        pairs(s)
            .for_each(|p|
                match pair_counts.get_mut(&p) {
                    Some(c) => { *c += 1; },
                    None => { pair_counts.insert(p, 1); }
                }
            );
        Self{pair_counts, initial: s.into()}
    }

    fn update(self: &mut Self, k: Pair, val: isize) {
        let mut new_val = match self.pair_counts.get_mut(&k) {
            Some(c) => { *c as isize + val }
            None => { val }
        };
        if new_val < 0 { new_val = 0; }
        self.pair_counts.insert(k, new_val as usize);
    }

    fn polymerize(self: &mut Self, rules: &Rules, steps: usize) {
        for _ in 0..steps {
            let mut next = Polymer::default();
            next.initial = self.initial.clone();
            for (p0, v) in self.pair_counts.iter() {
               match rules.0.get(p0) {
                   Some((p1, p2)) => {
                       next.update(*p1, *v as isize);
                       next.update(*p2, *v as isize);
                   },
                   None => {
                       next.update(*p0, *v as isize);
                   }
               }
            }
            *self = next;
        }
    }

    fn start(self: &Self) -> char {
        self.initial.chars().next().expect("Non empty initial polymer")
    }

    fn end(self: &Self) -> char {
        self.initial.chars().last().expect("Non empty initial polymer")
    }

    fn counts(self: &Self) -> ElementCounts {
        let mut counts = ElementCounts::new();
        for (k, v) in self.pair_counts.iter() {
            counts.update(k.0, *v as isize);
            counts.update(k.1, *v as isize);
        }
        for v in counts.0.values_mut() { *v /= 2; }
        counts.update(self.start(), 1);
        counts.update(self.end(), 1);
        counts
    }
}

impl<T> From<T> for Polymer 
    where T: AsRef<str>
{
    fn from(s: T) -> Self {
        Polymer::new(s.as_ref())
    }
}

fn pairs<'a> (s: &'a str) -> Pairs<'a> {
    Pairs(s.chars().peekable())
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

impl ElementCounts{
    fn new() -> Self {
        Self(HashMap::<char, usize>::new())
    }
    fn update(self: &mut Self, k: char, val: isize) {
        let mut new_val = match self.0.get_mut(&k) {
            Some(c) => { *c as isize + val }
            None => { val }
        };
        if new_val < 0 { new_val = 0; }
        self.0.insert(k, new_val as usize);
    }
    fn max(self: &Self) -> usize {
        *self.0.iter().map(|(_, v)| v).max().unwrap_or(&0)
    }
    fn min(self: &Self) -> usize {
        *self.0.iter().map(|(_, v)| v).min().unwrap_or(&0)
    }
}

pub fn polymer_index(steps: usize) -> u64 {
    let mut polymer = Polymer::from(POLYMER.trim());
    let rules = parse_rules(RULES);
    polymer.polymerize(&rules, steps);
    let counts = polymer.counts();
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
    fn generate_pairs() {
        let s = "ABCDE";
        let pairs: Vec<Pair> = pairs(s).collect();
        assert_eq!(4, pairs.len());
        assert_eq!(vec![('A','B'),('B','C'),('C','D'),('D','E')], pairs);
    }

    fn test_rules() -> Rules {
        [Rule::new(('A', 'B'), 'C')].into_iter().collect()
    }

    #[test]
    fn test_count_elements() {
        let polymer = Polymer::from("ABC");
        let counts = polymer.counts();
        assert_eq!(1, counts['A']);
        assert_eq!(1, counts['B']);
        assert_eq!(1, counts['C']);
        assert_eq!(0, counts['D']);
    }

    #[test]
    fn test_polymerize_no_step() {
        let mut polymer = Polymer::from("ABC");
        let rules = test_rules();
        polymer.polymerize(&rules, 0);
        let counts = polymer.counts();
        assert_eq!(1, counts['A']);
        assert_eq!(1, counts['B']);
        assert_eq!(1, counts['C']);
        assert_eq!(0, counts['D']);
    }

    #[test]
    fn test_polymerize() {
        let mut polymer = Polymer::from("ABC");
        let rules = test_rules();
        polymer.polymerize(&rules, 1);
        let counts = polymer.counts();
        assert_eq!(1, counts['A']);
        assert_eq!(1, counts['B']);
        assert_eq!(2, counts['C']);
        assert_eq!(0, counts['D']);
    }

    const EX_RULES: &str = include_str!("../data/14/example_rules");

    #[test]
    fn test_example() {
        let mut polymer = Polymer::from("NNCB");
        let rules = parse_rules(EX_RULES);
        polymer.polymerize(&rules, 10);
        let counts = polymer.counts();
        assert_eq!(1588, counts.max()-counts.min());
    }

}
