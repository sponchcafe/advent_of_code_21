use std::str::FromStr;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::boxed::Box;

const INPUT: &str = include_str!("../data/5/input");

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Point(u64, u64);

#[derive(Debug, PartialEq, Eq)]
struct Line (Point, Point);

#[derive(Debug, PartialEq)]
struct ParsePointError {}
impl FromStr for Point {
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec::<u64> = s
            .split(",")
            .map(str::trim)
            .filter_map(|p| p.parse::<u64>().ok())
            .collect();
        if let [x, y] = coords[..]{
            Ok(Point(x, y))
        }
        else {
            Err(ParsePointError{})
        }
    }
}

#[derive(Debug)]
struct ParseLineError {}
impl FromStr for Line {
    type Err = ParseLineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points : Vec<Point> = s
            .split("->")
            .map(str::trim)
            .filter_map(|p| p.parse::<Point>().ok())
            .collect();
        if let [from, to] = points[..]{
            Ok(Line(from, to))
        }
        else {
            Err(ParseLineError{})
        }
    }
}

impl Line {
    fn fill(self: &Self) -> Vec<Point> {
        if self.is_horizontal() {
            (self.start_x()..=self.end_x()).map(|i| Point(i, self.0.1)).collect()
        }
        else if self.is_vertical() {
            (self.start_y()..=self.end_y()).map(|i| Point(self.0.0, i)).collect()
        }
        else if self.is_diagonal() {
            let it_x = self.x_range();
            let it_y = self.y_range();
            it_x.into_iter().zip(it_y.into_iter()).map(|(x, y)| Point(x, y)).collect()
        }
        else {
            panic!("Non fillable line");
        }
    }

    fn x_range(self: &Self) -> Box<dyn Iterator<Item=u64>> {
        let it_x = self.start_x()..=self.end_x();
        if self.x_descending() {
            Box::new(it_x.into_iter().rev())
        }
        else {
            Box::new(it_x.into_iter())
        }
    }

    fn y_range(self: &Self) -> Box<dyn Iterator<Item=u64>> {
        let it_y = self.start_y()..=self.end_y();
        if self.y_descending() {
            Box::new(it_y.into_iter().rev())
        }
        else {
            Box::new(it_y.into_iter())
        }
    }

    fn start_x(self: &Self) -> u64 { u64::min(self.0.0, self.1.0) }
    fn end_x(self: &Self)   -> u64 { u64::max(self.0.0, self.1.0) }
    fn start_y(self: &Self) -> u64 { u64::min(self.0.1, self.1.1) }
    fn end_y(self: &Self)   -> u64 { u64::max(self.0.1, self.1.1) }

    fn x_descending(self: &Self) -> bool { self.0.0 > self.1.0 }
    fn y_descending(self: &Self) -> bool { self.0.1 > self.1.1 }

    fn is_horizontal(self: &Self) -> bool {
        self.start_y() == self.end_y()
    }

    fn is_vertical(self: &Self) -> bool {
        self.start_x() == self.end_x()
    }

    fn is_diagonal(self: &Self) -> bool {
        self.end_x()-self.start_x() == self.end_y()-self.start_y()
    }

    fn is_straight(self: &Self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }
}

#[derive(Debug)]
struct PointCount {
    data: HashMap<Point, u64>
}

impl FromIterator<Point> for PointCount {
    fn from_iter<T>(it: T) -> Self where T: IntoIterator<Item=Point> {
        let mut pc = PointCount{
            data: HashMap::<Point, u64>::new(),
        };
        for point in it {
            match pc.data.get_mut(&point) {
                Some(count) => {*count+=1;}
                None => {pc.data.insert(point, 1);}
            }
        }
        pc
    }
}

impl PointCount {
    fn overlaps(self: &Self) -> u64 {
        self.data.iter().filter(|(_, c)| **c>1).count() as u64
    }
}

fn parse_input(s: &str) -> Vec<Line> {
    s
        .split('\n')
        .filter_map(|item| item.parse::<Line>().ok())
        .collect()
}

pub fn overlapping_straight_line_count() -> u64 {
    parse_input(INPUT)
        .into_iter()
        .filter(Line::is_straight)
        .flat_map(|l| l.fill().into_iter())
        .collect::<PointCount>()
        .overlaps()
}

pub fn overlapping_diagonal_line_count() -> u64 {
    parse_input(INPUT)
        .into_iter()
        .flat_map(|l| l.fill().into_iter())
        .collect::<PointCount>()
        .overlaps()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_point() {
        let point = "1,2".parse::<Point>().unwrap();
        assert_eq!(Point(1,2), point);
        assert!("".parse::<Point>().is_err());
        assert!(",".parse::<Point>().is_err());
        assert!("1,".parse::<Point>().is_err());
        assert!("1,2,3".parse::<Point>().is_err());
    }

    #[test]
    fn parse_line() {
        let line = "1,2 -> 3,4".parse::<Line>().unwrap();
        assert_eq!(Line(Point(1,2),Point(3,4)), line);
        let line = " 1, 2 -> 3 ,4 ".parse::<Line>().unwrap();
        assert_eq!(Line(Point(1,2),Point(3,4)), line);
        assert!("".parse::<Line>().is_err());
        assert!("1 -> 2".parse::<Line>().is_err());
        assert!("1,2 -> 3".parse::<Line>().is_err());
    }

    #[test]
    fn line_is_horizontal() {
        let line: Line = "1,2 -> 4,2".parse().unwrap();
        assert!( line.is_horizontal());
        assert!(!line.is_vertical());
        assert!( line.is_straight());
        assert!(!line.is_diagonal());
    }

    #[test]
    fn line_is_vertical() {
        let line: Line = "1,4 -> 1,2".parse().unwrap();
        assert!(!line.is_horizontal());
        assert!( line.is_vertical());
        assert!( line.is_straight());
        assert!(!line.is_diagonal());
    }

    #[test]
    fn line_is_diagnoal() {
        let line: Line = "0,0 -> 3,3".parse().unwrap();
        assert!(!line.is_horizontal());
        assert!(!line.is_vertical());
        assert!(!line.is_straight());
        assert!( line.is_diagonal());

        assert!("0,0 -> 3,3".parse::<Line>().unwrap().is_diagonal());
        assert!("3,3 -> 0,0".parse::<Line>().unwrap().is_diagonal());
        assert!("0,3 -> 3,0".parse::<Line>().unwrap().is_diagonal());
        assert!("3,0 -> 0,3".parse::<Line>().unwrap().is_diagonal());
    }

    #[test]
    fn fill_horizontal_line() {
        let line: Line = "0,1 -> 3,1".parse().unwrap();
        let points = line.fill();
        assert_eq!(
            vec![Point(0,1), Point(1,1), Point(2,1), Point(3,1)],
            points
        );
    }

    #[test]
    fn fill_horizontal_line_reversed() {
        assert_eq!(4, Line(Point(0, 0), Point(0, 3)).fill().iter().count());
        assert_eq!(4, Line(Point(0, 3), Point(0, 0)).fill().iter().count());
        assert_eq!(4, Line(Point(0, 0), Point(3, 0)).fill().iter().count());
        assert_eq!(4, Line(Point(3, 0), Point(0, 0)).fill().iter().count());
    }

    #[test]
    fn fill_vertical_line() {
        let line: Line = "1,0 -> 1,3".parse().unwrap();
        let points = line.fill();
        assert_eq!(
            vec![Point(1,0), Point(1,1), Point(1,2), Point(1,3)],
            points
        );
    }

    #[test]
    fn fill_diagnoal() {
        assert_eq!(4, Line(Point(0, 0), Point(3, 3)).fill().iter().count());
        assert_eq!(4, Line(Point(3, 3), Point(0, 0)).fill().iter().count());
        assert_eq!(4, Line(Point(0, 3), Point(3, 0)).fill().iter().count());
        assert_eq!(4, Line(Point(3, 0), Point(0, 3)).fill().iter().count());

        assert_eq!(
            vec![Point(0,3),Point(1,2),Point(2,1),Point(3,0)],
            Line(Point(0,3), Point(3, 0)).fill()
        );

        assert_eq!(
            vec![Point(2,2),Point(1,1),Point(0,0)],
            Line(Point(2,2), Point(0, 0)).fill()
        );
    }

    #[test]
    fn count_points() {
        let points1 = vec![Point(0,1), Point(1,1), Point(2,1), Point(3,1)];
        let points2 = vec![Point(1,0), Point(1,1), Point(1,2), Point(1,3)];
        let count: PointCount = points1.into_iter().chain(points2.into_iter()).collect();
        let overlaps = count.overlaps();
        assert_eq!(1, overlaps);
    }

    const EXAMPLE_LINES: &str = "
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

    #[test]
    fn example_overlaps_straight_only() {
        let lines: Vec::<Line> = parse_input(EXAMPLE_LINES);
        assert_eq!(10, lines.iter().count());
        let straight_lines: Vec::<Line> = lines.into_iter().filter(Line::is_straight).collect();
        assert_eq!(6, straight_lines.iter().count());
        let points: Vec::<Point> = straight_lines.into_iter().flat_map(|l| l.fill().into_iter()).collect();
        assert_eq!(26, points.iter().count());
        let counter: PointCount = points.into_iter().collect();
        assert_eq!(5, counter.overlaps());
    }

    #[test]
    fn example_overlaps_diagnoal() {
        let lines: Vec::<Line> = parse_input(EXAMPLE_LINES);
        assert_eq!(10, lines.iter().count());
        let points: Vec::<Point> = lines.into_iter().flat_map(|l| l.fill().into_iter()).collect();
        assert_eq!(53, points.iter().count());
        let counter: PointCount = points.into_iter().collect();
        assert_eq!(12, counter.overlaps());
    }

    #[test]
    fn example_overlaps_diagnoal_custom() {
        const CUSTOM_LINES: &str = "
            1,0->3,2
            0,3->3,0
            0,3->1,2
            3,2->1,2
            2,0->0,2
        ";

        let lines: Vec::<Line> = parse_input(CUSTOM_LINES);
        assert_eq!(5, lines.iter().count());
        let points: Vec::<Point> = lines.into_iter().flat_map(|l| l.fill().into_iter()).collect();
        assert_eq!(15, points.iter().count());
        let counter: PointCount = points.into_iter().collect();
        assert_eq!(4, counter.overlaps());
    }

    #[test]
    fn pointlike() {
        assert_eq!(1, (0..=0).into_iter().rev().count());
    }

    fn expect_overlaps(data: &str, expectation: u64) {
        let lines: Vec<Line> = parse_input(data).into_iter().collect();
        let points: Vec<Point> = lines.into_iter().flat_map(|l| l.fill().into_iter()).collect();
        let counter: PointCount = points.into_iter().collect();
        assert_eq!(expectation, counter.overlaps());
    }

    #[test]
    fn intersect_l2r_t2b() {
        let data = "
            0,1->2,1
            1,0->1,2
        ";
        expect_overlaps(data, 1);
    }

    #[test]
    fn intersect_r2l_b2t() {
        let data = "
            2,1->0,1
            1,2->1,0
        ";
        expect_overlaps(data, 1);
    }

    #[test]
    fn intersect_r2l_t2b() {
        let data = "
            2,1->0,1
            1,0->1,2
        ";
        expect_overlaps(data, 1);
    }

    #[test]
    fn intersect_l2r_b2t() {
        let data = "
            0,1->2,1
            1,2->1,0
        ";
        expect_overlaps(data, 1);
    }
    
    #[test]
    fn intersect_tl2br_tr2bl() {
        let data = "
            0,0->2,2
            2,0->0,2
        ";
        expect_overlaps(data, 1);
    }

    #[test]
    fn intersect_br2tl_bl2tr() {
        let data = "
            2,2->0,0
            0,2->2,0
        ";
        expect_overlaps(data, 1);
    }

    #[test]
    fn intersect_tl2br_bl2tr() {
        let data = "
            0,0->2,2
            0,2->2,0
        ";
        expect_overlaps(data, 1);
    }

    #[test]
    fn intersect_br2tl_tr2bl() {
        let data = "
            2,2->0,0
            2,0->0,2
        ";
        expect_overlaps(data, 1);
    }
}
