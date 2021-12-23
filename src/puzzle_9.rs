use std::str::FromStr;
use std::collections::BTreeSet;

const INPUT: &str = include_str!("../data/9/input");

type Position = (isize, isize);

#[derive(Debug)]
struct Heightmap{
    data: Vec<u8>,
    shape: (usize, usize) 
}

#[derive(Debug, PartialEq)]
struct ParseHeightmapError(&'static str);
impl FromStr for Heightmap {
    type Err = ParseHeightmapError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut widths = s
            .trim()
            .split('\n')
            .map(str::trim)
            .map(str::chars)
            .map(|l| l.count());

        let width = widths.next().unwrap(); // One element is guaranteed (might be "")
        if width == 0 {
            return Err(ParseHeightmapError("Empty data"));
        }
        if !widths.all(|w| w == width) {
            return Err(ParseHeightmapError("Ragged data shape"));
        }

        let data: Vec<u8> = s
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| u8::from_str(&c.to_string()).unwrap())
            .collect();

        let shape = (data.len()/width, width);
        Ok(Heightmap{data, shape})
    }
}

impl Heightmap {
    fn contains(self: &Self, pos: Position) -> bool {
        (0..self.shape.0 as isize).contains(&pos.0) &&
        (0..self.shape.1 as isize).contains(&pos.1)
    }

    fn get(self: &Self, pos: Position) -> Option<u8> {
        if !self.contains(pos) {return None; }
        self.data.get(pos.0 as usize * self.shape.1 + pos.1 as usize).map(|v| *v)
    }

    fn get_adjacent_pos(self: &Self, pos: Position) -> Vec<Position> {
        let adj = vec![
            (pos.0-1, pos.1),
            (pos.0, pos.1-1),
            (pos.0, pos.1+1),
            (pos.0+1, pos.1),
        ];
        adj.into_iter().filter(|&a| self.contains(a)).collect()
    }

    fn pos_to_idx(self: &Self, idx: usize) -> Position {
        ((idx / self.shape.1) as isize, (idx % self.shape.1) as isize)
    }

    fn get_depth_pos(self: &Self) -> Vec<Position> {
        let mut depths = vec![];
        for (i, val) in self.data.iter().enumerate() {
            let pos = self.pos_to_idx(i);
            let adj = self.get_adjacent_pos(pos);
            if adj.iter().all(|a| self.get(*a).unwrap()>*val) {
                depths.push(pos);
            }
        }
        depths
    }

    fn get_risk_levels(self: &Self) -> Vec<u8> {
        self.get_depth_pos().into_iter().map(
            |pos| self.get(pos).expect("Depth pos not in this map")+1
        ).collect()
    }

    fn fill_basin(self: &Self, pos: Position) -> usize {
        let mut basin = BTreeSet::<Position>::new();
        let mut todo = BTreeSet::<Position>::new();
        todo.insert(pos);
        while !todo.is_empty() {
            let mut next_todo = BTreeSet::<Position>::new();
            for p in todo.into_iter() {
                basin.insert(p);
                next_todo.extend(
                    self.get_adjacent_pos(p).into_iter()
                    .filter(|a| !basin.contains(a))
                    .filter(|a| self.contains(*a))
                    .filter(|a| self.get(*a).expect("Position is not in this map")<9)
                );
            }
            todo = next_todo;
        }
        basin.len()
    }

    fn get_basin_sizes(self: &Self) -> Vec::<usize> { 
        self.get_depth_pos()
            .into_iter()
            .map(|d| self.fill_basin(d))
            .collect()
    }

    fn get_basin_score(self: &Self) -> usize {
        let mut basin_sizes = self.get_basin_sizes();
        basin_sizes.sort();
        basin_sizes.into_iter().rev().take(3).reduce(|total, s| total*s).expect("Not enough basins found")
    }
}

pub fn sum_risk_levels() -> usize {
    let heightmap: Heightmap = INPUT.parse().unwrap();
    heightmap.get_risk_levels().into_iter().map(|v| v as usize).sum()
}

pub fn basin_risk_level() -> usize {
    let heightmap: Heightmap = INPUT.parse().unwrap();
    heightmap.get_basin_score()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_HEIGHTMAP: &str = "
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    ";

    #[test]
    fn parse_example_heightmap() {
        let heightmap: Heightmap = EXAMPLE_HEIGHTMAP.parse().unwrap();
        assert_eq!(50, heightmap.data.len());
        assert_eq!((5, 10), heightmap.shape);
    }

    #[test]
    fn parse_error_heightmap() {
        match "".parse::<Heightmap>() {
            Err(ParseHeightmapError(msg)) => assert_eq!(msg, "Empty data"),
            Ok(_) => panic!("Empty string should not parse")
        }
        match "1\n12".parse::<Heightmap>() {
            Err(ParseHeightmapError(msg)) => assert_eq!(msg, "Ragged data shape"),
            Ok(_) => panic!("Ragged data should not parse")
        }
    }

    #[test]
    fn index_heightmap() {
        let heightmap: Heightmap = EXAMPLE_HEIGHTMAP.parse().unwrap();

        assert_eq!(Some(2), heightmap.get((0,0)));
        assert_eq!(Some(9), heightmap.get((4,0)));
        assert_eq!(Some(0), heightmap.get((0,9)));
        assert_eq!(Some(8), heightmap.get((4,9)));
        assert_eq!(Some(7), heightmap.get((2,4)));

        assert_eq!(None, heightmap.get((-1,0)));
        assert_eq!(None, heightmap.get((-1,9)));
        assert_eq!(None, heightmap.get((5,0)));
        assert_eq!(None, heightmap.get((5,9)));

        assert_eq!(None, heightmap.get((0,-1)));
        assert_eq!(None, heightmap.get((0,10)));
        assert_eq!(None, heightmap.get((4,-1)));
        assert_eq!(None, heightmap.get((4,10)));
    }

    #[test]
    fn get_adjacent_fields() {
        let heightmap: Heightmap = EXAMPLE_HEIGHTMAP.parse().unwrap();
        let corner = heightmap.get_adjacent_pos((0,0));
        let edge = heightmap.get_adjacent_pos((1,0));
        let field = heightmap.get_adjacent_pos((1,1));
        assert_eq!(2, corner.len());
        assert_eq!(3, edge.len());
        assert_eq!(4, field.len());
        assert_eq!(vec![(0,1), (1,0)], corner);
        assert_eq!(vec![(0,0), (1,1), (2,0)], edge);
        assert_eq!(vec![(0,1), (1,0), (1,2), (2, 1)], field);
    }

    #[test]
    fn example_get_depths() {
        let heightmap: Heightmap = EXAMPLE_HEIGHTMAP.parse().unwrap();
        let depths = heightmap.get_risk_levels();
        assert_eq!(vec![2, 1, 6, 6], depths);
    }

    #[test]
    fn fill_whole_map_basin() {
        let heightmap: Heightmap = "
            555
            515
            555
        ".parse().unwrap();

        let basin_size = heightmap.fill_basin((1,1));
        assert_eq!(9, basin_size);
    }

    #[test]
    fn fill_bounded_basin() {
        let heightmap: Heightmap = "
            5599
            5159
            5599
            9990
        ".parse().unwrap();

        let basin_size = heightmap.fill_basin((1,1));
        assert_eq!(7, basin_size);
    }

    #[test]
    fn example_basin_score() {
        let heightmap: Heightmap = EXAMPLE_HEIGHTMAP.parse().unwrap();
        assert_eq!(vec![3, 9, 14, 9], heightmap.get_basin_sizes());
        assert_eq!(1134, heightmap.get_basin_score());
    }

    #[test]
    fn solution_basin_score() {
        let heightmap: Heightmap = INPUT.parse().unwrap();
        assert_eq!(920448, heightmap.get_basin_score());
    }

}

