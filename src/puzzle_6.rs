const INPUT: &str = include_str!("../data/6/input");

#[derive(Debug, PartialEq, Clone)]
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

    fn birthdays(self: &Self, start_day: usize) -> Birthdays {
        Birthdays{
            next: self.0 as usize + start_day,
        }
    }
}

const GENERATION_TIME: usize = 7;

struct Birthdays{
    next: usize,
}

impl Iterator for Birthdays {
    type Item=usize;
    fn next(self: &mut Self) -> Option<Self::Item> {
        let ret = Some(self.next+1);
        self.next += GENERATION_TIME;
        ret
    }
}

impl From<u8> for Fish {
    fn from(life: u8) -> Self {
        Fish(life)
    }
}

#[allow(unused)]
fn simulate_growth(mut fish: Vec::<Fish>, days: usize) -> usize {
    for _ in 0..days {
        let new_fish: Vec<Fish> = fish.iter_mut().filter_map(|f| f.grow()).collect();
        fish.extend(new_fish);
    }
    fish.len()
}

fn calculate_growth(fish: Vec::<Fish>, days: usize) -> usize {
    let mut births = vec![0; days+1];
    // Add the fish birth for the initial fish population
    for f in fish.iter() {
        for bday in f.birthdays(0).take_while(|i| *i<=days) {
            births[bday] += 1;
        }
    }
    // Add the fish birth for all the offspring of the initial population day by day
    for d in 0..days {
        let fish = Fish::new();
        for bday in fish.birthdays(d).take_while(|i| *i<=days) {
            births[bday] += births[d];
        }
    }
    births.iter().sum::<usize>() + fish.len() // Final fish count is sum of all offspring + initial fish
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

pub fn lanternfish_population(days: usize) -> usize {
    let fish = parse_input(INPUT);
    calculate_growth(fish, days)
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

    #[test]
    fn birthdays_from_0() {
        let fish = Fish::new();
        let birthdays: Vec::<usize> = fish.birthdays(0).take_while(|i| *i<32).collect();
        assert_eq!(vec![9, 16, 23, 30], birthdays);
    }

    #[test]
    fn birthdays() {
        let fish = Fish(2);
        let birthdays: Vec::<usize> = fish.birthdays(25).take_while(|i| *i<50).collect();
        assert_eq!(vec![28, 35, 42, 49], birthdays);
    }

    #[test]
    fn calculate_fish_growth_simple() {
        let fish = vec![Fish(2), Fish(4)];
        assert_eq!(5, calculate_growth(fish.clone(), 10));
        assert_eq!(8, calculate_growth(fish, 15));
    }

    #[test]
    fn calculate_fish_growth_80() {
        let fish = example_fish();
        assert_eq!(5934, calculate_growth(fish, 80));
    }

    #[test]
    fn calculate_fish_growth_256() {
        let fish = example_fish();
        assert_eq!(26984457539, calculate_growth(fish, 256));
    }
}
