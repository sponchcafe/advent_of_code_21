use std::str::FromStr;
use std::collections::BTreeSet;

const DOTS: &str = include_str!("../data/13/dots");
const FOLDS: &str = include_str!("../data/13/folds");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Dot(i32, i32);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Fold{ X(i32), Y(i32) }

#[derive(Debug)]
struct Dots(BTreeSet<Dot>);

impl FromStr for Dot {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .trim()
            .split(',')
            .map(str::trim)
            .map(i32::from_str);

        match (parts.next(), parts.next(), parts.next()) {
            (Some(Ok(x)), Some(Ok(y)), None) => {Ok(Dot(x, y))},
            _ => {Err(())}
        }
    }
}

impl FromStr for Fold {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .trim()
            .split('=')
            .map(str::trim);

        match (parts.next(), parts.next(), parts.next()) {
            (Some("fold along x"), Some(x), None) => {
                let x = x.parse::<i32>().map_err(|_| ())?;
                Ok(Fold::X(x))
            },
            (Some("fold along y"), Some(y), None) => {
                let y = y.parse::<i32>().map_err(|_| ())?;
                Ok(Fold::Y(y))
            },
            _ => {Err(())}
        }
    }
}

impl FromIterator<Dot> for Dots {
    fn from_iter<T: IntoIterator<Item=Dot>>(iter: T) -> Dots {
        Dots(iter.into_iter().collect())
    }
}

impl Dots {
    fn len(self: &Self) -> usize {
        self.0.len()
    }

    fn fold<T: IntoIterator<Item=Fold>>(self: Self, folds: T) -> Self {
        let mut dots = self.0;
        for f in folds {
            dots = dots
                .into_iter()
                .map(|d| fold_dot(d, f))
                .collect()
        }
        Dots(dots)
    }
}

impl ToString for Dots {
    fn to_string(self: &Self) -> String {
        let dots = &self.0;
        let max_x = dots.iter().map(|d| d.0).max().unwrap() as usize;
        let max_y = dots.iter().map(|d| d.1).max().unwrap() as usize;
        let mut display = String::with_capacity((max_x+1)*max_y);
        for y in 0..=max_y {
            for x in 0..=max_x {
                if dots.contains(&Dot(x as i32,y as i32)) {
                    display.push('#');
                }
                else {
                    display.push(' ');
                }
            }
            display.push('\n');
        }
        display
    }
}

fn parse_dots(s: &str) -> Dots
{
    s
        .trim()
        .lines()
        .map(str::trim)
        .map(Dot::from_str)
        .map(Result::unwrap)
        .collect()
}

fn parse_folds(s: &str) -> Vec<Fold>
{
    s
        .trim()
        .lines()
        .map(str::trim)
        .map(Fold::from_str)
        .map(Result::unwrap)
        .collect()
}

fn fold_dot(dot: Dot, fold: Fold) -> Dot {
    match fold {
        Fold::X(x) if dot.0 > x => { Dot(2*x - dot.0, dot.1) }
        Fold::Y(y) if dot.1 > y => { Dot(dot.0, 2*y - dot.1) }
        _ => { dot }
    }
}

pub fn dots_after_one_fold() -> u64 {
    let mut dots: Dots = parse_dots(DOTS);
    let folds: Vec<Fold> = parse_folds(FOLDS);
    dots = dots.fold(folds.into_iter().take(1));
    dots.len() as u64
}

pub fn fold_and_format_dots() -> String {
    let dots: Dots = parse_dots(DOTS);
    let folds: Vec<Fold> = parse_folds(FOLDS);
    let dots = dots.fold(folds.into_iter());
    dots.to_string()
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn dot_parsing() {
        assert_eq!(Dot(0, 0), "0,0".parse::<Dot>().unwrap());
        assert_eq!(Dot(3, -4), " 3, -4".parse::<Dot>().unwrap());
        assert!("".parse::<Dot>().is_err());
        assert!("1,".parse::<Dot>().is_err());
        assert!("1,2,3".parse::<Dot>().is_err());
        assert!("1,x".parse::<Dot>().is_err());
    }

    #[test]
    fn fold_parsing() {
        assert_eq!(Fold::X(0), "fold along x=0".parse::<Fold>().unwrap());
        assert_eq!(Fold::Y(11), "fold along y=11".parse::<Fold>().unwrap());
        assert!("".parse::<Fold>().is_err());
        assert!("fold along p=2".parse::<Fold>().is_err());
        assert!("fold along y=2=0".parse::<Fold>().is_err());
    }

    #[test]
    fn fold_x() {
        let dots    = [Dot(5,7), Dot(20, 1), Dot(0,0), Dot(4, 4)];
        let fold    = Fold::X(4);
        let expect  = [Dot(3,7), Dot(-12, 1), Dot(0,0), Dot(4, 4)];
        for (dot, exp) in dots.into_iter().zip(expect.into_iter()) {
            assert_eq!(exp, fold_dot(dot, fold));
        }
    }

    #[test]
    fn fold_y() {
        let dots    = [Dot(7,5), Dot(1, 20), Dot(0,0), Dot(4, 4)];
        let fold    = Fold::Y(4);
        let expect  = [Dot(7,3), Dot(1, -12), Dot(0,0), Dot(4, 4)];
        for (dot, exp) in dots.into_iter().zip(expect.into_iter()) {
            assert_eq!(exp, fold_dot(dot, fold));
        }
    }

    #[test]
    fn dot_collection() {
        let dots = [Dot(7,5), Dot(1, 20), Dot(0,0), Dot(4, 4)];
        let coll: Dots = dots.into_iter().collect();
        assert_eq!(4, coll.0.len());
    }

    const EX_DOTS: &str = include_str!("../data/13/example_dots");
    const EX_FOLDS: &str = include_str!("../data/13/example_folds");

    #[test]
    fn single_fold_example() {
        let dots: Dots = parse_dots(EX_DOTS);
        let folds: Vec<Fold> = parse_folds(EX_FOLDS);
        let dots = dots.fold(folds.into_iter().take(1));
        assert_eq!(17, dots.len());

    }

}
