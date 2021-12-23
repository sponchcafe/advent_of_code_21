use std::str::FromStr;

const INPUT: &str = include_str!("../data/10/input");

#[derive(Debug, Clone, PartialEq)]
enum Scope { OPEN, CLOSE}

#[derive(Debug, Clone, PartialEq)]
enum Class {
    PAREN,
    SQUARE,
    CURLY,
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

    fn analyze(self: &Self) -> Result::<TokenStack, SyntaxError> {
        use Scope::*;

        let mut stack = TokenStack::new();

        for token in self.0.iter() {
            match (stack.peek(), token) {
                (Some(Token(class_open, OPEN)), Token(class_close, CLOSE)) => {
                    if class_open == *class_close {
                        stack.pop(); // Matching scope
                    }
                    else {
                        return Err(SyntaxError(class_close.clone())); // Mismatch
                    }
                }
                _ => { stack.push(token.clone()); }
            }
        }
        Ok(stack.rev())
    }

    fn error(self: &Self) -> Option::<SyntaxError> {
        self.analyze().err()
    }

    fn complete(self: &Self) -> Option<Vec::<Class>> {
        match self.analyze() {
            Ok(stack) if !stack.0.is_empty() => { 
                Some(stack.0.iter().map(|t| t.0.clone()).collect())
            },
            _ => { None }
        }
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
    fn rev(self: Self) -> Self {
        Self(self.0.into_iter().rev().collect())
    }
}

fn score_error(error: &SyntaxError) -> u64 {
    use Class::*;
    match error.0 {
        PAREN => {3}
        SQUARE => {57}
        CURLY => {1197}
        ANGLE => {25137}
    }
}

fn score_completion(completion: &Class) -> u64 {
    use Class::*;
    match completion {
        PAREN => {1}
        SQUARE => {2}
        CURLY => {3}
        ANGLE => {4}
    }
}

fn score_completions(completion: &Vec::<Class>) -> u64 {
    completion.iter().fold(0u64, |acc, comp| acc*5 + score_completion(comp))
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
        .filter_map(NavLine::error)
        .map(|e| score_error(&e))
        .sum()
}

fn calculate_middle_complete_score(data: &Vec::<NavLine>) -> u64 {
    let mut completion_scores: Vec<u64> = data
        .iter()
        .filter_map(NavLine::complete)
        .map(|e| score_completions(&e))
        .collect();
    completion_scores.sort();
    completion_scores[completion_scores.len()/2]
}

pub fn syntax_error_score() -> u64 {
    calculate_error_score(&parse_input(INPUT))
}

pub fn completion_score() -> u64 {
    calculate_middle_complete_score(&parse_input(INPUT))
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
        assert!("()".parse::<NavLine>().unwrap().error().is_none());
        assert_eq!(Some(SyntaxError(SQUARE)), "(]".parse::<NavLine>().unwrap().error());
        assert_eq!(Some(SyntaxError(SQUARE)), "(()]".parse::<NavLine>().unwrap().error());
        assert_eq!(Some(SyntaxError(PAREN)), "<([]){()}[{}])".parse::<NavLine>().unwrap().error());
        assert_eq!(Some(SyntaxError(ANGLE)), "<{([([[(<>()){}]>(<<{{".parse::<NavLine>().unwrap().error());
    }

    #[test]
    fn error_scoring() {
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
    fn example_error_score() {
        let input = parse_input(EXAMPLE);
        assert_eq!(26397u64, calculate_error_score(&input));
    }

    #[test]
    fn solution_error_score() {
        let input = parse_input(INPUT);
        assert_eq!(316851u64, calculate_error_score(&input));
    }

    #[test]
    fn complete_line() {
        assert!("".parse::<NavLine>().unwrap().complete().is_none());
        assert_eq!(Some(vec![PAREN]), "(".parse::<NavLine>().unwrap().complete());
        assert_eq!(Some(vec![CURLY, PAREN, PAREN]), "(<>[](<{}>{".parse::<NavLine>().unwrap().complete());
    }

    #[test]
    fn completion_scoring() { 

        fn from_str(s: &str) -> Vec::<Class> {
            s.parse::<NavLine>().unwrap().0.into_iter().map(|t| t.0).collect()
        }

        assert_eq!(288957u64,   score_completions(&from_str("}}]])})]")));
        assert_eq!(5566,        score_completions(&from_str(")}>]})")));
        assert_eq!(1480781,     score_completions(&from_str("}}>}>))))")));
        assert_eq!(995444,      score_completions(&from_str("]]}}]}]}>")));
        assert_eq!(294,         score_completions(&from_str("])}>")));
    }

    #[test]
    fn example_complete_score() {
        let input = parse_input(EXAMPLE);
        assert_eq!(288957u64, calculate_middle_complete_score(&input));
    }
    
}
