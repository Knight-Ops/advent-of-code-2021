#[derive(Debug, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    OpenFish,
    CloseFish,
    Invalid,
}

pub fn input_generator(input: &str) -> String {
    input.to_string()
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut token_stream = Vec::with_capacity(line.len() * std::mem::size_of::<Token>());
            line.chars()
                .map(|c| match c {
                    '(' => {
                        token_stream.push(Token::OpenParen);
                        0
                    }
                    '[' => {
                        token_stream.push(Token::OpenSquare);
                        0
                    }
                    '{' => {
                        token_stream.push(Token::OpenCurly);
                        0
                    }
                    '<' => {
                        token_stream.push(Token::OpenFish);
                        0
                    }
                    ')' => {
                        if let Some(tok) = token_stream.pop() {
                            if tok == Token::OpenParen {
                                0
                            } else {
                                3
                            }
                        } else {
                            panic!("More closing tokens than opening tokens, is this corrupted?")
                        }
                    }
                    ']' => {
                        if let Some(tok) = token_stream.pop() {
                            if tok == Token::OpenSquare {
                                0
                            } else {
                                57
                            }
                        } else {
                            panic!("More closing tokens than opening tokens, is this corrupted?")
                        }
                    }
                    '}' => {
                        if let Some(tok) = token_stream.pop() {
                            if tok == Token::OpenCurly {
                                0
                            } else {
                                1197
                            }
                        } else {
                            panic!("More closing tokens than opening tokens, is this corrupted?")
                        }
                    }
                    '>' => {
                        if let Some(tok) = token_stream.pop() {
                            if tok == Token::OpenFish {
                                0
                            } else {
                                25137
                            }
                        } else {
                            panic!("More closing tokens than opening tokens, is this corrupted?")
                        }
                    }
                    _ => {
                        token_stream.push(Token::Invalid);
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

pub fn part2(input: &str) -> usize {
    let mut scores = Vec::new();

    'lines: for line in input.lines() {
        let mut token_stream = Vec::with_capacity(line.len() * std::mem::size_of::<Token>());

        for c in line.chars() {
            match c {
                '(' => {
                    token_stream.push(Token::OpenParen);
                }
                '[' => {
                    token_stream.push(Token::OpenSquare);
                }
                '{' => {
                    token_stream.push(Token::OpenCurly);
                }
                '<' => {
                    token_stream.push(Token::OpenFish);
                }
                ')' => {
                    if let Some(tok) = token_stream.pop() {
                        if tok == Token::OpenParen {
                        } else {
                            continue 'lines;
                        }
                    } else {
                        panic!("More closing tokens than opening tokens, is this corrupted?")
                    }
                }
                ']' => {
                    if let Some(tok) = token_stream.pop() {
                        if tok == Token::OpenSquare {
                        } else {
                            continue 'lines;
                        }
                    } else {
                        panic!("More closing tokens than opening tokens, is this corrupted?")
                    }
                }
                '}' => {
                    if let Some(tok) = token_stream.pop() {
                        if tok == Token::OpenCurly {
                        } else {
                            continue 'lines;
                        }
                    } else {
                        panic!("More closing tokens than opening tokens, is this corrupted?")
                    }
                }
                '>' => {
                    if let Some(tok) = token_stream.pop() {
                        if tok == Token::OpenFish {
                        } else {
                            continue 'lines;
                        }
                    } else {
                        panic!("More closing tokens than opening tokens, is this corrupted?")
                    }
                }
                _ => {
                    token_stream.push(Token::Invalid);
                }
            }
        }

        // Score remaining line
        let mut score = 0;

        for tok in token_stream.iter().rev() {
            match tok {
                Token::OpenParen => {
                    score = score * 5 + 1;
                }
                Token::OpenSquare => {
                    score = score * 5 + 2;
                }
                Token::OpenCurly => {
                    score = score * 5 + 3;
                }
                Token::OpenFish => {
                    score = score * 5 + 4;
                }
                _ => unreachable!("Remaining token is not an Open* Token"),
            }
        }

        scores.push(score);
    }
    scores.sort();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    macro_rules! test {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!(
                    "input/2021/{}_test.txt",
                    name[name.len() - 2].trim()
                ));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(&input), $val);
            }
        };
    }

    test!(part1, 26397);
    test!(part2, 288957);
}
