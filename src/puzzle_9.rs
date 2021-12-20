use std::str::FromStr;

const INPUT: &str = include_str!("../data/9/input");

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
    fn get(self: &Self, pos: (i32, i32)) -> Option<u8> {
        if pos.0 < 0 || pos.1 < 0 { return None }
        self.data.get(pos.0 as usize *self.shape.1 + pos.1 as usize).map(|v| *v)
    }
    fn get_adjacent(self: &Self, pos: (i32, i32)) -> Vec<u8> {
        let mut adj = vec![];
        adj.push(self.get((pos.0-1, pos.1)));
        adj.push(self.get((pos.0, pos.1-1)));
        adj.push(self.get((pos.0, pos.1+1)));
        adj.push(self.get((pos.0+1, pos.1)));
        adj.into_iter().filter_map(|a| a).collect()
    }
    fn pos_to_idx(self: &Self, idx: usize) -> (i32, i32) {
        ((idx / self.shape.1) as i32, (idx % self.shape.1) as i32)
    }
    fn get_risk_levels(self: &Self) -> Vec<u8> {
        let mut depths = vec![];
        for (i, val) in self.data.iter().enumerate() {
            let idx = self.pos_to_idx(i);
            let adj = self.get_adjacent(idx);
            if adj.iter().all(|a| a>val) {
                depths.push(*val+1);
            }
        }
        depths
    }
}

pub fn sum_risk_levels() -> usize {
    let heightmap: Heightmap = INPUT.parse().unwrap();
    heightmap.get_risk_levels().into_iter().map(|v| v as usize).sum()
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
        assert_eq!(Some(8), heightmap.get((4,9)));
        assert_eq!(Some(7), heightmap.get((2,4)));

        assert_eq!(None, heightmap.get((-1,0)));
        assert_eq!(None, heightmap.get((-1,-2)));
        assert_eq!(None, heightmap.get((100,100)));
    }

    #[test]
    fn get_adjacent_fields() {
        let heightmap: Heightmap = EXAMPLE_HEIGHTMAP.parse().unwrap();
        let corner = heightmap.get_adjacent((0,0));
        let edge = heightmap.get_adjacent((1,0));
        let field = heightmap.get_adjacent((1,1));
        assert_eq!(2, corner.len());
        assert_eq!(3, edge.len());
        assert_eq!(4, field.len());
        assert_eq!(vec![1, 3], corner);
        assert_eq!(vec![2, 9, 9], edge);
        assert_eq!(vec![1, 3, 8, 8], field);
    }

    #[test]
    fn example_get_depths() {
        let heightmap: Heightmap = EXAMPLE_HEIGHTMAP.parse().unwrap();
        let depths = heightmap.get_risk_levels();
        assert_eq!(vec![2, 1, 6, 6], depths);
    }
}
