const INPUT: &str = include_str!("../data/3/input");
const DIAGNOSTIC_WIDTH: usize = 12; // Power diagnostics are 12 bit

#[derive(Default, Debug)]
struct PowerDiagnostic 
{
    bitcount: [u32; DIAGNOSTIC_WIDTH],
    total: u32,
}

fn parse_input(s: &str) -> Vec<u32>
{
    s
        .split('\n') // Lines
        .filter_map(|item| u32::from_str_radix(item, 2).ok()) // Parse binary
        .collect() // collect the u32
}

impl PowerDiagnostic
{
    fn new() -> Self
    {
        Default::default()
    }

    fn from_data<I: IntoIterator<Item=u32>>(data: I) -> Self
    {
        // This very for-loopy... maybe this could be done more consisely with reduce
        let mut diagnostic = PowerDiagnostic::new();
        for item in data.into_iter()
        {
            for bit in 0..DIAGNOSTIC_WIDTH
            {
                if (item & (1<<bit)) >= 1
                {
                    diagnostic.bitcount[bit as usize] += 1;
                }
            }
            diagnostic.total += 1;
        }
        diagnostic
    }

    fn gamma(self: &Self) -> u32
    {
        self.bitcount
            .iter()
            .enumerate()
            .filter(|(_, it)| {**it > self.total/2}) // Check if this bit the most common
            .map(|(i, _)| {1<<i})
            .sum()
    }
}

fn gamma_to_epsilon(gamma: u32) -> u32
{
    !gamma & (1<<DIAGNOSTIC_WIDTH)-1
}

pub fn calculate_power_consumption() -> u32
{
    let data = parse_input(INPUT);
    let pd = PowerDiagnostic::from_data(data);
    let gamma = pd.gamma();
    let epsilon = gamma_to_epsilon(gamma);
    gamma * epsilon 
}

mod test {

    #[test]
    fn power_consumption()
    {
        assert_eq!(3901196u32, super::calculate_power_consumption());
    }
}
