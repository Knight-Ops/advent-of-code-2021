use std::collections::VecDeque;

use fnv::{FnvHashMap, FnvHashSet};

pub struct CaveFloor {
    map: FnvHashMap<Coordinate, u32>,
    max_x: usize,
    max_y: usize,
}

impl<'a> CaveFloor {
    fn find_lowest_points(&self) -> usize {
        self.map
            .keys()
            .filter(|coord| self.is_lowest_neighbor(coord))
            .map(|coord| {
                let val = self
                    .map
                    .get(coord)
                    .expect("Error while retrieving coord from map");
                *val as usize + 1
            })
            .sum()
    }

    fn find_basins(&self) -> usize {
        let mut basin_sizes = self
            .map
            .keys()
            .filter(|coord| self.is_lowest_neighbor(coord))
            .map(|coord| {
                let mut explored = FnvHashSet::default();
                self.explore_basin(coord, &mut explored);
                explored.len()
            })
            .collect::<VecDeque<usize>>();

        basin_sizes.make_contiguous().sort();

        basin_sizes
            .pop_back()
            .expect("Basin sizes isn't 1 entry long")
            * basin_sizes
                .pop_back()
                .expect("Basin sizes isn't 2 entries long")
            * basin_sizes
                .pop_back()
                .expect("Basin sizes isn't 3 entries long")
    }

    fn explore_basin(&self, coord: &Coordinate, explored: &mut FnvHashSet<Coordinate>) {
        if explored.insert(coord.to_owned()) == false {
            return;
        };

        let all_neighbors = coord.get_neighbors(self.max_x, self.max_y);
        all_neighbors
            .iter()
            .filter(|entry| {
                entry.is_some()
                    && self
                        .map
                        .get(&entry.expect("Known good neighbor is not good"))
                        .expect("Error retrieving value of provided coordinates")
                        != &9
            })
            .for_each(|basin_dir| {
                self.explore_basin(
                    &basin_dir.expect("Known good neighbor is not actually good"),
                    explored,
                )
            })
    }

    fn is_lowest_neighbor(&self, coord: &Coordinate) -> bool {
        let current_height = self.map.get(coord).expect("Error getting current height!");

        if current_height == &9 {
            // This is the highest a point can be, no point in checking
            return false;
        }

        if current_height == &0 {
            // This is the lowest a point can be, so we can assume its a low point
            return true;
        }

        let all_neighbors = coord.get_neighbors(self.max_x, self.max_y);
        let valid_neighbors = all_neighbors.iter().filter(|entry| entry.is_some());

        for neighbor in valid_neighbors {
            if current_height
                >= self
                    .map
                    .get(&neighbor.expect("Neighbor in valid_neighbors isn't actually valid"))
                    .expect("Coordinate from valid_neighbors doesn't exist in map")
            {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }

    fn get_neighbors(&self, max_x: usize, max_y: usize) -> [Option<Coordinate>; 4] {
        // In our neighbors array, we are going to populate starting from Up and going clockwise
        let mut neighbors = [None; 4];

        // Default is None, so we can just ignore it if we are on the edge here
        if self.y != 0 {
            neighbors[0] = Some(Self::new(self.x, self.y - 1));
        }

        if self.x != max_x {
            neighbors[1] = Some(Self::new(self.x + 1, self.y))
        }

        if self.y != max_y {
            neighbors[2] = Some(Self::new(self.x, self.y + 1))
        }

        if self.x != 0 {
            neighbors[3] = Some(Self::new(self.x - 1, self.y))
        }

        neighbors
    }
}

pub fn input_generator(input: &str) -> CaveFloor {
    let mut map = FnvHashMap::default();
    let mut max_x = 0;
    let mut max_y = 0;
    input
        .lines()
        .into_iter()
        .enumerate()
        .for_each(|(row, col)| {
            let mut coord;
            let mut value;

            for (idx, c) in col.chars().enumerate() {
                coord = Coordinate { x: idx, y: row };

                if idx > max_x {
                    max_x = idx;
                }
                if row > max_y {
                    max_y = row;
                }

                value = c.to_digit(10).expect("Error converting char to digit");

                map.insert(coord, value);
            }
        });

    CaveFloor { map, max_x, max_y }
}

#[inline(never)]
pub fn part1(input: &CaveFloor) -> usize {
    input.find_lowest_points()
}

#[inline(never)]
pub fn part2(input: &CaveFloor) -> usize {
    input.find_basins()
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

    test!(part1, 15);
    test!(part2, 1134);
}
