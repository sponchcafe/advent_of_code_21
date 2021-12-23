use std::str::FromStr;

const INPUT: &str = include_str!("../data/10/input");

#[derive(Debug, Clone, PartialEq)]
enum Scope { OPEN, CLOSE}

#[derive(Debug, Clone, PartialEq)]
enum Class {
    PAREN,
    CURLY,
    SQUARE,
    ANGLE,
}

#[derive(Debug, Clone, PartialEq)]
struct Token ( Class, Scope, );

#[derive(Debug, Clone, PartialEq)]
struct SyntaxError(Class);

impl Token {
    fn from_char(c: char) -> Result<Self, &'static str> {
        use Class::*;
        use Scope::*;
        match c {
            '(' => Ok(Token(PAREN, OPEN)),
            ')' => Ok(Token(PAREN, CLOSE)),
            '{' => Ok(Token(CURLY, OPEN)),
            '}' => Ok(Token(CURLY, CLOSE)),
            '[' => Ok(Token(SQUARE, OPEN)),
            ']' => Ok(Token(SQUARE, CLOSE)),
            '<' => Ok(Token(ANGLE, OPEN)),
            '>' => Ok(Token(ANGLE, CLOSE)),
            _ => Err("Unknown token"),
        }
    }
}

struct NavLine( Vec::<Token> );


impl FromStr for NavLine {
    type Err = &'static str;
    fn from_str(s: &str) -> Result::<Self, Self::Err> {
        let tokens = s.chars().map(Token::from_char).collect::<Vec::<Result::<Token, &'static str>>>();
        if tokens.iter().all(|t| t.is_ok()) {
            let tokens = tokens.into_iter().map(|t| t.unwrap()).collect::<Vec::<Token>>();
            Ok(Self(tokens))
        }
        else {
            Err("Invalid token")
        }
    }
}

impl NavLine {

    fn errors(self: &Self) -> Vec::<SyntaxError> {
        use Scope::*;

        let mut stack = TokenStack::new();
        let mut errors = Vec::<SyntaxError>::new();

        for token in self.0.iter() {
            match (stack.peek(), token) {
                (Some(Token(class_open, OPEN)), Token(class_close, CLOSE)) => {
                    if class_open == *class_close {
                        stack.pop(); // Matching scope
                    }
                    else {
                        errors.push(SyntaxError(class_close.clone())); // Mismatch
                    }
                }
                _ => { stack.push(token.clone()); }
            }
        }
        errors
    }
}

struct TokenStack ( Vec<Token> );
impl TokenStack {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn push(self: &mut Self, t: Token) {
        self.0.push(t);
    }
    fn pop(self: &mut Self) {
        self.0.pop();
    }
    fn peek(self: &Self) -> Option<Token> {
        self.0.last().map(Token::to_owned)
    }
}

fn score_error(error: &SyntaxError) -> u64 {
    use Class::*;
    match error.0 {
        PAREN => {3}
        CURLY => {1197}
        SQUARE => {57}
        ANGLE => {25137}
    }
}

fn parse_input(s: &str) -> Vec::<NavLine> {
    s.trim().split('\n')
        .map(str::trim)
        .map(NavLine::from_str)
        .map(|l| l.expect("Invalid input"))
        .collect()
}

fn calculate_error_score(data: &Vec::<NavLine>) -> u64 {
    data
        .iter()
        .filter_map(|navline| navline.errors().get(0).map(SyntaxError::to_owned))
        .map(|e| score_error(&e))
        .sum()
}

pub fn syntax_error_score() -> u64 {
    calculate_error_score(&parse_input(INPUT))
}

#[cfg(test)]
mod test {
    use super::*;
    use Class::*;
    use Scope::*;

    const EXAMPLE: &str = "
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    ";

    #[test]
    fn parse_token() {
        assert_eq!(Token(PAREN, OPEN), Token::from_char('(').unwrap());
        assert_eq!(Token(PAREN, CLOSE), Token::from_char(')').unwrap());
        assert_eq!(Token(CURLY, OPEN), Token::from_char('{').unwrap());
        assert_eq!(Token(CURLY, CLOSE), Token::from_char('}').unwrap());
        assert_eq!(Token(SQUARE, OPEN), Token::from_char('[').unwrap());
        assert_eq!(Token(SQUARE, CLOSE), Token::from_char(']').unwrap());
        assert_eq!(Token(ANGLE, OPEN), Token::from_char('<').unwrap());
        assert_eq!(Token(ANGLE, CLOSE), Token::from_char('>').unwrap());

        for c in " abcdx%^".chars() {
            assert!(Token::from_char(c).is_err());
        }
    }

    #[test]
    fn parse_nav_lne() {
        let line: NavLine = "".parse().unwrap();
        assert_eq!(0, line.0.len());

        let line: NavLine = "((([][}}([]]><>".parse().unwrap();
        assert_eq!(15, line.0.len());

        let line = "))*[]}".parse::<NavLine>();
        assert!(line.is_err());
    }

    #[test]
    fn parse_example_input() {
        let navline = parse_input(EXAMPLE);
        assert_eq!(10, navline.len());
        assert_eq!(Token(ANGLE, OPEN), navline[4].0[2]);
    }

    #[test]
    fn syntax_error() {
        let navline: NavLine = "()".parse().unwrap();
        assert!(navline.errors().is_empty());

        let navline: NavLine = "(]".parse().unwrap();
        assert!(!navline.errors().is_empty());

        let navline: NavLine = "(()]".parse().unwrap();
        assert!(!navline.errors().is_empty());

        let navline: NavLine = "<([]){()}[{}])".parse().unwrap();
        assert!(!navline.errors().is_empty());
    }

    #[test]
    fn get_errors() {
        let navline: NavLine = "()".parse().unwrap();
        let errors: Vec::<SyntaxError> = Vec::new();
        assert_eq!(errors, navline.errors());

        let navline: NavLine = "(]".parse().unwrap();
        assert_eq!(vec![SyntaxError(SQUARE)], navline.errors());

        let navline: NavLine = "<{([([[(<>()){}]>(<<{{".parse().unwrap();
        assert_eq!(vec![SyntaxError(ANGLE)], navline.errors());
    }

    #[test]
    fn scoring() {
        let errors = vec![
            SyntaxError(PAREN),  // 3
            SyntaxError(SQUARE), // 57
            SyntaxError(CURLY),  // 1197
            SyntaxError(ANGLE),  // 25137
            SyntaxError(PAREN),  // 3
        ];                       // 26397

        assert_eq!(26397u64, errors.iter().map(score_error).sum());
    }

    #[test]
    fn example_score() {
        let input = parse_input(EXAMPLE);
        assert_eq!(26397u64, calculate_error_score(&input));
    }


    #[test]
    fn solution_score() {
        let input = parse_input(INPUT);
        assert_eq!(316851u64, calculate_error_score(&input));
    }

}
