use std::str::FromStr;
use std::ops::Add;
use std::collections::BTreeSet;
use std::iter::Extend;

const INPUT: &str = include_str!("../data/11/input");

#[derive(Debug, Clone, Copy, PartialOrd, Ord, Eq, PartialEq)]
struct Position(isize, isize); // x, y

impl Add for Position {
    type Output = Position;
    fn add(self: Self, other: Self) -> Self::Output {
        Position(self.0+other.0, self.1+other.1)
    }
}

#[derive(Debug)]
struct OctoMap{
    data: Vec<u8>,
    shape: (usize, usize)  // x, y
}

#[derive(Debug, PartialEq)]
struct ParseOctoMapError(&'static str);
impl FromStr for OctoMap {
    type Err = ParseOctoMapError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut x_dimensions = s
            .trim()
            .split('\n')
            .map(str::trim)
            .map(str::chars)
            .map(|l| l.count());

        let x_dimension = x_dimensions.next().unwrap(); // One element is guaranteed (might be "")
        if x_dimension  == 0 { return Err(ParseOctoMapError("Empty data")); }
        if !x_dimensions .all(|x| x == x_dimension) {
            return Err(ParseOctoMapError("Ragged data shape in x dimension"));
        }

        let data: Vec<u8> = s
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| u8::from_str(&c.to_string()).unwrap())
            .collect();

        let shape = (data.len()/x_dimension, x_dimension);
        Ok(OctoMap{data, shape})
    }
}

impl OctoMap {
    fn contains(self: &Self, pos: Position) -> bool {
        (0..self.shape.0 as isize).contains(&pos.0) &&
        (0..self.shape.1 as isize).contains(&pos.1)
    }

    #[allow(unused)]
    fn get(self: &Self, pos: Position) -> Option<u8> {
        if !self.contains(pos) {return None; }
        self.data.get(pos.1 as usize * self.shape.0 + pos.0 as usize).map(|v| *v)
    }

    // Adjancency offsets 
    const ADJACENTS: [Position; 8] = [
        Position(-1, -1), Position( 0, -1), Position( 1, -1),
        Position(-1,  0),                   Position( 1,  0),
        Position(-1,  1), Position( 0,  1), Position( 1,  1),
    ];

    fn get_adjacent_pos(self: &Self, pos: Position) -> BTreeSet<Position> {
        Self::ADJACENTS
            .into_iter()
            .map(|offset| offset+pos)
            .filter(|&a| self.contains(a))
            .collect()
    }

    fn idx_to_pos(self: &Self, idx: usize) -> Position {
        Position((idx % self.shape.0) as isize, (idx / self.shape.0) as isize)
    }

    fn pos_to_idx(self: &Self, pos: Position) -> usize {
        pos.0 as usize % self.shape.0  + pos.1 as usize *self.shape.0
    }

    fn get_flashers(self: &Self) -> BTreeSet<Position> {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, e)| **e>9)
            .map(|(i, _)| self.idx_to_pos(i))
            .collect()
    }

    fn apply<I, F>(self: &mut Self, it: I, f: F) 
        where I: IntoIterator<Item=Position>, F: Fn(u8) -> u8
    {
        for pos in it {
            let idx = self.pos_to_idx(pos);
            self.data[idx] = f(self.data[idx]);
        }
    }

    fn apply_all<F>(self: &mut Self, f: F)
        where F: Fn(u8) -> u8
    {
        self.data.iter_mut().for_each(|e| *e=f(*e));
    }

    fn step(self: &mut Self) -> u64 {
        self.apply_all(|e| e+1); // Add one to all energy levels
        let mut all_flashers = BTreeSet::<Position>::new();
        loop {
            let new_flashers = self.get_flashers();
            if new_flashers.is_empty() { break; } // No new flashers means step is done
            all_flashers.extend(new_flashers.iter());
            let flashed: Vec<Position> = 
                new_flashers
                    .iter()
                    .flat_map(|n| self.get_adjacent_pos(*n))
                    .filter(|a| !all_flashers.contains(a))
                    .collect();
            self.apply(flashed, |e| e+1); // Add one to all affected neighbours
            self.apply(new_flashers, |_| 0); // Reset all flashers to 0
        }
        all_flashers.len() as u64
    }

    fn get_first_sync_flash(self: &mut Self) -> u64 {
        let mut steps = 0;
        loop {
            steps += 1;
            if self.step() as usize == self.data.len() { break; }
        }
        steps
    }
}


pub fn octopus_flashes() -> u64 {
    let mut octo: OctoMap = INPUT.parse().unwrap();
    (0u8..100).map(|_| octo.step()).sum()
}

pub fn octopus_sync_flashes() -> u64 {
    let mut octo: OctoMap = INPUT.parse().unwrap();
    octo.get_first_sync_flash()
}


#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/11/example");

    #[test]
    fn parse_octo_map_example() {
        let octo: OctoMap = EXAMPLE.parse().unwrap();
        assert_eq!(100, octo.data.len());
        assert_eq!((10, 10), octo.shape);
    }

    #[test]
    fn index_octo_map() {
        let octo: OctoMap = EXAMPLE.parse().unwrap();

        assert_eq!(Some(5), octo.get(Position(0,0)));
        assert_eq!(Some(5), octo.get(Position(0,9)));
        assert_eq!(Some(3), octo.get(Position(9,0)));
        assert_eq!(Some(6), octo.get(Position(9,9)));
        assert_eq!(Some(3), octo.get(Position(4,4)));

        assert_eq!(None, octo.get(Position(-1,0)));
        assert_eq!(None, octo.get(Position(-1,9)));
        assert_eq!(None, octo.get(Position(10,0)));
        assert_eq!(None, octo.get(Position(10,9)));

        assert_eq!(None, octo.get(Position(0,-1)));
        assert_eq!(None, octo.get(Position(0,10)));
        assert_eq!(None, octo.get(Position(9,-1)));
        assert_eq!(None, octo.get(Position(9,10)));
    }


    #[test]
    fn pos_idx() {
        let octo: OctoMap = EXAMPLE.parse().unwrap();
        assert_eq!(Position(0, 0), octo.idx_to_pos(0));
        assert_eq!(0, octo.pos_to_idx(Position(0, 0)));
        assert_eq!(Position(1, 0), octo.idx_to_pos(1));
        assert_eq!(1, octo.pos_to_idx(Position(1, 0)));
        for i in 0..5 {
            assert_eq!(i, octo.pos_to_idx(octo.idx_to_pos(i)));
        }
    }

    #[test]
    fn adjacent_pos() {
        let octo: OctoMap = EXAMPLE.parse().unwrap();
        assert_eq!(3, octo.get_adjacent_pos(Position(0,0)).len());
        assert_eq!(5, octo.get_adjacent_pos(Position(1,0)).len());
        assert_eq!(8, octo.get_adjacent_pos(Position(1,1)).len());
    }

    #[test]
    fn step_increment() {
        let mut octo: OctoMap = "123\n456\n321".parse().unwrap();
        octo.step();
        assert_eq!(2, octo.get(Position(0,0)).unwrap());
        assert_eq!(6, octo.get(Position(1,1)).unwrap());
    }

    #[test]
    fn find_flashers() {
        let mut octo: OctoMap = "123\n456\n321".parse().unwrap();
        for _ in 0..3 {octo.step();}
        assert_eq!(2, octo.step());
    }

    #[test]
    fn count_flahes_example() {
        let mut octo: OctoMap = EXAMPLE.parse().unwrap();
        let total_flashes: u64 = (0u8..100).map(|_| octo.step()).sum();
        assert_eq!(1656, total_flashes);
    }

    #[test]
    fn count_flahes_solution() {
        let mut octo: OctoMap = INPUT.parse().unwrap();
        let total_flashes: u64 = (0u8..100).map(|_| octo.step()).sum();
        assert_eq!(1700, total_flashes);
    }

    #[test]
    fn sync_flash_example() {
        let mut octo: OctoMap = EXAMPLE.parse().unwrap();
        assert_eq!(195, octo.get_first_sync_flash());
    }
}

