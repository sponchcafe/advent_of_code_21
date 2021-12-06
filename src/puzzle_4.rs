const INPUT_NUMBERS: &str = include_str!("../data/4/numbers");
const INPUT_BOARDS: &str = include_str!("../data/4/boards");

fn parse_numbers(s: &str) -> Vec<u32>
{
    s
        .split(',') // Lines
        .filter_map(|item| item.parse::<u32>().ok()) // parse to i32 (if some)
        .collect() // collect the i32
}

fn parse_boards(s: &str, shape: (usize, usize)) -> Vec<Board>
{
    s
        .split("\n\n") // Blocks
        .map(|block| {
            let data : Vec<u32> = block
                .split(|c: char| c.is_whitespace())
                .filter_map(|item| item.parse::<u32>().ok())
                .collect();
            Board::new(&data[..], shape)
        }).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Field {
    Checked(u32),
    Unchecked(u32),
}

impl From<&u32> for Field {
    fn from(v: &u32) -> Self {
        Field::Unchecked(*v)
    }
}

#[derive(Debug)]
struct Board { 
    data: Vec<Field>,
    shape: (usize, usize),
    last_checked: u32,
}

#[derive(Debug)]
struct Rows<'a> {
    iter: std::slice::Chunks<'a, Field>,
}

#[derive(Debug)]
struct Row<'a> {
    data: std::slice::Iter<'a, Field>,
}

#[derive(Debug)]
struct Cols<'a> {
    data: &'a [Field],
    shape: usize,
    c: usize,
}

#[derive(Debug)]
struct Col<'a> {
    data: std::iter::StepBy<std::iter::Skip<std::slice::Iter<'a, Field>>>
}

trait Bingo {
    fn bingo(&mut self) -> bool;
}

impl<T> Bingo for T where T: Iterator<Item=Field> {
    fn bingo(self: &mut Self) -> bool {
        self.all(|f| {
            match f {
                Field::Checked(_) => true,
                _ => false,
            }
        })
    }
}

impl<'a> Iterator for Rows<'a> {
    type Item = Row<'a>;
    fn next(self: &mut Self) -> Option<Self::Item> {
        Some(Row{data: self.iter.next()?.iter()})
    }
}

impl<'a> Iterator for Row<'a> {
    type Item = Field;
    fn next(self: &mut Self) -> Option<Self::Item> {
        self.data.next().map(Field::to_owned)
    }
}

impl<'a> Iterator for Cols<'a> {
    type Item = Col<'a>;
    fn next(self: &mut Self) -> Option<Self::Item> {
        let col = Some(Col{
            data: self.data.iter().skip(self.c).step_by(self.shape),
        });
        self.c += 1;
        if self.c > self.shape { None }
        else { col } 
    }
}

impl<'a> Iterator for Col<'a> {
    type Item = Field;
    fn next(self: &mut Self) -> Option<Self::Item> {
        self.data.next().map(Field::to_owned)
    }
}

impl Board {
    fn new(data: &[u32], shape: (usize, usize)) -> Self {
        Board{
            data: data.iter().map(Field::from).collect(),
            shape,
            last_checked: 0,
        }
    }

    fn rows(self: &Self) -> Rows {
        Rows{ iter: self.data.chunks(self.shape.0) }
    }

    fn cols(self: &Self) -> Cols {
        Cols{ data: &self.data[..], shape: self.shape.1, c: 0 }
    }

    fn check(self: &mut Self, number: u32) {
        use Field::*;
        for f in self.data.iter_mut() {
            if let &mut Unchecked(v) = f {
                if v == number {
                    *f = Checked(number);
                    self.last_checked = number;
                }
            }
        }
    }

    fn score(self: &mut Self) -> Option<u32> {
        let row_bingo = self.rows().any(|mut row| row.bingo());
        let col_bingo = self.cols().any(|mut col| col.bingo());
        if row_bingo || col_bingo {
            let score: u32 = self.data.iter().filter_map(|f| {
                match f {
                    Field::Unchecked(v) => Some(v),
                    _ => None
                }
            }).sum::<u32>() * self.last_checked;
            return Some(score);
        }
        None
    }
}

pub fn winning_bingo_score() -> u32
{
    let numbers = parse_numbers(INPUT_NUMBERS);
    let mut boards = parse_boards(INPUT_BOARDS, (5, 5));
    for num in numbers {
        for board in boards.iter_mut() {
            board.check(num);
            if let Some(score) = board.score() {
                return score
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    const BOARD_SIZE : usize = 5;

    const EXAMPLE_NUMBERS : &[u32] = &[
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
    ];

    const EXAMPLE_BOARDS : &[[u32; BOARD_SIZE*BOARD_SIZE]] = &[
        [
            22, 13, 17, 11, 0,
            8,  2,  23, 4,  24,
            21, 9,  14, 16, 7,
            6,  10, 3,  18, 5,
            1,  12, 20, 15, 19,
        ],
        [
            3,  15, 0,  2,  22,
            9,  18, 13, 17, 5,
            19, 8,  7,  25, 23,
            20, 11, 10, 24, 4,
            14, 21, 16, 12, 6,
        ],
        [
            14, 21, 17, 24, 4,
            10, 16, 15, 9,  19,
            18, 8,  23, 26, 20,
            22, 11, 13, 6,  5,
            2,  0,  12, 3,  7,
        ],
    ];

    fn new_board() -> Board {
        Board::new(&EXAMPLE_BOARDS[0][..], (5, 5))
    }

    #[test]
    fn board_create(){
        let _ = new_board();
    }

    #[test]
    fn board_rows_count(){ 
        let board = new_board();
        assert_eq!(5, board.rows().count());
    }

    #[test]
    fn board_rows_content(){ 
        use Field::*;
        let board = new_board();
        let first_row : Vec<Field> = board.rows().next().unwrap().collect();
        assert_eq!(
            vec![
                Unchecked(22),
                Unchecked(13),
                Unchecked(17),
                Unchecked(11),
                Unchecked(0)
            ],
            first_row
        );
        let last_row : Vec<Field> = board.rows().skip(4).next().unwrap().collect();
        let last_value = last_row.into_iter().skip(4).next().unwrap();
        assert_eq!(Unchecked(19), last_value);
    }

    #[test]
    fn board_cols_count(){ 
        let board = new_board();
        assert_eq!(5, board.cols().count());
    }

    #[test]
    fn board_cols_value(){ 
        use Field::*;
        let board = new_board();
        let first_col : Vec<Field> = board.cols().next().unwrap().collect();
        assert_eq!(
            vec![
                Unchecked(22),
                Unchecked(8),
                Unchecked(21),
                Unchecked(6),
                Unchecked(1)
            ],
            first_col
        );
        let last_col : Vec<Field> = board.cols().skip(4).next().unwrap().collect();
        let last_value = last_col.into_iter().skip(4).next().unwrap();
        assert_eq!(Unchecked(19), last_value);
    }

    #[test]
    fn check_number() {
        let mut board = new_board();
        board.check(11);
        let mut fields = board.rows().next().unwrap().skip(3);
        assert_eq!(Field::Checked(11), fields.next().unwrap());
        assert_eq!(Field::Unchecked(0), fields.next().unwrap());
    }

    #[test]
    fn row_is_bingo() {
        use Field::*;
        let data = [Unchecked(1u32), Unchecked(2), Unchecked(3)];
        let mut row = Row{data: data[..].iter()};
        assert!(!row.bingo());
        let data = [Checked(1u32), Checked(2), Checked(3)];
        let mut row = Row{data: data[..].iter()};
        assert!(row.bingo());
    }

    #[test]
    fn bingo_row_example() {
        let mut board = Board::new(&EXAMPLE_BOARDS[2][..], (BOARD_SIZE, BOARD_SIZE));
        for n in EXAMPLE_NUMBERS {
            board.check(*n);
            if let Some(_) = board.score() { 
                break; 
            }
        }
        assert_eq!(Some(4512), board.score());
    }

    #[test]
    fn bingo_example() {
        let mut boards = Vec::<Board>::new();
        for board_data in EXAMPLE_BOARDS {
            boards.push(Board::new(board_data, (BOARD_SIZE, BOARD_SIZE)));
        }

        for num in EXAMPLE_NUMBERS {
            for board in boards.iter_mut() {
                board.check(*num);
                if let Some(score) = board.score() {
                    assert_eq!(4512, score);
                    return;
                }
            }
        }
        panic!("No board won");
    }

}
