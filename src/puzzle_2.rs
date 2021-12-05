use std::str::FromStr;

const INPUT: &str = include_str!("../data/2/input");

#[derive(Debug)]
enum Direction 
{
    UP, 
    DOWN, 
    FORWARD,
}

#[derive(Debug)]
struct Move 
{
    direction: Direction,
    stepsize: i32,
}

#[derive(Debug)]
struct Position
{
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new() -> Self
    {
        Position{horizontal: 0, depth: 0, aim: 0}
    }

    fn update(self: &mut Self, m: Move) -> () 
    {
        use Direction::*;
        match m.direction {
            UP => {self.depth -= m.stepsize;}
            DOWN => {self.depth += m.stepsize;}
            FORWARD => {self.horizontal += m.stepsize;}
        }
    }

    fn update_aimed(self: &mut Self, m: Move) -> ()
    {
        use Direction::*;
        match m.direction {
            UP => {
                self.aim -= m.stepsize;
            }
            DOWN => {
                self.aim += m.stepsize;
            }
            FORWARD => {
                self.horizontal += m.stepsize;
                self.depth += self.aim*m.stepsize;
            }
        }
    }

    fn metric(self: &Self) -> i32
    {
        self.horizontal * self.depth
    }
}

struct ParseDirectionError {}

impl FromStr for Direction {
    type Err = ParseDirectionError;
    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s {
            "up" => Ok(Direction::UP),
            "down" => Ok(Direction::DOWN),
            "forward" => Ok(Direction::FORWARD),
            _ => Err(Self::Err{})
        }
    }
}

fn parse_move(s: &str) -> Option<Move>
{
    let mut it = s.split(' ');
    Some(
        Move {
           direction: it.next()?.parse().ok()?,
           stepsize: it.next()?.parse().ok()?,
        }
    )
}

fn parse_input(s: &str) -> Vec<Move>
{
    s
        .split('\n')
        .filter_map(parse_move)
        .collect()
}

pub fn compute_position() -> i32
{
    let data = parse_input(INPUT);
    let mut position = Position::new();
    for m in data {
        position.update(m);
    }
    position.metric()
}

pub fn compute_position_aimed() -> i32
{
    let data = parse_input(INPUT);
    let mut position = Position::new();
    for m in data {
        position.update_aimed(m);
    }
    position.metric()
}
