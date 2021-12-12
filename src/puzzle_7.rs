const INPUT: &str = include_str!("../data/7/input");

fn parse_input(s: &str) -> Vec<i32> {
    s
        .split(',')
        .map(str::trim)
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect()
}

fn align_cost(coords: &Vec<i32>, coord: i32) -> u32 {
    coords
        .iter()
        .map(|&c| i32::abs(c-coord) as u32)
        .sum::<u32>()
}

fn best_align_cost(coords: Vec<i32>) -> Option<u32> {
    let min = *coords.iter().min()?;
    let max = *coords.iter().max()?;
    (min..=max).into_iter()
        .map(|c| align_cost(&coords, c))
        .min()
}

pub fn align_crabs_cost() -> u32 {
    let crabs = parse_input(INPUT);
    best_align_cost(crabs).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn align_no_coord() {
        let crabs = vec![];
        assert_eq!(None, best_align_cost(crabs));
    }

    #[test]
    fn align_single_coord() {
        let crabs = vec![4];
        assert_eq!(Some(0), best_align_cost(crabs));
    }

    #[test]
    fn align_two_coords() {
        let crabs = vec![4, 8];
        assert_eq!(Some(4), best_align_cost(crabs));
    }

    #[test]
    fn align_three_coords() {
        let crabs = vec![4, 8, 16];
        assert_eq!(Some(12), best_align_cost(crabs));
    }

    #[test]
    fn example_crabs() {
        let crabs = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(Some(37), best_align_cost(crabs));
    }
}
