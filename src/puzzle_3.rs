
const INPUT: &str = include_str!("../data/3/input");
const DIAGNOSTIC_WIDTH: usize = 12; // Power diagnostics are 12 bit

#[derive(Debug, Clone)]
struct Diagnostic<const WIDTH: usize> 
{
    data: Vec<u32>,
    bitcount: [u32; WIDTH],
}

fn parse_input(s: &str) -> Vec<u32>
{
    s
        .split('\n') // Lines
        .filter_map(|item| u32::from_str_radix(item, 2).ok()) // Parse binary
        .collect() // collect the u32
}

impl<const WIDTH: usize> Diagnostic<WIDTH>
{
    fn new() -> Self
    {
        Diagnostic {
            data: Vec::new(),
            bitcount: [0u32; WIDTH],
        }
    }
    
    fn push(self: &mut Self, val: u32)
    {
        for bit in 0..WIDTH
        {
            if (val & 1<<bit) != 0
            {
                self.bitcount[bit as usize] += 1;
            }
        }
        self.data.push(val);
    }

    fn common_bit(self: &Self, idx: usize) -> u32
    {
        let total = self.len() as u32;
        let ones = self.bitcount[idx];
        let zeros = total - ones;
        let common = (ones >= zeros) as u32;
        let cbit = common << idx;
        cbit
    }

    fn common(self: &Self) -> u32
    {
        (0..WIDTH).into_iter().map(|i| self.common_bit(i)).sum()
    }

    fn filter_by_common_bit(self: Self, idx: usize) -> Self 
    {
        let cbit = self.common_bit(idx);
        self.data.into_iter().filter(|it| (it & 1<<idx) == cbit).collect()
    }

    fn filter_by_uncommon_bit(self: Self, idx: usize) -> Self 
    {
        let cbit = self.common_bit(idx);
        self.data.into_iter().filter(|it| (it & 1<<idx) != cbit).collect()
    }
    
    fn len(self: &Self) -> usize
    {
        self.data.len()
    }
}

impl<const WIDTH: usize> FromIterator<u32> for Diagnostic<WIDTH>
{
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item=u32>
    {
        let mut diagnostic = Diagnostic::<WIDTH>::new();
        for item in iter
        {
            diagnostic.push(item);
        }
        diagnostic
    }
}

fn gamma<const WIDTH: usize>(diagnostic: &Diagnostic::<WIDTH>) -> u32
{
    diagnostic.common()
}

fn gamma_to_epsilon(gamma: u32) -> u32
{
    !gamma & (1<<DIAGNOSTIC_WIDTH)-1
}

pub fn calculate_power_consumption() -> u32
{
    let data = parse_input(INPUT);
    let diagnostic = data.into_iter().collect::<Diagnostic::<DIAGNOSTIC_WIDTH>>();
    let gamma = gamma(&diagnostic);
    let epsilon = gamma_to_epsilon(gamma);
    gamma * epsilon 
}

fn oxygen_rating<const WIDTH: usize>(diagnostic: Diagnostic::<WIDTH>) -> u32
{
    let mut d = diagnostic;
    for i in (0..WIDTH).rev() {
        if d.len() == 1 { break }
        d = d.filter_by_common_bit(i);
    }
    d.data[0]
}

fn co2_rating<const WIDTH: usize>(diagnostic: Diagnostic::<WIDTH>) -> u32
{
    let mut d = diagnostic;
    for i in (0..WIDTH).rev() {
        if d.len() == 1 { break }
        d = d.filter_by_uncommon_bit(i);
    }
    d.data[0]
}

pub fn calculate_life_rating() -> u32
{
    let data = parse_input(INPUT);
    let diagnostic = data.into_iter().collect::<Diagnostic::<DIAGNOSTIC_WIDTH>>();
    let ox_rating = oxygen_rating(diagnostic.clone());
    let co2_rating = co2_rating(diagnostic);

    ox_rating*co2_rating
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn diagnostic_create() 
    {
        let d = Diagnostic::<8>::new();
        let g = gamma(&d);
        assert_eq!(0xFF, g);
    }

    #[test]
    fn diagnostic_push() 
    {
        let mut d = Diagnostic::<5>::new();
        d.push(0b01111);
        d.push(0b00111);
        d.push(0b00011);
        d.push(0b00001);
        assert_eq!(0b00001, d.common_bit(0));
        assert_eq!(0b00010, d.common_bit(1));
        assert_eq!(0b00100, d.common_bit(2));
        assert_eq!(0b00000, d.common_bit(3));
        assert_eq!(0b00000, d.common_bit(4));
        assert_eq!(0b00111, d.common());
    }

    #[test]
    fn power_consumption()
    {
        assert_eq!(3901196u32, calculate_power_consumption());
    }

    const TEST_DATA: [u32; 12] = [
        0b00100u32, 0b11110, 0b10110, 0b10111,
        0b10101, 0b01111, 0b00111, 0b11100,
        0b10000, 0b11001, 0b00010, 0b01010,
    ];

    #[test]
    fn oygen_rating_example()
    {
        let diagnostic = TEST_DATA.into_iter().collect::<Diagnostic::<5>>();
        assert_eq!(0b10111, oxygen_rating(diagnostic));
    }
    #[test]
    fn co2_rating_example()
    {
        let diagnostic = TEST_DATA.into_iter().collect::<Diagnostic::<5>>();
        assert_eq!(0b01010, co2_rating(diagnostic));
    }
}
