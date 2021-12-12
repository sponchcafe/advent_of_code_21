const INPUT: &str = include_str!("../data/7/input");

type CostFn = fn(i32, i32) -> u32;

fn parse_input(s: &str) -> Vec<i32> { s
        .split(',')
        .map(str::trim)
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect()
}

fn constant_cost(from: i32, to: i32) -> u32 {
    i32::abs(to-from) as u32
}

fn linear_cost(from: i32, to: i32) -> u32 {
    let steps = i32::abs(to-from);
    ((i32::pow(steps, 2)+steps)/2) as u32
}

fn align_cost(coords: &Vec<i32>, coord: i32, cost_fn: CostFn) -> u32 {
    coords
        .iter()
        .map(|&c| cost_fn(c, coord))
        .sum::<u32>()
}

fn best_align_cost(coords: Vec<i32>, cost_fn: CostFn) -> Option<u32> {
    let min = *coords.iter().min()?;
    let max = *coords.iter().max()?;
    (min..=max).into_iter()
        .map(|c| align_cost(&coords, c, cost_fn))
        .min()
}

pub fn align_crabs_cost() -> u32 {
    let crabs = parse_input(INPUT);
    best_align_cost(crabs, constant_cost).unwrap()
}

pub fn align_crabs_cost_linear() -> u32 {
    let crabs = parse_input(INPUT);
    best_align_cost(crabs, linear_cost).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn align_no_coord() {
        let crabs = vec![];
        assert_eq!(None, best_align_cost(crabs, constant_cost));
    }

    #[test]
    fn align_single_coord() {
        let crabs = vec![4];
        assert_eq!(Some(0), best_align_cost(crabs, constant_cost));
    }

    #[test]
    fn align_two_coords() {
        let crabs = vec![4, 8];
        assert_eq!(Some(4), best_align_cost(crabs, constant_cost));
    }

    #[test]
    fn align_three_coords() {
        let crabs = vec![4, 8, 16];
        assert_eq!(Some(12), best_align_cost(crabs, constant_cost));
    }

    #[test]
    fn example_crabs() {
        let crabs = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(Some(37), best_align_cost(crabs, constant_cost));
    }

    #[test]
    fn linear_cost_calculation() {
        assert_eq!(66, linear_cost(16, 5));
        assert_eq!(10, linear_cost(1, 5));
        assert_eq!(6, linear_cost(2, 5));
    }

    #[test]
    fn example_crabs_linear() {
        let crabs = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(Some(168), best_align_cost(crabs, linear_cost));
    }
}
