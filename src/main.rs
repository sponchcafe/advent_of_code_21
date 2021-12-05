const INPUT: &str = include_str!("../data/1/input");

fn parse_input(s: &str) -> Vec<i32>
{
    s
        .split('\n') // Lines
        .filter_map(|item| item.parse::<i32>().ok()) // parse to i32 (if some)
        .collect() // collect the i32
}

fn count_increasing_depth() -> usize
{
    let data = parse_input(INPUT);
    let it1 = data.iter();
    let mut it2 = data.iter();
    it2.next();
    it1.zip(it2).filter(|(i1, i2)| i2 > i1).count()
}


fn main() {
    println!("Puzzle 1.1: There are {} instances of increased depth.", count_increasing_depth());
}
