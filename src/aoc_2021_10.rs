use std::iter::Peekable;

// use rayon::prelude::*;

#[cfg(test)]
mod tests;

pub fn determine_total_syntax_error_score<'a>(code: impl IntoIterator<Item = &'a str>) -> u64 {
    code.into_iter()
        .map(to_token_tree)
        .map(to_syntax_error_value)
        .sum()
}

fn to_token_tree(line: &str) -> TokenTree {
    TokenTree::from(line)
}

fn to_syntax_error_value(tree: TokenTree) -> u64 {
    tree.syntax_error_value()
}

struct TokenTree {
    opening: Token,
    closing: Option<Token>,
    children: Vec<TokenTree>,
}

impl TokenTree {
    fn syntax_error_value(&self) -> u64 {
        for child in self.children.iter() {
            let sev = child.syntax_error_value();
            if sev > 0 {
                return sev;
            }
        }
        use Token::*;
        match (self.opening, self.closing) {
            (OpenParen, Some(CloseSquare))
            | (OpenCurly, Some(CloseSquare))
            | (OpenAngle, Some(CloseSquare)) => 57,
            (OpenParen, Some(CloseCurly))
            | (OpenSquare, Some(CloseCurly))
            | (OpenAngle, Some(CloseCurly)) => 1197,
            (OpenParen, Some(CloseAngle))
            | (OpenSquare, Some(CloseAngle))
            | (OpenCurly, Some(CloseAngle)) => 25137,
            (OpenSquare, Some(CloseParen))
            | (OpenCurly, Some(CloseParen))
            | (OpenAngle, Some(CloseParen)) => 3,
            (CloseParen, Some(_)) => 0,
            (CloseSquare, Some(_)) => 0,
            (CloseCurly, Some(_)) => 0,
            (CloseAngle, Some(_)) => 0,
            (OpenParen, None) => 0,
            (OpenSquare, None) => 0,
            (OpenCurly, None) => 0,
            (OpenAngle, None) => 0,
            (CloseParen, None) => 0,
            (CloseSquare, None) => 0,
            (CloseCurly, None) => 0,
            (CloseAngle, None) => 0,
            _ => 0,
        }
    }
}

impl From<&str> for TokenTree {
    fn from(line: &str) -> Self {
        let mut line = line.chars().peekable();
        token_tree_from(&mut line)
    }
}

fn token_tree_from(line: &mut Peekable<impl Iterator<Item = char>>) -> TokenTree {
    let first_token = Token::from(line.next().unwrap());
    let mut tree = TokenTree {
        opening: first_token,
        closing: None,
        children: Vec::new(),
    };
    while line.peek().is_some() && Token::from(*line.peek().unwrap()).is_open_token() {
        tree.children.push(token_tree_from(line));
    }
    if line.peek().is_some() {
        tree.closing = Some(Token::from(line.next().unwrap()))
    }
    tree
}

#[derive(Copy, Clone)]
enum Token {
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    OpenAngle,
    CloseAngle,
}

impl Token {
    // fn is_close_token(&self) -> bool {
    //     use Token::*;
    //     matches!(self, CloseParen | CloseSquare | CloseCurly | CloseAngle)
    // }

    fn is_open_token(&self) -> bool {
        use Token::*;
        matches!(self, OpenParen | OpenSquare | OpenCurly | OpenAngle)
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        use Token::*;
        match c {
            '(' => OpenParen,
            ')' => CloseParen,
            '[' => OpenSquare,
            ']' => CloseSquare,
            '{' => OpenCurly,
            '}' => CloseCurly,
            '<' => OpenAngle,
            '>' => CloseAngle,
            _ => panic!("Invalid Token"),
        }
    }
}
