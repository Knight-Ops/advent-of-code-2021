use fnv::FnvHashMap;
use fnv::FnvHashSet;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Cave {
    vertices: usize,
    edges: FnvHashMap<usize, FnvHashMap<usize, isize>>,
    visited: Vec<usize>,
}

impl Cave {
    fn new(vertices: usize) -> Self {
        Cave {
            vertices,
            edges: FnvHashMap::default(),
            visited: Vec::new(),
        }
    }

    fn add_edge(&mut self, start: usize, end: usize, weight: isize) {
        self.edges.entry(end).or_default().insert(start, weight);
    }

    fn dijkstra(&self, start_vertex: usize, end_vertex: usize) -> Vec<usize> {
        let mut dist = vec![usize::MAX; self.vertices];
        dist[start_vertex] = 0;

        let mut bh = BinaryHeap::new();
        bh.push((0, start_vertex));

        while !bh.is_empty() {
            let (_, current_vertex) = bh.pop().expect("The binary heap should not be empty!");
            for (&neighbor, &distance) in self
                .edges
                .get(&current_vertex)
                .expect("Error getting current vertex neighbors hashmap")
                .iter()
            {
                // let distance = *self.edges.get(&current_vertex).expect("Error getting current vertex neighbors hashmap").get(&neighbor).expect("Error getting distance");
                if !self.visited.contains(&neighbor) {
                    let old_cost = dist[neighbor];
                    let new_cost = dist[current_vertex] + distance as usize;
                    if new_cost < old_cost {
                        bh.push((new_cost as isize, neighbor));
                        dist[neighbor] = new_cost;
                    }
                }

                if self.visited.contains(&end_vertex) {
                    return dist;
                }
            }
        }

        dist
    }
}

pub fn input_generator(input: &str) -> Cave {
    let row_size = input.lines().next().expect("There no line at all").len();
    let lines = input.lines().count();

    let mut cave = Cave::new(row_size * lines);

    input.lines().enumerate().for_each(|(y_idx, line)| {
        line.chars().enumerate().for_each(|(x_idx, c)| {
            let curr_idx = (y_idx * row_size) + x_idx;
            if y_idx != 0 {
                cave.add_edge(
                    curr_idx,
                    curr_idx - row_size,
                    c.to_digit(10)
                        .expect("Could not convert character to digit")
                        as isize,
                )
            }
            if x_idx != row_size - 1 {
                cave.add_edge(
                    curr_idx,
                    curr_idx + 1,
                    c.to_digit(10)
                        .expect("Could not convert character to digit")
                        as isize,
                )
            }
            if y_idx != lines - 1 {
                cave.add_edge(
                    curr_idx,
                    curr_idx + row_size,
                    c.to_digit(10)
                        .expect("Could not convert character to digit")
                        as isize,
                )
            }
            if x_idx != 0 {
                cave.add_edge(
                    curr_idx,
                    curr_idx - 1,
                    c.to_digit(10)
                        .expect("Could not convert character to digit")
                        as isize,
                )
            }
        })
    });

    cave
}

pub fn input_generator_tiled(input: &str) -> Cave {
    for y in 0..5 {
        for x in 0..5 {}
    }

    let row_size = input.lines().next().expect("There no line at all").len() * 5;
    let lines = input.lines().count() * 5;

    let mut cave = Cave::new(row_size * lines);

    input.lines().enumerate().for_each(|(y_idx, line)| {
        line.chars().enumerate().for_each(|(x_idx, c)| {
            let curr_idx = (y_idx * row_size) + x_idx;
            if y_idx != 0 {
                cave.add_edge(
                    curr_idx,
                    curr_idx - row_size,
                    c.to_digit(10)
                        .expect("Could not convert character to digit")
                        as isize,
                )
            }
            if x_idx != row_size - 1 {
                cave.add_edge(
                    curr_idx,
                    curr_idx + 1,
                    c.to_digit(10)
                        .expect("Could not convert character to digit")
                        as isize,
                )
            }
            if y_idx != lines - 1 {
                cave.add_edge(
                    curr_idx,
                    curr_idx + row_size,
                    c.to_digit(10)
                        .expect("Could not convert character to digit")
                        as isize,
                )
            }
            if x_idx != 0 {
                cave.add_edge(
                    curr_idx,
                    curr_idx - 1,
                    c.to_digit(10)
                        .expect("Could not convert character to digit")
                        as isize,
                )
            }
        })
    });

    cave
}

pub fn part1(input: &Cave) -> usize {
    let distance_vector = input.dijkstra(0, input.vertices - 1);

    distance_vector[distance_vector.len() - 1]
}

pub fn part2(input: &Cave) -> usize {
    // let huge_cave = input.enlarge();

    let distance_vector = input.dijkstra(0, input.vertices - 1);

    distance_vector[distance_vector.len() - 1]
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

    test!(part1, 40);
    // test!(part2, 0);

    // #[test]
    // fn part2_test() {
    //     let name = module_path!().split("::").collect::<Vec<&str>>();
    //     let i = read_input_file(&format!(
    //         "input/2021/{}_test.txt",
    //         name[name.len() - 2].trim()
    //     ));

    //     let input = super::input_generator_tiled(&i);
    //     assert_eq!(super::part2(&input), 315);
    // }
}
