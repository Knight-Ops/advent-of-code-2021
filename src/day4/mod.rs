use std::collections::VecDeque;

use fnv::FnvHashMap;

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Cordinate {
    x: usize,
    y: usize,
}

impl Cordinate {
    fn new(x: usize, y: usize) -> Self {
        Cordinate { x, y }
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct BingoSpace {
    value: usize,
    was_called: bool,
}

impl BingoSpace {
    fn new(value: usize) -> Self {
        BingoSpace {
            value,
            was_called: false,
        }
    }

    fn get_value(&self) -> usize {
        self.value
    }

    fn mark_space(&mut self) {
        self.was_called = true;
    }

    fn is_marked(&self) -> bool {
        self.was_called
    }
}

#[derive(Default, Debug, Clone)]
pub struct BingoBoard {
    board: FnvHashMap<Cordinate, BingoSpace>,
    numbers_marked: usize,
    board_size: usize,
    completed: bool,
}

impl BingoBoard {
    fn insert(&mut self, k: Cordinate, v: BingoSpace) -> Option<BingoSpace> {
        self.board.insert(k, v)
    }

    fn check_for_number(&mut self, value: usize) -> bool {
        let mut filtered_board = self
            .board
            .iter_mut()
            .filter(|(_, space)| space.get_value() == value);

        if let Some((_, space)) = filtered_board.next() {
            space.mark_space();
            self.numbers_marked += 1;
            true
        } else {
            false
        }
    }

    fn check_for_winner(&mut self) -> bool {
        if self.numbers_marked < self.board_size || self.completed {
            false
        } else {
            if self.check_rows() {
                self.completed = true;
                true
            } else if self.check_columns() {
                self.completed = true;
                true
            } else {
                false
            }
        }
    }

    fn check_rows(&self) -> bool {
        'row: for y in 0..self.board_size {
            // let mut row_winner = true;
            for x in 0..self.board_size {
                if !self
                    .board
                    .get(&Cordinate { x, y })
                    .expect("Could not retrieve BingoSpace for provided cordinates!")
                    .is_marked()
                {
                    // row_winner = false;
                    continue 'row;
                }
            }

            return true;
        }

        false
    }

    fn check_columns(&self) -> bool {
        'column: for x in 0..self.board_size {
            // let mut col_winner = true;
            for y in 0..self.board_size {
                if !self
                    .board
                    .get(&Cordinate { x, y })
                    .expect("Could not retrieve BingoSpace for provided cordinates!")
                    .is_marked()
                {
                    // col_winner = false;
                    continue 'column;
                }
            }

            return true;
        }

        false
    }

    fn set_board_size(&mut self, value: usize) {
        if value > self.board_size {
            self.board_size = value;
        }
    }

    fn board_size(&self) -> usize {
        self.board_size
    }
}

#[derive(Default, Debug, Clone)]
pub struct Bingo {
    called_numbers: VecDeque<usize>,
    boards: Vec<BingoBoard>,
    number_counter: usize,
}

impl Bingo {
    fn call_number(&mut self) -> Option<(Vec<BingoBoard>, usize)> {
        let num = self
            .called_numbers
            .pop_front()
            .expect("Error while getting number to call");
        self.number_counter += 1;
        for board in &mut self.boards {
            board.check_for_number(num);
        }

        let mut winning_boards = Vec::new();

        if self.number_counter >= self.boards[0].board_size() {
            for board in &mut self.boards {
                if board.check_for_winner() {
                    winning_boards.push(board.clone());
                }
            }
        }

        if winning_boards.len() > 0 {
            Some((winning_boards, num))
        } else {
            None
        }
    }

    fn play(&mut self) -> Option<(Vec<BingoBoard>, usize)> {
        while self.called_numbers.len() != 0 {
            if let Some(bingo_board) = self.call_number() {
                return Some(bingo_board);
            }
        }

        None
    }

    fn play_until_last(&mut self) -> Option<(BingoBoard, usize)> {
        let mut winning_boards = 0;
        let mut last_win = (Vec::new(), 0);
        while self.called_numbers.len() != 0 {
            if let Some(bingo_board) = self.call_number() {
                winning_boards += bingo_board.0.len();

                last_win = bingo_board;

                if winning_boards == self.boards.len() {
                    if last_win.0.len() != 1 {
                        panic!("Multiple final losing boards?");
                    } else {
                        return Some((last_win.0[0].clone(), last_win.1));
                    }
                }
            }
        }

        Some((last_win.0[0].clone(), last_win.1))
    }
}

pub fn input_generator(input: &str) -> Bingo {
    let called_numbers;

    let mut split_input = input.split("\n\n");

    // Parse out the called_numbers
    if let Some(num_string) = split_input.next() {
        called_numbers = num_string
            .split(",")
            .into_iter()
            .filter_map(|x| x.trim().parse::<usize>().ok())
            .collect();
    } else {
        panic!("Error while called_numbers from provided input");
    }

    // Move on to parsing our boards
    let mut boards = Vec::with_capacity(512);

    while let Some(board_data) = split_input.next() {
        let mut board = BingoBoard::default();
        let mut y_cord = 0;
        board_data.lines().into_iter().for_each(|line| {
            let mut x_cord = 0;
            line.split_whitespace().into_iter().for_each(|val| {
                let board_value = val
                    .parse::<usize>()
                    .expect("Error while parsing board value");

                board.insert(Cordinate::new(x_cord, y_cord), BingoSpace::new(board_value));

                x_cord += 1;
            });
            y_cord += 1;

            board.set_board_size(y_cord);
        });

        boards.push(board);
    }

    Bingo {
        called_numbers,
        boards,
        number_counter: 0,
    }
}

pub fn part1(input: &mut Bingo) -> usize {
    if let Some((winning_board, winning_number)) = input.play() {
        if winning_board.len() != 1 {
            panic!("We have more than one first winner");
        }

        // We have a winning board at this point so we need to sum unmarked numbers
        let board_value: usize = winning_board[0]
            .board
            .values()
            .into_iter()
            .map(|x| if !x.is_marked() { x.get_value() } else { 0 })
            .sum();

        // Multiply by winning number for our answer
        board_value * winning_number
    } else {
        panic!("No winning board was found");
    }
}

pub fn part2(input: &mut Bingo) -> usize {
    if let Some((winning_board, winning_number)) = input.play_until_last() {
        // We have a winning board at this point so we need to sum unmarked numbers
        let board_value: usize = winning_board
            .board
            .values()
            .into_iter()
            .map(|x| if !x.is_marked() { x.get_value() } else { 0 })
            .sum();

        // Multiply by winning number for our answer
        board_value * winning_number
    } else {
        panic!("No winning board was found");
    }
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

                let mut input = super::input_generator(&i);
                assert_eq!(super::$func(&mut input), $val);
            }
        };
    }

    test!(part1, 4512);
    test!(part2, 1924);
}
