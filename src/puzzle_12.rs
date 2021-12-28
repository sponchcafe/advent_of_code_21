use std::str::FromStr;
use std::collections::HashSet;

const INPUT: &str = include_str!("../data/12/input");

type ID = String;
type Set<T> = HashSet<T>;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
enum Cave {
    Start,
    End,
    Small(ID),
    Large(ID),
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Tunnel { 
    from: Cave,
    to: Cave
}

#[derive(Debug)]
struct CaveMap {
    tunnels: Vec<Tunnel>,
}

#[derive(Debug, Clone)]
struct Path ( Vec<Cave> );

impl FromStr for Cave{
    type Err = ();
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        if id.is_empty() { Err(()) }
        else if id == "start" { Ok(Cave::Start) }
        else if id == "end" { Ok(Cave::End) }
        else if id.chars().all(|c| c.is_uppercase()) { Ok(Cave::Large(id.into())) }
        else if id.chars().all(|c| c.is_lowercase()) { Ok(Cave::Small(id.into())) }
        else { Err(()) }
    }
}

impl FromStr for Tunnel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut caves = s
            .trim()
            .split('-')
            .map(str::trim)
            .map(Cave::from_str);
        let from = caves.next().ok_or(())??;
        let to   = caves.next().ok_or(())??;
        if caves.next().is_some() { return Err(()) } // More than 2 caves
        Ok(Tunnel{from, to})
    }
}

impl FromIterator<Tunnel> for CaveMap {
    fn from_iter<T>(it: T) -> Self 
        where T: IntoIterator<Item=Tunnel>
    {
        let tunnels: Vec<Tunnel> = it.into_iter().collect();
        CaveMap{tunnels}
    }
}

impl CaveMap {

    fn caves<'a, T: Iterator<Item=&'a Tunnel>>(tunnels: T) -> Set<Cave> {
        let mut caves = Set::<Cave>::new();
        tunnels.for_each(|t| {
            caves.insert(t.from.clone());
            caves.insert(t.to.clone());
        });
        caves
    }

    fn connected(self: &Self, here: &Cave) -> Set<Cave> {
        let mut caves = Self::caves(
            self.tunnels
            .iter()
            .filter(|c| c.to == *here || c.from == *here)
        );
        caves.remove(here);
        caves
    }

    fn count_paths(self: &Self) -> u64 {
        let mut paths = vec![Path(vec![Cave::Start])];
        let mut count = 0u64;
        loop {
            paths = paths
                .into_iter()
                .filter(|p| !p.complete())
                .flat_map(|p| {
                    let connected = self.connected(p.tip());
                    let n = extend(p, connected);
                    n
                })
                .filter(|p| p.valid())
                .collect();
            if paths.is_empty() { break; }
            count += paths.iter().filter(|p| p.complete()).count() as u64;
        }
        count
    }
}

impl Path {
    fn complete(self: &Self) -> bool {
        self.0.first() == Some(&Cave::Start) && 
        self.0.last() == Some(&Cave::End)
    }

    fn valid(self: &Self) -> bool {
        !self._double_visit_small() &&
        !self._double_stay() &&
        !self._multi_start()
    }

    fn _double_visit_small(self: &Self) -> bool {
        let small_caves: Vec<String> = self.0.iter().filter_map(|c| match c {
            Cave::Small(id) => Some(id.clone()),
            _ => None
        }).collect();
        small_caves.len() != small_caves.into_iter().collect::<Set<_>>().len()
    }

    fn _double_stay(self: &Self) -> bool {
        let it1 = self.0.iter();
        let mut it2 = it1.clone();
        it2.next();
        it1.zip(it2).any(|(i1, i2)| i1 == i2)
    }

    fn _multi_start(self: &Self) -> bool {
        self.0.iter().filter(|c| **c == Cave::Start).count() > 1
    }

    fn tip(self: &Self) -> &Cave {
        self.0.last().expect("non empty path")
    }
}

fn extend(p: Path, c: Set<Cave>) -> Vec<Path> {
    c.iter()
        .map(|c| {
            let mut new = p.clone();
            new.0.push(c.clone()); 
            new
        }
        ).collect()
}


fn parse_input(s: &str) -> CaveMap {
    s
        .trim()
        .split('\n')
        .map(str::trim)
        .map(|t| t.parse::<Tunnel>().expect("Valid tunnel spec"))
        .collect()
}

pub fn number_of_paths() -> u64 {
    parse_input(INPUT).count_paths()
}


#[cfg(test)]
mod test {
    
    use super::*;

    const EX1: &str = include_str!("../data/12/example1");
    const EX2: &str = include_str!("../data/12/example2");
    const EX3: &str = include_str!("../data/12/example3");

    #[test]
    fn parse_cave() {
        assert_eq!(Cave::Start, "start".parse::<Cave>().unwrap());
        assert_eq!(Cave::End, "end".parse::<Cave>().unwrap());
        assert_eq!(Cave::Small("a".into()), "a".parse::<Cave>().unwrap());
        assert_eq!(Cave::Large("A".into()), "A".parse::<Cave>().unwrap());
    }

    #[test]
    fn parse_path() {
        let path: Tunnel = "a-B".parse().unwrap();
        assert_eq!(Cave::Small("a".into()), path.from);
        assert_eq!(Cave::Large("B".into()), path.to);
    }

    #[test]
    fn parse_path_error() {
        assert!("".parse::<Tunnel>().is_err());
        assert!("a".parse::<Tunnel>().is_err());
        assert!("a-".parse::<Tunnel>().is_err());
        assert!("a--".parse::<Tunnel>().is_err());
        assert!("a--".parse::<Tunnel>().is_err());
        assert!("a-b-c".parse::<Tunnel>().is_err());
    }

    #[test]
    fn cave_map_from_iter() {
        let mut paths = Vec::<Tunnel>::new();
        paths.push(Tunnel{
            from: Cave::Small("a".into()),
            to: Cave::Small("b".into()),
        });
        let map: CaveMap = paths.into_iter().collect();
        assert_eq!(2, CaveMap::caves(map.tunnels.iter()).iter().count());
    }

    #[test]
    fn connected_caves() {
        let map = parse_input(EX1);
        assert_eq!(2, map.connected(&Cave::Start).len());
        assert_eq!(4, map.connected(&Cave::Large("A".into())).len());
        assert_eq!(1, map.connected(&Cave::Small("d".into())).len());
    }

    #[test]
    fn path_is_complete() {
        assert_eq!(false, Path(vec![]).complete());
        assert_eq!(false, Path(vec![Cave::Start]).complete());
        assert_eq!(true, Path(vec![Cave::Start, Cave::End]).complete());
        assert_eq!(true, Path(vec![Cave::Start, Cave::Small("a".into()), Cave::End]).complete());
    }

    #[test]
    fn path_is_valid() {
        assert_eq!(true, Path(vec![]).valid());
        assert_eq!(true, Path(vec![Cave::Start]).valid());
        assert_eq!(true, Path(vec![Cave::Start, Cave::End]).valid());
        assert_eq!(true, Path(vec![Cave::Start, Cave::Small("a".into()), Cave::End]).valid());
        assert_eq!(true, Path(vec![
                Cave::Start,
                Cave::Large("A".into()),
                Cave::Small("b".into()),
                Cave::Large("A".into()),
                Cave::End,
        ]).valid());
    }

    #[test]
    fn path_invalid_mutliple_visits_small() {
        assert_eq!(false, Path(vec![
                Cave::Start,
                Cave::Small("a".into()),
                Cave::Small("b".into()),
                Cave::Small("a".into()),
        ]).valid());
    }

    #[test]
    fn path_invalid_stay_in_one_cave() {
        assert_eq!(false, Path(vec![
                Cave::Start,
                Cave::Large("A".into()),
                Cave::Large("A".into()),
                Cave::End,
        ]).valid());
    }

    #[test]
    fn path_invalid_multi_start() {
        assert_eq!(false, Path(vec![
                Cave::Start,
                Cave::Small("a".into()),
                Cave::Start,
                Cave::End,
        ]).valid());
    }

    #[test]
    fn extend_path() {
        let path = Path(vec![Cave::Start]);
        let connected = [Cave::Small("a".into()), Cave::Large("B".into()), Cave::End].into_iter().collect::<Set<_>>();
        let paths = extend(path, connected);
        assert_eq!(3, paths.len());
        assert_eq!(2, paths[0].0.len());
    }

    #[test]
    fn example_1() {
        assert_eq!(10, parse_input(EX1).count_paths());
    }

    #[test]
    fn example_2() {
        assert_eq!(19, parse_input(EX2).count_paths());
    }

    #[test]
    fn example_3() {
        assert_eq!(226, parse_input(EX3).count_paths());
    }

    #[test]
    fn solution_1() {
        assert_eq!(4495, parse_input(INPUT).count_paths());
    }

}
