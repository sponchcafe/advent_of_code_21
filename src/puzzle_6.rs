const INPUT: &str = include_str!("../data/6/input");

#[derive(Debug, PartialEq)]
struct Fish(u8);

impl Fish {
    fn new() -> Self {
        Fish(8)
    }

    fn grow(self: &mut Self) -> Option<Fish> {
        if self.0 == 0 {
            self.0 = 6;
            Some(Fish::new())
        }
        else {
            self.0 -= 1;
            None
        }
    }
}

impl From<u8> for Fish {
    fn from(life: u8) -> Self {
        Fish(life)
    }
}

fn simulate_growth(mut fish: Vec::<Fish>, days: u32) -> usize {
    for _ in 0..days {
        let new_fish: Vec<Fish> = fish.iter_mut().filter_map(|f| f.grow()).collect();
        fish.extend(new_fish);
    }
    fish.len()
}

fn parse_input(s: &str) -> Vec<Fish> {
    s
        .split(',')
        .map(str::trim)
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .map(Fish::from)
        .collect()
}

pub fn lanternfish_population() -> usize {
    let fish = parse_input(INPUT);
    simulate_growth(fish, 80)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grow_fish() {
        let mut f = Fish::new();
        assert_eq!(8, f.0);
        for _ in 0..8 {
            assert_eq!(None, f.grow());
        }
        assert_eq!(Some(Fish(8)), f.grow());
    }

    fn example_fish() -> Vec::<Fish> {
        vec![3u8,4,3,1,2].into_iter().map(Fish::from).collect()
    }

    #[test]
    fn lanternfish_example_18() {
        let fish = example_fish();
        assert_eq!(26, simulate_growth(fish, 18));
    }

    #[test]
    fn lanternfish_example_80() {
        let fish = example_fish();
        assert_eq!(5934, simulate_growth(fish, 80));
    }
}
