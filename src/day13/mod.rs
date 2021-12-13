use std::{collections::VecDeque, fmt::Display};

use fnv::FnvHashSet;

#[derive(Debug, Clone)]
pub struct Origami {
    points: FnvHashSet<Coordinate>,
    folds: VecDeque<Fold>,
    largest_x: usize,
    largest_y: usize,
}

impl Origami {
    fn fold(&mut self) {
        let fold = self.folds.pop_front().expect("No more folds to execute!");

        match fold {
            Fold::X(loc) => {
                self.largest_x = loc;
                let mut second_points: Vec<Coordinate> =
                    self.points.drain_filter(|coord| coord.x > loc).collect();

                for point in &mut second_points {
                    self.points.insert(*point.x_translate(loc));
                }
            }
            Fold::Y(loc) => {
                self.largest_y = loc;
                let mut second_points: Vec<Coordinate> =
                    self.points.drain_filter(|coord| coord.y > loc).collect();

                for point in &mut second_points {
                    self.points.insert(*point.y_translate(loc));
                }
            }
        }
    }
}

impl Display for Origami {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.largest_y {
            for x in 0..self.largest_x {
                if self.points.get(&Coordinate { x, y }).is_some() {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn x_translate(&mut self, x: usize) -> &Self {
        self.x = (x as isize - (self.x as isize - x as isize)).abs() as usize;
        self
    }

    fn y_translate(&mut self, y: usize) -> &Self {
        self.y = (y as isize - (self.y as isize - y as isize)).abs() as usize;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Fold {
    X(usize),
    Y(usize),
}

pub fn input_generator(input: &str) -> Origami {
    let mut input = input.split("\n\n");

    let points: FnvHashSet<Coordinate> = input
        .next()
        .expect("Error getting populated points")
        .lines()
        .map(|line| {
            let mut coord_split = line.split(",");

            Coordinate {
                x: coord_split
                    .next()
                    .expect("Error getting x Coord out of iterator")
                    .parse::<usize>()
                    .expect("Error parsing x coord"),
                y: coord_split
                    .next()
                    .expect("Error getting y Coord out of iterator")
                    .parse::<usize>()
                    .expect("Error parsing y coord"),
            }
        })
        .collect();

    let folds: VecDeque<Fold> = input
        .next()
        .expect("Error getting fold locations")
        .lines()
        .map(|line| {
            let mut fold_line = line.trim_start_matches("fold along ").split("=");

            match fold_line
                .next()
                .expect("Error while getting fold direction from iterator")
            {
                "x" => Fold::X(
                    fold_line
                        .next()
                        .expect("Error getting fold location from iterator")
                        .parse::<usize>()
                        .expect("Error parsing fold location"),
                ),
                "y" => Fold::Y(
                    fold_line
                        .next()
                        .expect("Error getting fold location from iterator")
                        .parse::<usize>()
                        .expect("Error parsing fold location"),
                ),
                _ => unreachable!("Unable to fold on provided fold line"),
            }
        })
        .collect();

    Origami {
        points,
        folds,
        largest_x: 0,
        largest_y: 0,
    }
}

pub fn part1(input: &Origami) -> usize {
    let mut foldable_origami = input.clone();

    foldable_origami.fold();

    foldable_origami.points.len()
}

pub fn part2(input: &Origami) -> usize {
    let mut foldable_origami = input.clone();
    while foldable_origami.folds.len() != 0 {
        foldable_origami.fold();
    }

    print!("{}", foldable_origami);
    0
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

    test!(part1, 17);
    test!(part2, 0);
}
