use fnv::FnvHashMap;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
pub enum VerticalDirection {
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub enum HorizontalDirection {
    Left,
    Right
}

#[derive(Debug, Clone)]
pub enum DiagonalType {
    UpRight,
    DownRight,
    DownLeft,
    UpLeft,
}

#[derive(Debug, Clone)]
pub enum Slope {
    Vertical(VerticalDirection),
    Horizontal(HorizontalDirection),
    Diagonal(DiagonalType),
}

#[derive(Debug, Clone)]
pub struct Line {
    start: Point,
    end: Point,
    slope: Option<Slope>,
}

impl Line {
    fn find_slope(&mut self) {

        let y_slope = self.end.y - self.start.y;

        if y_slope == 0 {
            if self.end.x > self.start.x {
                self.slope = Some(Slope::Horizontal(HorizontalDirection::Right));
                return;
            } else {
                self.slope = Some(Slope::Horizontal(HorizontalDirection::Left));
                return;
            }
        }

        let x_slope = self.end.x - self.start.x;

        if x_slope == 0 {
            if self.end.y > self.start.y {
                self.slope = Some(Slope::Vertical(VerticalDirection::Up));
                return;
            } else {
                self.slope = Some(Slope::Vertical(VerticalDirection::Down));
                return;
            }
        }

        let dir = if self.end.y > self.start.y && self.end.x > self.start.x {
            DiagonalType::UpRight
        } else if self.start.y > self.end.y && self.end.x > self.start.x {
            DiagonalType::DownRight
        } else if self.end.y > self.start.y && self.start.x > self.end.x {
            DiagonalType::UpLeft
        } else if self.start.y > self.end.y && self.start.x > self.end.x {
            DiagonalType::DownLeft
        } else {        
            unreachable!("Somehow the line doesn't actually have slope?")
        };

        self.slope = Some(Slope::Diagonal(dir));
    }
}

pub fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .into_iter()
        .map(|line_data| {
            let mut coordinate_pairs = line_data.split(" -> ").into_iter();
            let mut start_pair = coordinate_pairs
                .next()
                .expect("coordinate_pairs doesn't have elements for start_point")
                .split(",")
                .into_iter();
            let start_point = Point {
                x: start_pair
                    .next()
                    .expect("Iterator doesn't have elements for start_point.x")
                    .parse()
                    .expect("Error while parsing start_point.x"),
                y: start_pair
                    .next()
                    .expect("Iterator doesn't have elements for start_point.y")
                    .parse()
                    .expect("Error while parsing start_point.y"),
            };

            let mut end_pair = coordinate_pairs
                .next()
                .expect("coordinate_pairs doesn't have elements for end_point")
                .split(",")
                .into_iter();
            let end_point = Point {
                x: end_pair
                    .next()
                    .expect("Iterator doesn't have elements for end_point.x")
                    .parse()
                    .expect("Error while parsing end_point.x"),
                y: end_pair
                    .next()
                    .expect("Iterator doesn't have elements for end_point.y")
                    .parse()
                    .expect("Error while parsing end_point.y"),
            };

            let mut line = Line {
                start: start_point,
                end: end_point,
                slope: None,
            };
            
            line.find_slope();

            line
        })
        .collect()
}

pub fn part1(input: &[Line]) -> usize {
    let mut board: FnvHashMap<Point, usize> = FnvHashMap::default();

    for line in input {
        match &line.slope {
            Some(Slope::Diagonal(_)) => continue,
            Some(Slope::Horizontal(dir)) => {
                let y = line.start.y;

                match dir {
                    &HorizontalDirection::Left => {
                        for x in line.end.x..=line.start.x {
                            *board.entry(Point{x, y}).or_insert(0) += 1;
                        }
                    }
                    &HorizontalDirection::Right => {
                        for x in line.start.x..=line.end.x {
                            *board.entry(Point{x, y}).or_insert(0) += 1;
                        }
                    }
                }

            },
            Some(Slope::Vertical(dir)) => {
                let x = line.start.x;

                match dir {
                    &VerticalDirection::Up => {
                        for y in line.start.y..=line.end.y {
                            *board.entry(Point{x, y}).or_insert(0) += 1;
                        }
                    }
                    &VerticalDirection::Down => {
                        for y in line.end.y..=line.start.y {
                            *board.entry(Point{x, y}).or_insert(0) += 1;
                        }
                    }
                }

            },
            None => panic!("Slope was never solved for?")
        }
    }

    board.values().into_iter().filter(|&&x| x > 1).count()
}

pub fn part2(input: &[Line]) -> usize {
    let mut board: FnvHashMap<Point, usize> = FnvHashMap::default();

    for line in input {
        let mut x = line.start.x;
        let mut y = line.start.y;
        match &line.slope {
            Some(Slope::Diagonal(ty)) => {
                match ty {
                    &DiagonalType::UpRight => {
                        while x != line.end.x && y != line.end.y {
                            *board.entry(Point{x, y}).or_insert(0) += 1;
                            x += 1;
                            y += 1;
                        }

                        *board.entry(Point{x, y}).or_insert(0) += 1;
                    },
                    &DiagonalType::DownRight => {
                        while x != line.end.x && y != line.end.y {
                            *board.entry(Point{x, y}).or_insert(0) += 1;
                            x += 1;
                            y -= 1;
                        }

                        *board.entry(Point{x, y}).or_insert(0) += 1;
                    },
                    &DiagonalType::UpLeft => {                      
                        while x != line.end.x && y != line.end.y {
                            *board.entry(Point{x, y}).or_insert(0) += 1;
                            x -= 1;
                            y += 1;
                        }

                        *board.entry(Point{x, y}).or_insert(0) += 1;
                    },
                    &DiagonalType::DownLeft => {
                        while x != line.end.x && y != line.end.y {
                            *board.entry(Point{x, y}).or_insert(0) += 1;
                            x -= 1;
                            y -= 1;
                        }

                        *board.entry(Point{x, y}).or_insert(0) += 1;
                    },
                }


            },
            Some(Slope::Horizontal(dir)) => {
                // let y = line.start.y;

                match dir {
                    &HorizontalDirection::Left => {
                        for x in line.end.x..=line.start.x {
                            *board.entry(Point{x, y}).or_insert(0) += 1;
                        }
                    }
                    &HorizontalDirection::Right => {
                        for x in line.start.x..=line.end.x {
                            *board.entry(Point{x, y}).or_insert(0) += 1;
                        }
                    }
                }

            },
            Some(Slope::Vertical(dir)) => {
                // let x = line.start.x;

                match dir {
                    &VerticalDirection::Up => {
                        for y in line.start.y..=line.end.y {
                            *board.entry(Point{x, y}).or_insert(0) += 1;
                        }
                    }
                    &VerticalDirection::Down => {
                        for y in line.end.y..=line.start.y {
                            *board.entry(Point{x, y}).or_insert(0) += 1;
                        }
                    }
                }

            },
            None => panic!("Slope was never solved for?")
        }
    }

    board.values().into_iter().filter(|&&x| x > 1).count()
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

    test!(part1, 5);
    test!(part2, 12);
}
