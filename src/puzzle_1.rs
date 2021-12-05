const INPUT: &str = include_str!("../data/1/input");

fn parse_input(s: &str) -> Vec<i32>
{
    s
        .split('\n') // Lines
        .filter_map(|item| item.parse::<i32>().ok()) // parse to i32 (if some)
        .collect() // collect the i32
}

pub fn count_increasing_depth() -> usize
{
    let data = parse_input(INPUT);
    let it1 = data.iter();
    let mut it2 = data.iter();
    it2.next();
    it1.zip(it2).filter(|(i1, i2)| i2 > i1).count()
}

pub fn count_sliding_window_increasing_depth() -> usize
{
    let data = parse_input(INPUT);
    let it1 = data.windows(3);
    let mut it2 = it1.clone();
    it2.next();
    it1
        .zip(it2)
        .filter(|(i1, i2)| &i2.iter().sum::<i32>() > &i1.iter().sum::<i32>())
        .count()
}
