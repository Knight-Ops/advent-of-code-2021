use fnv::FnvHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate {
            x, y
        }
    }

    fn get_neighbors(&self, max_x: usize, max_y: usize) -> [Option<Coordinate>; 8] {
        let mut neighbors = [
            Some(Coordinate::new(self.x, self.y+1)),
            Some(Coordinate::new(self.x+1, self.y+1)),
            Some(Coordinate::new(self.x+1, self.y)),
            Some(Coordinate::new(self.x+1, (self.y as isize-1) as usize)),
            Some(Coordinate::new(self.x, (self.y as isize-1) as usize)),
            Some(Coordinate::new((self.x as isize-1) as usize, (self.y as isize-1) as usize)),
            Some(Coordinate::new((self.x as isize-1) as usize, self.y)),
            Some(Coordinate::new((self.x as isize-1) as usize, self.y+1)),
        ];

        if self.x == 0 {
            neighbors[5] = None;
            neighbors[6] = None;
            neighbors[7] = None;
        } else if self.x == max_x {
            neighbors[1] = None;
            neighbors[2] = None;
            neighbors[3] = None;
        }

        if self.y == 0 {
            neighbors[3] = None;
            neighbors[4] = None;
            neighbors[5] = None;
        } else if self.y == max_y {
            neighbors[7] = None;
            neighbors[0] = None;
            neighbors[1] = None;
        }

        neighbors
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Octopus {
    energy: usize,
    flashed: bool,
}

pub struct OctopusSwarm {
    swarm: FnvHashMap<Coordinate, Octopus>,
    max_x: usize,
    max_y: usize,
}

impl OctopusSwarm {
    fn step(&mut self) -> usize {
        // Inc each octo energy level
        for (_, octo) in self.swarm.iter_mut() {
            octo.energy += 1;
        }


        loop {
            if self.swarm.iter().filter(|(_, octo)| octo.energy > 9 && !octo.flashed).count() == 0 {
                break
            }

            let mut neighbors = Vec::new();
            for (coord, octo) in self.swarm.iter_mut().filter(|(_, octo)| octo.energy > 9 && !octo.flashed) {
                neighbors.extend_from_slice(&coord.get_neighbors(self.max_x, self.max_y));
                octo.flashed = true;
            }

            for neighbor in neighbors {
                if let Some(valid_neighbor) = neighbor {
                    if let Some(charged_octo) = self.swarm.get_mut(&valid_neighbor) {
                        charged_octo.energy += 1;
                    }
                }
            }
                
        }

        let flashed = self.swarm.iter().filter(|(_, octo)| octo.flashed).count();

        for (_, octo) in self.swarm.iter_mut() {
            if octo.flashed {
                octo.energy = 0;
                octo.flashed = false;
            }
        }

        flashed
    }
}

pub fn input_generator(input: &str) -> (FnvHashMap<Coordinate, Octopus>, usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut map = FnvHashMap::default();
    input
        .lines()
        .enumerate()
        .into_iter()
        .for_each(|(y_idx, line)| {
            line.chars().enumerate().for_each(|(x_idx, c)| {
                let energy = c.to_digit(10).expect("Error converting char to energy");

                max_x = x_idx;
                map.insert(Coordinate{x: x_idx, y: y_idx}, Octopus {energy: energy as usize, flashed: false});
            });
            max_y = y_idx;
        });

    (map, max_x, max_y)
}

pub fn part1(input: &(FnvHashMap<Coordinate, Octopus>, usize, usize)) -> usize {
    let mut swarm = OctopusSwarm {swarm: input.0.to_owned(), max_x: input.1, max_y: input.2};

    let mut flashes = 0;

    for _ in 0..100 {
        flashes += swarm.step()
    }

    flashes
}

pub fn part2(input: &(FnvHashMap<Coordinate, Octopus>, usize, usize)) -> usize {
    let mut swarm = OctopusSwarm {swarm: input.0.to_owned(), max_x: input.1, max_y: input.2};

    let mut flashes = 0;
    let mut steps = 0;

    while flashes != ((input.1 + 1)) * (input.2 + 1) {
        flashes = swarm.step();
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    macro_rules! test {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!("input/2021/{}_test.txt", name[name.len() - 2].trim()));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(&input), $val);
            }
        }
    }

    test!(part1, 1656);
    test!(part2, 195);
}