
const INPUT: &str = include_str!("../data/3/input");
const DIAGNOSTIC_WIDTH: usize = 12; // Power diagnostics are 12 bit

#[derive(Debug)]
struct Diagnostic<const WIDTH: usize> 
{
    bitcount: [u32; WIDTH],
    total: u32,
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
            bitcount: [0u32; WIDTH],
            total: 0,
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
        self.total += 1;
    }

    fn from_iter<'a, T>(iter: T) -> Self where T: IntoIterator<Item = &'a u32>
    {
        let mut diagnostic = Diagnostic::<WIDTH>::new();
        for item in iter
        {
            diagnostic.push(*item);
        }
        diagnostic
    }

    fn common_bit(self: &Self, idx: usize) -> u32
    {
        ((self.bitcount[idx] >= self.total/2) as u32) << idx
    }

    fn common(self: &Self) -> u32
    {
        (0..WIDTH).into_iter().map(|i| self.common_bit(i)).sum()
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
    let diagnostic = Diagnostic::<DIAGNOSTIC_WIDTH>::from_iter(data.iter());
    let gamma = gamma(&diagnostic);
    let epsilon = gamma_to_epsilon(gamma);
    gamma * epsilon 
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
}
